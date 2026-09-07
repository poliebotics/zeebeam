//! The proving ceremony, one pass per row: core STARK + recursion + Groth16 wrap inside SP1's
//! `.groth16()` request, verification of the result, three tamper controls, the oracle check on
//! the public values, and an exact on-disk artifact with a manifest. `verify` cold-verifies a saved
//! artifact on another machine.
//!
//! usage:
//!   ceremony prove  --raw PATH --out DIR [--prover cpu|cuda] [--cycle-limit N]
//!   ceremony verify --proof PATH --raw-sha256 HEX
//! Row selection and every witness path are the environment contract in witness.rs (ROW_JSON,
//! TRAPDOOR_LE_HEX, EXPECT_TRAPDOOR_FLAG, ...).

mod witness;

use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::time::Instant;

use bincode::Options;
use sha2::{Digest, Sha256};
use sp1_sdk::{
    blocking::{ProveRequest, Prover, ProverClient},
    include_elf, Elf, HashableKey, ProvingKey, SP1Proof, SP1ProofWithPublicValues, SP1PublicValues,
    SP1Stdin, SP1_CIRCUIT_VERSION,
};
use zeebeam_row_binding_membership::CLASSIFICATION;
use zeebeam_row_binding_membership_native::TOTAL_PUBLIC_BYTES;

const ZEEBEAM_ELF: Elf = include_elf!("zeebeam-row-binding-membership-program");
const DEFAULT_CYCLE_LIMIT: u64 = 4_400_000_000;
const MAX_PROOF_BYTES: u64 = 64 * 1024 * 1024;
const TAMPER_PUBLIC_BYTE: usize = 87;
const TAMPER_GROTH16_PROOF_BYTE: usize = 96;

fn usage() -> ! {
    eprintln!(
        "usage:\n  ceremony prove  --raw PATH --out DIR [--prover cpu|cuda] [--cycle-limit N]\n  ceremony verify --proof PATH [--elf PATH] [--raw-sha256 HEX]\n  ceremony export --proof PATH --out DIR\n  ceremony vkey [--elf PATH]"
    );
    std::process::exit(2);
}

fn arg(args: &[String], flag: &str) -> Option<String> {
    args.iter().position(|a| a == flag).and_then(|i| args.get(i + 1).cloned())
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn peak_rss_kib() -> Option<u64> {
    let status = fs::read_to_string("/proc/self/status").ok()?;
    status
        .lines()
        .find(|l| l.starts_with("VmHWM:"))
        .and_then(|l| l.split_whitespace().nth(1))
        .and_then(|v| v.parse().ok())
}

fn save_exact(proof: &SP1ProofWithPublicValues, output: &Path) -> Result<(u64, String), String> {
    let parent = output.parent().ok_or("output has no parent")?;
    fs::create_dir_all(parent).map_err(|e| format!("cannot create {}: {e}", parent.display()))?;
    let pending = output.with_extension("pending");
    let _ = fs::remove_file(&pending);
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(&pending)
        .map_err(|e| format!("cannot create {}: {e}", pending.display()))?;
    let mut writer = BufWriter::new(file);
    bincode::DefaultOptions::new()
        .with_fixint_encoding()
        .reject_trailing_bytes()
        .serialize_into(&mut writer, proof)
        .map_err(|e| format!("cannot serialize proof: {e}"))?;
    writer.flush().map_err(|e| format!("cannot flush proof: {e}"))?;
    writer.get_ref().sync_all().map_err(|e| format!("cannot fsync proof: {e}"))?;
    drop(writer);
    let bytes = fs::read(&pending).map_err(|e| format!("cannot re-read proof: {e}"))?;
    if bytes.len() as u64 > MAX_PROOF_BYTES {
        return Err("proof artifact exceeds the size limit".into());
    }
    fs::rename(&pending, output).map_err(|e| format!("cannot commit proof: {e}"))?;
    File::open(parent).and_then(|d| d.sync_all()).map_err(|e| format!("cannot fsync directory: {e}"))?;
    Ok((bytes.len() as u64, sha256_hex(&bytes)))
}

fn load_exact(path: &Path) -> Result<(SP1ProofWithPublicValues, u64, String), String> {
    let bytes = fs::read(path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
    if bytes.len() as u64 > MAX_PROOF_BYTES {
        return Err("proof artifact exceeds the size limit".into());
    }
    let digest = sha256_hex(&bytes);
    let mut reader = BufReader::new(&bytes[..]);
    let proof: SP1ProofWithPublicValues = bincode::DefaultOptions::new()
        .with_fixint_encoding()
        .with_limit(bytes.len() as u64)
        .reject_trailing_bytes()
        .deserialize_from(&mut reader)
        .map_err(|e| format!("proof framing is not exact: {e}"))?;
    let mut trailing = [0u8; 1];
    if reader.read(&mut trailing).map_err(|e| format!("cannot test framing: {e}"))? != 0 {
        return Err("proof framing has trailing bytes".into());
    }
    Ok((proof, bytes.len() as u64, digest))
}

fn mutate_groth16_proof_byte(proof: &mut SP1ProofWithPublicValues) -> Result<(), String> {
    let SP1Proof::Groth16(groth16) = &mut proof.proof else {
        return Err("proof is not Groth16".into());
    };
    let mut encoded = groth16.encoded_proof.as_bytes().to_vec();
    if encoded.len() <= TAMPER_GROTH16_PROOF_BYTE {
        return Err("encoded proof too short to tamper".into());
    }
    // flip one hex nibble in place, keeping the string valid hex
    let c = encoded[TAMPER_GROTH16_PROOF_BYTE];
    encoded[TAMPER_GROTH16_PROOF_BYTE] = if c == b'0' { b'1' } else { b'0' };
    groth16.encoded_proof = String::from_utf8(encoded).map_err(|_| "tampered proof is not UTF-8")?;
    Ok(())
}

/// Verify the proof, then require that a flipped public byte, a flipped proof byte and a wrong
/// verification key are all rejected.
fn verify_original_and_tamper<P: Prover>(
    client: &P,
    proof: &SP1ProofWithPublicValues,
    vkey: &sp1_sdk::SP1VerifyingKey,
) -> Result<(), String> {
    client
        .verify(proof, vkey, None)
        .map_err(|e| format!("proof does not verify: {e}"))?;
    let mut public_tampered = proof.clone();
    let mut bytes = public_tampered.public_values.to_vec();
    bytes[TAMPER_PUBLIC_BYTE] ^= 1;
    public_tampered.public_values = SP1PublicValues::from(&bytes);
    if client.verify(&public_tampered, vkey, None).is_ok() {
        return Err("a tampered public byte was accepted".into());
    }
    let mut proof_tampered = proof.clone();
    mutate_groth16_proof_byte(&mut proof_tampered)?;
    if client.verify(&proof_tampered, vkey, None).is_ok() {
        return Err("a tampered proof byte was accepted".into());
    }
    let mut wrong_vkey = vkey.clone();
    wrong_vkey.vk.preprocessed_commit.rotate_left(1); // deterministic corruption, as in groth16.rs
    if wrong_vkey.bytes32() == vkey.bytes32() {
        return Err("wrong-verifying-key mutation did not change its hash".into());
    }
    if client.verify(proof, &wrong_vkey, None).is_ok() {
        return Err("a wrong verification key was accepted".into());
    }
    Ok(())
}

fn run_prove<P: Prover>(client: &P, prover_name: &str, raw_path: &Path, out_dir: &Path, cycle_limit: u64) -> Result<(), String> {
    let raw = fs::read(raw_path).map_err(|e| format!("cannot read raw frame: {e}"))?;
    let raw_sha256 = sha256_hex(&raw);
    let raw_bytes = raw.len();
    let built = witness::build(raw);
    let elf_sha256 = sha256_hex(&ZEEBEAM_ELF);

    let setup_started = Instant::now();
    let proving_key = client.setup(ZEEBEAM_ELF).map_err(|e| format!("SP1 setup failed: {e}"))?;
    let vkey = proving_key.verifying_key().bytes32();
    let setup_ms = setup_started.elapsed().as_millis();

    println!("classification={CLASSIFICATION}");
    println!("mode=ceremony_groth16");
    println!("prover={prover_name}");
    println!("row={}", built.row);
    println!("trapdoor_flag_expected={}", built.trapdoor_flag);
    println!("sp1_circuit_version={SP1_CIRCUIT_VERSION}");
    println!("cycle_limit={cycle_limit}");
    println!("raw_bytes={raw_bytes} raw_sha256={raw_sha256}");
    println!("guest_elf_bytes={} guest_elf_sha256={elf_sha256}", ZEEBEAM_ELF.len());
    println!("sp1_vkey={vkey}");
    println!("setup_elapsed_ms={setup_ms}");

    let prove_started = Instant::now();
    let proof = client
        .prove(&proving_key, built.stdin.clone())
        .groth16()
        .cycle_limit(cycle_limit)
        .with_proof_nonce([0, 0, 0, 0])
        .run()
        .map_err(|e| format!("SP1 Groth16 proof failed: {e}"))?;
    let prove_ms = prove_started.elapsed().as_millis();
    if !matches!(&proof.proof, SP1Proof::Groth16(_)) {
        return Err("SP1 returned a non-Groth16 proof".into());
    }
    if proof.sp1_version != SP1_CIRCUIT_VERSION {
        return Err("SP1 returned a proof under a different circuit version".into());
    }

    let public = proof.public_values.to_vec();
    let report = witness::check(&public, &built)?;
    let verify_started = Instant::now();
    verify_original_and_tamper(client, &proof, proving_key.verifying_key())?;
    let verify_ms = verify_started.elapsed().as_millis();

    let out = out_dir.join(format!("row_{:03}_groth16.bin", built.row));
    let (proof_bytes, proof_sha256) = save_exact(&proof, &out)?;
    let public_hex = witness::hex(&public);
    fs::write(out_dir.join(format!("row_{:03}_public_values.hex", built.row)), format!("{public_hex}\n"))
        .map_err(|e| format!("cannot write public values: {e}"))?;
    let manifest = serde_json::json!({
        "schema": "zeebeam-one-proof-ceremony/v1",
        "row": built.row,
        "prover": prover_name,
        "sp1_version": proof.sp1_version,
        "guest_elf_bytes": ZEEBEAM_ELF.len(),
        "guest_elf_sha256": elf_sha256,
        "sp1_vkey": vkey,
        "raw_frame_sha256": raw_sha256,
        "public_values_bytes": public.len(),
        "public_values_sha256": sha256_hex(&public),
        "public_values_hex": public_hex,
        "oracle_mode": report.mode,
        "oracle_bytes_checked": report.bytes_checked,
        "oracle_bytes_circuit_derived": report.bytes_circuit_derived,
        "oracle_notes": report.notes,
        "trapdoor_flag": public[public.len() - 1],
        "proof_file": out.file_name().unwrap().to_string_lossy(),
        "proof_bytes": proof_bytes,
        "proof_sha256": proof_sha256,
        "cycle_limit": cycle_limit,
        "setup_elapsed_ms": setup_ms,
        "prove_elapsed_ms": prove_ms,
        "verify_and_tamper_elapsed_ms": verify_ms,
        "host_peak_rss_kib": peak_rss_kib(),
        "tamper_controls": ["public byte 87 flipped: REJECT", "groth16 proof nibble 96 flipped: REJECT", "wrong verification key: REJECT"],
    });
    fs::write(out_dir.join(format!("row_{:03}_manifest.json", built.row)), serde_json::to_string_pretty(&manifest).unwrap())
        .map_err(|e| format!("cannot write manifest: {e}"))?;

    println!("oracle={} checked={} circuit_derived={}", report.mode, report.bytes_checked, report.bytes_circuit_derived);
    for n in &report.notes {
        println!("oracle_note={n}");
    }
    println!("public_values_bytes={}", public.len());
    println!("public_values_sha256={}", sha256_hex(&public));
    println!("prove_elapsed_ms={prove_ms}");
    println!("verify_and_tamper_elapsed_ms={verify_ms}");
    println!("host_peak_rss_kib={:?}", peak_rss_kib());
    println!("proof_output={}", out.display());
    println!("proof_bytes={proof_bytes} proof_sha256={proof_sha256}");
    println!("verified_proof=true");
    println!("proof_generated=true");
    Ok(())
}

fn run_verify<P: Prover>(client: &P, path: &Path, raw_sha256: &str, elf_path: Option<&Path>) -> Result<(), String> {
    // With --elf, verify against the exact guest ELF the prover built (its bytes are pinned by
    // SHA-256 in the manifest); without it, against this binary's embedded ELF.
    let elf: Elf = match elf_path {
        Some(p) => Elf::from(fs::read(p).map_err(|e| format!("cannot read ELF {}: {e}", p.display()))?),
        None => ZEEBEAM_ELF,
    };
    let (proof, bytes, digest) = load_exact(path)?;
    if !matches!(&proof.proof, SP1Proof::Groth16(_)) {
        return Err("loaded proof is not Groth16".into());
    }
    if proof.sp1_version != SP1_CIRCUIT_VERSION {
        return Err("loaded proof uses a different circuit version".into());
    }
    let elf_sha256 = sha256_hex(&elf);
    let proving_key = client.setup(elf).map_err(|e| format!("SP1 setup failed: {e}"))?;
    let vkey = proving_key.verifying_key().bytes32();
    verify_original_and_tamper(client, &proof, proving_key.verifying_key())?;
    // the oracle: rebuild the expectation for the row the proof names, without any raw frame
    let public = proof.public_values.to_vec();
    if public.len() != TOTAL_PUBLIC_BYTES {
        return Err("public value length differs".into());
    }
    let built = witness::build(Vec::new());
    let report = witness::check(&public, &built)?;
    println!("classification={CLASSIFICATION}");
    println!("mode=cold_verify_groth16");
    println!("sp1_vkey={vkey}");
    println!("guest_elf_sha256={elf_sha256}");
    println!("proof_bytes={bytes} proof_sha256={digest}");
    println!("raw_frame_sha256_claimed={raw_sha256}");
    println!("oracle={} checked={} circuit_derived={}", report.mode, report.bytes_checked, report.bytes_circuit_derived);
    println!("public_values_sha256={}", sha256_hex(&public));
    println!("verified_proof=true");
    Ok(())
}

fn main() {
    sp1_sdk::utils::setup_logger();
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(command) = args.first() else { usage() };
    let prover = arg(&args, "--prover").unwrap_or_else(|| "cpu".into());
    let result = match command.as_str() {
        "prove" => {
            let raw = PathBuf::from(arg(&args, "--raw").unwrap_or_else(|| usage()));
            let out = PathBuf::from(arg(&args, "--out").unwrap_or_else(|| usage()));
            let cycle_limit = arg(&args, "--cycle-limit").map(|v| v.parse().expect("cycle limit")).unwrap_or(DEFAULT_CYCLE_LIMIT);
            match prover.as_str() {
                "cpu" => run_prove(&ProverClient::builder().cpu().build(), "local_cpu_explicit", &raw, &out, cycle_limit),
                #[cfg(feature = "cuda")]
                "cuda" => run_prove(&ProverClient::builder().cuda().build(), "local_cuda_explicit", &raw, &out, cycle_limit),
                other => Err(format!("unknown or unbuilt prover {other}")),
            }
        }
        "vkey" => {
            // derive and print the verification key of an ELF (this binary's embedded one, or --elf)
            let elf: Elf = match arg(&args, "--elf") {
                Some(p) => Elf::from(fs::read(&p).expect("read ELF")),
                None => ZEEBEAM_ELF,
            };
            let sha = sha256_hex(&elf);
            let len = elf.len();
            let client = ProverClient::builder().cpu().build();
            client.setup(elf).map(|pk| {
                println!("guest_elf_bytes={len} guest_elf_sha256={sha}");
                println!("sp1_vkey={}", pk.verifying_key().bytes32());
            }).map_err(|e| format!("SP1 setup failed: {e}"))
        }
        "export" => {
            // raw on-chain-style bytes for a standalone verifier: 4-byte Groth16 vkey-hash prefix +
            // encoded proof (proof.bytes()), and the 1,085 public-value bytes
            let proof = PathBuf::from(arg(&args, "--proof").unwrap_or_else(|| usage()));
            let out = PathBuf::from(arg(&args, "--out").unwrap_or_else(|| usage()));
            (|| -> Result<(), String> {
                let (p, _, digest) = load_exact(&proof)?;
                if !matches!(&p.proof, SP1Proof::Groth16(_)) {
                    return Err("not a Groth16 proof".into());
                }
                let stem = proof.file_stem().unwrap().to_string_lossy().to_string();
                let raw = p.bytes();
                let public = p.public_values.to_vec();
                fs::write(out.join(format!("{stem}_proof.bin")), &raw).map_err(|e| e.to_string())?;
                fs::write(out.join(format!("{stem}_public_values.bin")), &public).map_err(|e| e.to_string())?;
                println!("source_artifact_sha256={digest}");
                println!("raw_proof_bytes={} raw_proof_sha256={}", raw.len(), sha256_hex(&raw));
                println!("public_values_bytes={} public_values_sha256={}", public.len(), sha256_hex(&public));
                println!("sp1_version={}", p.sp1_version);
                Ok(())
            })()
        }
        "verify" => {
            let proof = PathBuf::from(arg(&args, "--proof").unwrap_or_else(|| usage()));
            let raw_sha = arg(&args, "--raw-sha256").unwrap_or_default();
            let elf = arg(&args, "--elf").map(PathBuf::from);
            run_verify(&ProverClient::builder().cpu().build(), &proof, &raw_sha, elf.as_deref())
        }
        _ => usage(),
    };
    if let Err(e) = result {
        eprintln!("CEREMONY FAILED: {e}");
        std::process::exit(1);
    }
}

#[allow(dead_code)]
fn _stdin_type_check(_: SP1Stdin) {}
