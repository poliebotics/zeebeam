//! Host for the whole-session chain relation: execute (with an oracle built without the circuit),
//! prove (Groth16, cpu or cuda, with tamper controls and an exact artifact), verify (cold, optionally
//! against a given ELF), export (raw proof + public values), vkey.
//!
//! usage:
//!   zeebeam-chain execute [--expect PATH]
//!   zeebeam-chain prove --out DIR [--prover cpu|cuda] [--expect PATH]
//!   zeebeam-chain verify --proof PATH [--elf PATH] [--expect PATH]
//!   zeebeam-chain export --proof PATH --out DIR
//!   zeebeam-chain vkey
//! Environment: CHAIN_CSV (default the session's chain_log.csv), SESSION_ID, MANIFEST_SHA256_HEX, S_N_HEX;
//! --expect is a JSON of expected public fields produced by the Python oracle (chain_expect.py).

use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::time::Instant;

use bincode::Options;
use sha2::{Digest, Sha256};
use sp1_sdk::{
    blocking::{ProveRequest, Prover, ProverClient},
    include_elf, Elf, HashableKey, ProvingKey, SP1Proof, SP1ProofWithPublicValues, SP1PublicValues, SP1Stdin,
    SP1_CIRCUIT_VERSION,
};

const CHAIN_ELF: Elf = include_elf!("zeebeam-chain-program");
const DEFAULT_CSV: &str = "/data/zeebeam_evidence_20260822/stage/live_300s_training_001/chain_log.csv";
const DEFAULT_SESSION_ID: &str = "ZEEBEAM_MAINNET_BLOCKING_TRAINING_300S_20260822_001";
const DEFAULT_MANIFEST: &str = "740d752d27b70cb63c9501a470562a52616f4a445a7a9c0fb300844f61c7d783";
const DEFAULT_S_N: &str = "aeea9f4d6a55ebecd900eae187ea70dce05696551f96ae976c9a5243b3a4398e";
const DEFAULT_EXPECT: &str = "/home/c/Documents/BOSUN/scratch/joined_build_20260901/chain_expect.json";
const CYCLE_LIMIT: u64 = 2_000_000_000;
const MAX_PROOF_BYTES: u64 = 64 * 1024 * 1024;
const TAMPER_PUBLIC_BYTE: usize = 87;
const TAMPER_GROTH16_PROOF_BYTE: usize = 96;

fn usage() -> ! {
    eprintln!("usage: zeebeam-chain execute|prove|verify|export|vkey [--out DIR] [--prover cpu|cuda] [--proof PATH] [--elf PATH] [--expect PATH]");
    std::process::exit(2);
}
fn arg(args: &[String], flag: &str) -> Option<String> {
    args.iter().position(|a| a == flag).and_then(|i| args.get(i + 1).cloned())
}
fn env_or(k: &str, d: &str) -> String { std::env::var(k).unwrap_or_else(|_| d.to_string()) }
fn hex(b: &[u8]) -> String { b.iter().map(|x| format!("{x:02x}")).collect() }
fn sha256_hex(b: &[u8]) -> String { format!("{:x}", Sha256::digest(b)) }
fn unhex(s: &str) -> Vec<u8> { (0..s.len() / 2).map(|i| u8::from_str_radix(&s[2 * i..2 * i + 2], 16).expect("hex")).collect() }

fn build_stdin() -> (SP1Stdin, Vec<u8>) {
    let csv = fs::read(env_or("CHAIN_CSV", DEFAULT_CSV)).expect("chain log");
    let session_id = env_or("SESSION_ID", DEFAULT_SESSION_ID).into_bytes();
    let manifest = unhex(&env_or("MANIFEST_SHA256_HEX", DEFAULT_MANIFEST));
    let s_n = unhex(&env_or("S_N_HEX", DEFAULT_S_N));
    let mut stdin = SP1Stdin::new();
    stdin.write_vec(csv.clone());
    stdin.write_vec(session_id);
    stdin.write_vec(manifest);
    stdin.write_vec(s_n);
    (stdin, csv)
}

/// Decode the ZBCHAIN1 statement into named fields.
fn decode(pv: &[u8]) -> serde_json::Value {
    assert!(pv.len() >= 8 + 4 + 2 + 4 + 8 + 8 + 32 * 6 + 2, "statement too short");
    assert_eq!(&pv[0..8], b"ZBCHAIN1", "bad magic");
    let u32be = |o: usize| u32::from_be_bytes(pv[o..o + 4].try_into().unwrap());
    let u16be = |o: usize| u16::from_be_bytes(pv[o..o + 2].try_into().unwrap());
    let u64be = |o: usize| u64::from_be_bytes(pv[o..o + 8].try_into().unwrap());
    let sid_len = u16be(226) as usize;
    serde_json::json!({
        "magic": "ZBCHAIN1", "total_bytes": pv.len(), "sha256": sha256_hex(pv),
        "row_count": u32be(8), "tree_depth": u16be(12), "distinct_rounds": u32be(14),
        "first_round": u64be(18), "last_round": u64be(26),
        "s_0": hex(&pv[34..66]), "s_n": hex(&pv[66..98]), "chain_log_blake3": hex(&pv[98..130]),
        "authority_manifest_sha256": hex(&pv[130..162]), "context_digest_blake3": hex(&pv[162..194]),
        "ordered_session_root_blake3": hex(&pv[194..226]),
        "session_id": String::from_utf8_lossy(&pv[228..228 + sid_len]).to_string(),
    })
}

/// Compare every decoded field with the Python oracle's expectations; every byte of the statement is
/// covered by these fields, so a full match means no byte rests on the circuit alone.
fn check(pv: &[u8], expect_path: &str) -> Result<(usize, usize), String> {
    let d = decode(pv);
    let e: serde_json::Value = serde_json::from_str(&fs::read_to_string(expect_path).map_err(|x| format!("expect file: {x}"))?)
        .map_err(|x| format!("expect json: {x}"))?;
    let mut checked = 0usize;
    let mut mismatches = Vec::new();
    for (k, bytes) in [("row_count", 4), ("tree_depth", 2), ("distinct_rounds", 4), ("first_round", 8), ("last_round", 8),
                       ("s_0", 32), ("s_n", 32), ("chain_log_blake3", 32), ("authority_manifest_sha256", 32),
                       ("context_digest_blake3", 32), ("ordered_session_root_blake3", 32), ("session_id", 0)] {
        let got = &d[k]; let want = &e[k];
        if want.is_null() { mismatches.push(format!("{k}: no expectation")); continue; }
        if got != want { mismatches.push(format!("{k}: circuit {got} oracle {want}")); }
        checked += if k == "session_id" { 2 + d["session_id"].as_str().unwrap().len() } else { bytes };
    }
    checked += 8; // magic, fixed
    if !mismatches.is_empty() { return Err(mismatches.join("; ")); }
    Ok((checked, pv.len()))
}

fn save_exact(proof: &SP1ProofWithPublicValues, output: &Path) -> Result<(u64, String), String> {
    let parent = output.parent().ok_or("output has no parent")?;
    fs::create_dir_all(parent).map_err(|e| format!("cannot create {}: {e}", parent.display()))?;
    let pending = output.with_extension("pending");
    let _ = fs::remove_file(&pending);
    let file = OpenOptions::new().write(true).create_new(true).mode(0o600).open(&pending).map_err(|e| format!("cannot create {}: {e}", pending.display()))?;
    let mut writer = BufWriter::new(file);
    bincode::DefaultOptions::new().with_fixint_encoding().reject_trailing_bytes().serialize_into(&mut writer, proof).map_err(|e| format!("cannot serialize proof: {e}"))?;
    writer.flush().map_err(|e| format!("cannot flush proof: {e}"))?;
    writer.get_ref().sync_all().map_err(|e| format!("cannot fsync proof: {e}"))?;
    drop(writer);
    let bytes = fs::read(&pending).map_err(|e| format!("cannot re-read proof: {e}"))?;
    if bytes.len() as u64 > MAX_PROOF_BYTES { return Err("proof artifact exceeds the size limit".into()); }
    fs::rename(&pending, output).map_err(|e| format!("cannot commit proof: {e}"))?;
    File::open(parent).and_then(|d| d.sync_all()).map_err(|e| format!("cannot fsync directory: {e}"))?;
    Ok((bytes.len() as u64, sha256_hex(&bytes)))
}

fn load_exact(path: &Path) -> Result<(SP1ProofWithPublicValues, u64, String), String> {
    let bytes = fs::read(path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
    if bytes.len() as u64 > MAX_PROOF_BYTES { return Err("proof artifact exceeds the size limit".into()); }
    let digest = sha256_hex(&bytes);
    let mut reader = BufReader::new(&bytes[..]);
    let proof: SP1ProofWithPublicValues = bincode::DefaultOptions::new().with_fixint_encoding().with_limit(bytes.len() as u64).reject_trailing_bytes()
        .deserialize_from(&mut reader).map_err(|e| format!("proof framing is not exact: {e}"))?;
    let mut trailing = [0u8; 1];
    if reader.read(&mut trailing).map_err(|e| format!("cannot test framing: {e}"))? != 0 { return Err("proof framing has trailing bytes".into()); }
    Ok((proof, bytes.len() as u64, digest))
}

fn mutate_groth16_proof_byte(proof: &mut SP1ProofWithPublicValues) -> Result<(), String> {
    let SP1Proof::Groth16(groth16) = &mut proof.proof else { return Err("proof is not Groth16".into()); };
    let mut encoded = groth16.encoded_proof.as_bytes().to_vec();
    if encoded.len() <= TAMPER_GROTH16_PROOF_BYTE { return Err("encoded proof too short to tamper".into()); }
    let c = encoded[TAMPER_GROTH16_PROOF_BYTE];
    encoded[TAMPER_GROTH16_PROOF_BYTE] = if c == b'0' { b'1' } else { b'0' };
    groth16.encoded_proof = String::from_utf8(encoded).map_err(|_| "tampered proof is not UTF-8")?;
    Ok(())
}

fn verify_original_and_tamper<P: Prover>(client: &P, proof: &SP1ProofWithPublicValues, vkey: &sp1_sdk::SP1VerifyingKey) -> Result<(), String> {
    client.verify(proof, vkey, None).map_err(|e| format!("proof does not verify: {e}"))?;
    let mut public_tampered = proof.clone();
    let mut bytes = public_tampered.public_values.to_vec();
    bytes[TAMPER_PUBLIC_BYTE] ^= 1;
    public_tampered.public_values = SP1PublicValues::from(&bytes);
    if client.verify(&public_tampered, vkey, None).is_ok() { return Err("a tampered public byte was accepted".into()); }
    let mut proof_tampered = proof.clone();
    mutate_groth16_proof_byte(&mut proof_tampered)?;
    if client.verify(&proof_tampered, vkey, None).is_ok() { return Err("a tampered proof byte was accepted".into()); }
    let mut wrong_vkey = vkey.clone();
    wrong_vkey.vk.preprocessed_commit.rotate_left(1);
    if wrong_vkey.bytes32() == vkey.bytes32() { return Err("wrong-verifying-key mutation did not change its hash".into()); }
    if client.verify(proof, &wrong_vkey, None).is_ok() { return Err("a wrong verification key was accepted".into()); }
    Ok(())
}

fn run_execute<P: Prover>(client: &P, expect: &str) -> Result<(), String> {
    let (stdin, csv) = build_stdin();
    println!("mode=execute_only chain_csv_bytes={} chain_csv_sha256={}", csv.len(), sha256_hex(&csv));
    let started = Instant::now();
    let (output, report) = client.execute(CHAIN_ELF, stdin).run().map_err(|e| format!("execute failed: {e}"))?;
    let pv = output.as_slice();
    println!("public_values_bytes={}", pv.len());
    println!("public_values_hex={}", hex(pv));
    println!("decoded={}", serde_json::to_string(&decode(pv)).unwrap());
    match check(pv, expect) {
        Ok((checked, total)) => println!("oracle=chain_python checked={checked} of {total} circuit_derived={}", total - checked),
        Err(e) => { println!("MISMATCH: {e}"); return Err("public values differ from the oracle".into()); }
    }
    println!("total_instruction_count={}", report.total_instruction_count());
    println!("total_syscall_count={}", report.total_syscall_count());
    for (name, cycles) in report.cycle_tracker.iter() { println!("CYCLE_REGION {name} = {cycles}"); }
    println!("host_elapsed_ms={}", started.elapsed().as_millis());
    println!("verified_public_values=true");
    Ok(())
}

fn run_prove<P: Prover>(client: &P, prover_name: &str, out_dir: &Path, expect: &str) -> Result<(), String> {
    let (stdin, csv) = build_stdin();
    let elf_sha256 = sha256_hex(&CHAIN_ELF);
    let setup_started = Instant::now();
    let pk = client.setup(CHAIN_ELF).map_err(|e| format!("SP1 setup failed: {e}"))?;
    let vkey = pk.verifying_key().bytes32();
    let setup_ms = setup_started.elapsed().as_millis();
    println!("mode=ceremony_groth16 prover={prover_name} sp1_circuit_version={SP1_CIRCUIT_VERSION}");
    println!("guest_elf_bytes={} guest_elf_sha256={elf_sha256}", CHAIN_ELF.len());
    println!("sp1_vkey={vkey}");
    println!("setup_elapsed_ms={setup_ms}");
    let prove_started = Instant::now();
    let proof = client.prove(&pk, stdin).groth16().cycle_limit(CYCLE_LIMIT).with_proof_nonce([0, 0, 0, 0]).run().map_err(|e| format!("SP1 Groth16 proof failed: {e}"))?;
    let prove_ms = prove_started.elapsed().as_millis();
    if !matches!(&proof.proof, SP1Proof::Groth16(_)) { return Err("SP1 returned a non-Groth16 proof".into()); }
    if proof.sp1_version != SP1_CIRCUIT_VERSION { return Err("SP1 returned a proof under a different circuit version".into()); }
    let public = proof.public_values.to_vec();
    let (checked, total) = check(&public, expect)?;
    let verify_started = Instant::now();
    verify_original_and_tamper(client, &proof, pk.verifying_key())?;
    let verify_ms = verify_started.elapsed().as_millis();
    let out = out_dir.join("chain_groth16.bin");
    let (proof_bytes, proof_sha256) = save_exact(&proof, &out)?;
    fs::write(out_dir.join("chain_public_values.hex"), format!("{}\n", hex(&public))).map_err(|e| format!("cannot write public values: {e}"))?;
    let manifest = serde_json::json!({
        "schema": "zeebeam-chain-ceremony/v1", "prover": prover_name, "sp1_version": proof.sp1_version,
        "chain_csv_bytes": csv.len(), "chain_csv_sha256": sha256_hex(&csv),
        "guest_elf_bytes": CHAIN_ELF.len(), "guest_elf_sha256": elf_sha256, "sp1_vkey": vkey,
        "public_values_bytes": public.len(), "public_values_sha256": sha256_hex(&public), "public_values_hex": hex(&public),
        "decoded": decode(&public), "oracle_mode": "chain_python", "oracle_bytes_checked": checked, "oracle_bytes_circuit_derived": total - checked,
        "proof_file": "chain_groth16.bin", "proof_bytes": proof_bytes, "proof_sha256": proof_sha256,
        "cycle_limit": CYCLE_LIMIT, "setup_elapsed_ms": setup_ms, "prove_elapsed_ms": prove_ms, "verify_and_tamper_elapsed_ms": verify_ms,
        "tamper_controls": ["public byte 87 flipped: REJECT", "groth16 proof nibble 96 flipped: REJECT", "wrong verification key: REJECT"],
    });
    fs::write(out_dir.join("chain_manifest.json"), serde_json::to_string_pretty(&manifest).unwrap()).map_err(|e| format!("cannot write manifest: {e}"))?;
    println!("oracle=chain_python checked={checked} of {total} circuit_derived={}", total - checked);
    println!("public_values_bytes={} public_values_sha256={}", public.len(), sha256_hex(&public));
    println!("prove_elapsed_ms={prove_ms}");
    println!("verify_and_tamper_elapsed_ms={verify_ms}");
    println!("proof_output={}", out.display());
    println!("proof_bytes={proof_bytes} proof_sha256={proof_sha256}");
    println!("verified_proof=true");
    Ok(())
}

fn run_verify<P: Prover>(client: &P, path: &Path, elf_path: Option<&Path>, expect: &str) -> Result<(), String> {
    let elf: Elf = match elf_path { Some(p) => Elf::from(fs::read(p).map_err(|e| format!("cannot read ELF {}: {e}", p.display()))?), None => CHAIN_ELF };
    let (proof, bytes, digest) = load_exact(path)?;
    if !matches!(&proof.proof, SP1Proof::Groth16(_)) { return Err("loaded proof is not Groth16".into()); }
    if proof.sp1_version != SP1_CIRCUIT_VERSION { return Err("loaded proof uses a different circuit version".into()); }
    let elf_sha256 = sha256_hex(&elf);
    let pk = client.setup(elf).map_err(|e| format!("SP1 setup failed: {e}"))?;
    let vkey = pk.verifying_key().bytes32();
    verify_original_and_tamper(client, &proof, pk.verifying_key())?;
    let public = proof.public_values.to_vec();
    let (checked, total) = check(&public, expect)?;
    println!("mode=cold_verify_groth16 sp1_vkey={vkey} guest_elf_sha256={elf_sha256}");
    println!("proof_bytes={bytes} proof_sha256={digest}");
    println!("oracle=chain_python checked={checked} of {total} circuit_derived={}", total - checked);
    println!("decoded={}", serde_json::to_string(&decode(&public)).unwrap());
    println!("verified_proof=true");
    Ok(())
}

fn run_export(path: &Path, out_dir: &Path) -> Result<(), String> {
    let (proof, _, _) = load_exact(path)?;
    fs::create_dir_all(out_dir).map_err(|e| format!("cannot create {}: {e}", out_dir.display()))?;
    let raw = proof.bytes();
    let pv = proof.public_values.to_vec();
    fs::write(out_dir.join("chain_groth16_proof.bin"), &raw).map_err(|e| format!("cannot write raw proof: {e}"))?;
    fs::write(out_dir.join("chain_groth16_public_values.bin"), &pv).map_err(|e| format!("cannot write public values: {e}"))?;
    println!("raw_proof_bytes={} raw_proof_sha256={}", raw.len(), sha256_hex(&raw));
    println!("public_values_bytes={} public_values_sha256={}", pv.len(), sha256_hex(&pv));
    Ok(())
}

fn main() {
    sp1_sdk::utils::setup_logger();
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(command) = args.first() else { usage() };
    let prover = arg(&args, "--prover").unwrap_or_else(|| "cpu".into());
    let expect = arg(&args, "--expect").unwrap_or_else(|| DEFAULT_EXPECT.into());
    let result = match command.as_str() {
        "execute" => { let c = ProverClient::builder().cpu().build(); run_execute(&c, &expect) }
        "prove" => {
            let out = PathBuf::from(arg(&args, "--out").unwrap_or_else(|| usage()));
            match prover.as_str() {
                "cpu" => { let c = ProverClient::builder().cpu().build(); run_prove(&c, "local_cpu_explicit", &out, &expect) }
                #[cfg(feature = "cuda")]
                "cuda" => { let c = ProverClient::builder().cuda().build(); run_prove(&c, "local_cuda_explicit", &out, &expect) }
                other => Err(format!("unknown or unavailable prover {other}")),
            }
        }
        "verify" => {
            let proof = PathBuf::from(arg(&args, "--proof").unwrap_or_else(|| usage()));
            let elf = arg(&args, "--elf").map(PathBuf::from);
            let c = ProverClient::builder().cpu().build();
            run_verify(&c, &proof, elf.as_deref(), &expect)
        }
        "export" => {
            let proof = PathBuf::from(arg(&args, "--proof").unwrap_or_else(|| usage()));
            let out = PathBuf::from(arg(&args, "--out").unwrap_or_else(|| usage()));
            run_export(&proof, &out)
        }
        "vkey" => {
            let c = ProverClient::builder().cpu().build();
            match c.setup(CHAIN_ELF) {
                Ok(pk) => { println!("guest_elf_bytes={} guest_elf_sha256={}", CHAIN_ELF.len(), sha256_hex(&CHAIN_ELF)); println!("sp1_vkey={}", pk.verifying_key().bytes32()); Ok(()) }
                Err(e) => Err(format!("setup failed: {e}")),
            }
        }
        _ => usage(),
    };
    if let Err(e) = result { eprintln!("CHAIN_FAILED: {e}"); std::process::exit(1); }
    println!("CHAIN_OK");
}
