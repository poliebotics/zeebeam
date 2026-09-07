//! Exact local CPU Groth16 prover and cold verifier for the frozen membership guest.

use std::ffi::OsString;
use std::fs::{self, File, OpenOptions, Permissions};
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::time::Instant;

use bincode::Options;
use sha2::{Digest, Sha256};
use sp1_sdk::{
    blocking::{ProveRequest, Prover, ProverClient},
    include_elf, Elf, HashableKey, ProvingKey, SP1Proof, SP1ProofWithPublicValues, SP1PublicValues,
    SP1Stdin, SP1VerificationError, SP1_CIRCUIT_VERSION,
};
use zeebeam_row_binding_membership::{CLASSIFICATION, PUBLIC_BYTES};
use zeebeam_row_binding_membership_native::{
    check_public_values, expected_row96_public_values, row96_header_bytes,
    row96_membership_witness_bytes,
};

const ZEEBEAM_ELF: Elf = include_elf!("zeebeam-row-binding-membership-program");
const RAW_BYTES: u64 = 24_472_000;
const RAW_SHA256: &str = "552e9e7ef2d70346665be990bed9c4d0fc82d65a2e561b244e95075e4df3bd30";
const ELF_BYTES: usize = 377_576;
const ELF_SHA256: &str = "2b1ac3e0bc682946357e0f683081b0ec61fb83a4230158ae935f9bd4da6df160";
const VKEY_HASH: &str = "0x00e71b811e57ddefa1f4c37da64b52f34dea3ad2136c55fc0d72c797cda800af";
const PUBLIC_SHA256: &str = "df95a5bf891be3b8573aaf60bbbaf21ae6f18f18de9c60acc2ebf4df403e6abb";
const EXPECTED_CIRCUIT_VERSION: &str = "v6.1.0";
const TAMPER_PUBLIC_BYTE: usize = 87;
const TAMPER_GROTH16_PROOF_BYTE: usize = 96;
const PROVE_CYCLE_LIMIT: u64 = 4_300_000_000;
const MAX_PROOF_BYTES: u64 = 512 * 1024 * 1024 * 1024;
const FORBIDDEN_AMBIENT_VARIABLES: &[&str] = &[
    "DUMP_ELF_OUTPUT",
    "ELEMENT_THRESHOLD",
    "FULL_SIZE_SHARDS",
    "GAS_TRACE_CHUNK_SLOTS",
    "GAS_TRACE_CHUNK_THRESHOLD",
    "HEIGHT_THRESHOLD",
    "MEMORY_LIMIT",
    "MINIMAL_TRACE_CHUNK_THRESHOLD",
    "NETWORK_PRIVATE_KEY",
    "NETWORK_RPC_URL",
    "RUST_LOGGER",
    "SHARD_SIZE",
    "SKIP_SIMULATION",
    "TRACE_CHUNK_SLOTS",
    "TRACE_FILE",
    "TRACE_SAMPLE_RATE",
    "WITHOUT_VK_VERIFICATION",
];

#[derive(Debug, PartialEq, Eq)]
enum Command {
    Prove {
        raw: PathBuf,
        output: PathBuf,
    },
    Verify {
        proof: PathBuf,
        expected_bytes: u64,
        expected_sha256: String,
    },
}
fn usage() -> &'static str {
    "usage:\n  zeebeam-row-binding-membership-groth16 prove --raw PATH --output PATH\n  zeebeam-row-binding-membership-groth16 verify --proof PATH --expected-bytes N --expected-sha256 HEX"
}

fn parse_u64(value: &OsString, label: &str) -> Result<u64, String> {
    let text = value
        .to_str()
        .ok_or_else(|| format!("{label} is not UTF-8"))?;
    if text.is_empty()
        || text == "0"
        || text.starts_with('0')
        || text.bytes().any(|byte| !byte.is_ascii_digit())
    {
        return Err(format!("{label} is not a canonical positive u64"));
    }
    text.parse::<u64>()
        .map_err(|_| format!("{label} is not a canonical positive u64"))
}

fn parse_digest(value: &OsString) -> Result<String, String> {
    let digest = value
        .to_str()
        .ok_or_else(|| "expected SHA-256 is not UTF-8".to_string())?;
    if digest.len() != 64
        || digest
            .bytes()
            .any(|byte| !byte.is_ascii_digit() && !(b'a'..=b'f').contains(&byte))
    {
        return Err("expected SHA-256 is not 64 lowercase hexadecimal digits".to_string());
    }
    Ok(digest.to_string())
}

fn parse_arguments(arguments: Vec<OsString>) -> Result<Command, String> {
    match arguments.as_slice() {
        [command, raw_flag, raw, output_flag, output]
            if command == "prove" && raw_flag == "--raw" && output_flag == "--output" =>
        {
            Ok(Command::Prove {
                raw: PathBuf::from(raw),
                output: PathBuf::from(output),
            })
        }
        [command, proof_flag, proof, bytes_flag, expected_bytes, digest_flag, digest]
            if command == "verify"
                && proof_flag == "--proof"
                && bytes_flag == "--expected-bytes"
                && digest_flag == "--expected-sha256" =>
        {
            Ok(Command::Verify {
                proof: PathBuf::from(proof),
                expected_bytes: parse_u64(expected_bytes, "expected bytes")?,
                expected_sha256: parse_digest(digest)?,
            })
        }
        _ => Err(usage().to_string()),
    }
}

fn is_forbidden_ambient_name(name: &str) -> bool {
    name.starts_with("SP1_") || FORBIDDEN_AMBIENT_VARIABLES.contains(&name)
}

fn reject_ambient_sp1_controls() -> Result<(), String> {
    let mut present = Vec::new();
    for (name, _) in std::env::vars_os() {
        let Some(name) = name.to_str() else {
            continue;
        };
        if is_forbidden_ambient_name(name) {
            present.push(name.to_string());
        }
    }
    present.sort_unstable();
    if present.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "forbidden ambient SP1 control variables are set: {}",
            present.join(",")
        ))
    }
}

fn require_reader_eof(reader: &mut impl Read) -> Result<(), String> {
    let mut trailing = [0_u8; 1];
    let trailing_bytes = reader
        .read(&mut trailing)
        .map_err(|error| format!("cannot test proof framing endpoint: {error}"))?;
    if trailing_bytes == 0 {
        Ok(())
    } else {
        Err("proof framing has trailing bytes".to_string())
    }
}

fn open_regular_nofollow(path: &Path, expected_bytes: Option<u64>) -> Result<File, String> {
    let file = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC)
        .open(path)
        .map_err(|error| {
            format!(
                "cannot open {} without following links: {error}",
                path.display()
            )
        })?;
    let metadata = file
        .metadata()
        .map_err(|error| format!("cannot fstat {}: {error}", path.display()))?;
    if !metadata.file_type().is_file() {
        return Err(format!(
            "{} is not a regular non-symlink file",
            path.display()
        ));
    }
    if let Some(expected) = expected_bytes {
        if metadata.len() != expected {
            return Err(format!(
                "{} has {} bytes, expected {expected}",
                path.display(),
                metadata.len()
            ));
        }
    }
    Ok(file)
}

fn sha256_open_file(file: &mut File, label: &str) -> Result<(u64, String), String> {
    file.seek(SeekFrom::Start(0))
        .map_err(|error| format!("cannot rewind {label}: {error}"))?;
    let mut hasher = Sha256::new();
    let mut buffer = vec![0_u8; 1024 * 1024];
    let mut total = 0_u64;
    loop {
        let count = file
            .read(&mut buffer)
            .map_err(|error| format!("cannot hash {label}: {error}"))?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
        total = total
            .checked_add(u64::try_from(count).expect("usize fits u64"))
            .ok_or_else(|| "hashed byte count overflowed".to_string())?;
    }
    file.seek(SeekFrom::Start(0))
        .map_err(|error| format!("cannot rewind {label}: {error}"))?;
    Ok((total, format!("{:x}", hasher.finalize())))
}

fn checked_raw(path: &Path) -> Result<Vec<u8>, String> {
    let file = open_regular_nofollow(path, Some(RAW_BYTES))?;
    let mut raw = Vec::with_capacity(usize::try_from(RAW_BYTES).expect("raw size fits usize"));
    file.take(RAW_BYTES + 1)
        .read_to_end(&mut raw)
        .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
    if u64::try_from(raw.len()).expect("usize fits u64") != RAW_BYTES
        || format!("{:x}", Sha256::digest(&raw)) != RAW_SHA256
    {
        return Err("raw input differs from the frozen row-96 artifact".to_string());
    }
    Ok(raw)
}

fn stdin_for(raw: Vec<u8>) -> SP1Stdin {
    let mut stdin = SP1Stdin::new();
    stdin.write_vec(row96_header_bytes().to_vec());
    stdin.write_vec(row96_membership_witness_bytes());
    stdin.write_vec(raw);
    stdin
}

fn public_oracle() -> Result<Vec<u8>, String> {
    let expected = expected_row96_public_values();
    if expected.len() != PUBLIC_BYTES || format!("{:x}", Sha256::digest(&expected)) != PUBLIC_SHA256
    {
        return Err("compiled public oracle differs".to_string());
    }
    Ok(expected)
}

fn pinned_program_identity(proving_key: &impl ProvingKey) -> Result<String, String> {
    if ZEEBEAM_ELF.len() != ELF_BYTES
        || format!("{:x}", Sha256::digest(&*ZEEBEAM_ELF)) != ELF_SHA256
    {
        return Err("embedded guest ELF differs from the frozen artifact".to_string());
    }
    let vkey = proving_key.verifying_key().bytes32();
    if vkey != VKEY_HASH {
        return Err("derived SP1 verification key differs from the frozen key".to_string());
    }
    Ok(vkey)
}

fn pinned_circuit_version() -> Result<(), String> {
    if SP1_CIRCUIT_VERSION != EXPECTED_CIRCUIT_VERSION {
        return Err(format!(
            "compiled SP1 circuit version is {SP1_CIRCUIT_VERSION}, expected {EXPECTED_CIRCUIT_VERSION}"
        ));
    }
    Ok(())
}

fn mutate_groth16_proof_byte(proof: &mut SP1ProofWithPublicValues) -> Result<(), String> {
    let SP1Proof::Groth16(groth16) = &mut proof.proof else {
        return Err("cannot mutate a non-Groth16 proof".to_string());
    };
    let mut encoded = groth16.encoded_proof.as_bytes().to_vec();
    if encoded.iter().any(|byte| !byte.is_ascii_hexdigit()) || encoded.len() % 2 != 0 {
        return Err("Groth16 encoded proof is not canonical hexadecimal".to_string());
    }
    let nibble = TAMPER_GROTH16_PROOF_BYTE
        .checked_mul(2)
        .ok_or_else(|| "Groth16 proof mutation offset overflowed".to_string())?;
    let byte = encoded
        .get_mut(nibble)
        .ok_or_else(|| "Groth16 encoded proof is too short for mutation".to_string())?;
    *byte = if *byte == b'0' { b'1' } else { b'0' };
    groth16.encoded_proof =
        String::from_utf8(encoded).map_err(|_| "mutated Groth16 proof is not UTF-8".to_string())?;
    Ok(())
}

fn verify_original_and_tamper<P: Prover>(
    client: &P,
    proof: &mut SP1ProofWithPublicValues,
    vkey: &sp1_sdk::SP1VerifyingKey,
    expected: &[u8],
) -> Result<(), String> {
    if proof.tee_proof.is_some() {
        return Err("proof unexpectedly contains a TEE attachment".to_string());
    }
    check_public_values(proof.public_values.as_slice(), expected).map_err(ToString::to_string)?;
    client
        .verify(proof, vkey, None)
        .map_err(|error| format!("proof verification failed: {error}"))?;

    let mut public_tampered = proof.clone();
    let mut changed = public_tampered.public_values.to_vec();
    changed[TAMPER_PUBLIC_BYTE] ^= 1;
    public_tampered.public_values = SP1PublicValues::from(&changed);
    match client.verify(&public_tampered, vkey, None) {
        Err(SP1VerificationError::InvalidPublicValues) => {}
        Err(error) => Err(format!(
            "single-bit public mutation produced an unexpected verifier error: {error}"
        ))?,
        Ok(()) => return Err("single-bit public mutation was accepted".to_string()),
    }

    let mut proof_tampered = proof.clone();
    mutate_groth16_proof_byte(&mut proof_tampered)?;
    match client.verify(&proof_tampered, vkey, None) {
        Err(SP1VerificationError::Groth16(_)) => {}
        Err(error) => {
            return Err(format!(
                "single-nibble Groth16 proof mutation produced an unexpected verifier error: {error}"
            ));
        }
        Ok(()) => return Err("single-nibble Groth16 proof mutation was accepted".to_string()),
    }

    let mut wrong_vkey = vkey.clone();
    wrong_vkey.vk.preprocessed_commit.rotate_left(1);
    if wrong_vkey.bytes32() == vkey.bytes32() {
        return Err(
            "deterministic wrong-verifying-key mutation did not change its hash".to_string(),
        );
    }
    match client.verify(proof, &wrong_vkey, None) {
        Err(SP1VerificationError::Groth16(_)) => Ok(()),
        Err(error) => Err(format!(
            "wrong verifying key produced an unexpected verifier error: {error}"
        )),
        Ok(()) => Err("wrong verifying key was accepted".to_string()),
    }
}

fn output_paths(output: &Path) -> Result<(PathBuf, PathBuf), String> {
    if !output.is_absolute() || output.file_name().is_none() {
        return Err("proof output must be an absolute file path".to_string());
    }
    if output.exists() || fs::symlink_metadata(output).is_ok() {
        return Err("proof output already exists".to_string());
    }
    let parent = output
        .parent()
        .ok_or_else(|| "proof output has no parent".to_string())?;
    let parent_metadata = fs::symlink_metadata(parent)
        .map_err(|error| format!("cannot stat proof output parent: {error}"))?;
    if !parent_metadata.is_dir() || parent_metadata.file_type().is_symlink() {
        return Err("proof output parent is not a real directory".to_string());
    }
    let name = output
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| "proof output name is not UTF-8".to_string())?;
    if name.is_empty()
        || name.len() > 128
        || name
            .bytes()
            .any(|byte| !byte.is_ascii_alphanumeric() && !matches!(byte, b'.' | b'_' | b'-'))
    {
        return Err("proof output name is not canonical".to_string());
    }
    let pending = parent.join(format!(".{name}.pending.{}", std::process::id()));
    if pending.exists() || fs::symlink_metadata(&pending).is_ok() {
        return Err("proof pending output already exists".to_string());
    }
    Ok((parent.to_path_buf(), pending))
}

fn save_exact(
    proof: &SP1ProofWithPublicValues,
    output: &Path,
    parent: &Path,
    pending: &Path,
) -> Result<(u64, String), String> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(pending)
        .map_err(|error| format!("cannot create {}: {error}", pending.display()))?;
    let mut writer = BufWriter::new(file);
    bincode::DefaultOptions::new()
        .with_fixint_encoding()
        .reject_trailing_bytes()
        .serialize_into(&mut writer, proof)
        .map_err(|error| format!("cannot serialize proof: {error}"))?;
    writer
        .flush()
        .map_err(|error| format!("cannot flush proof: {error}"))?;
    writer
        .get_ref()
        .sync_all()
        .map_err(|error| format!("cannot fsync proof: {error}"))?;
    let mut file = writer
        .into_inner()
        .map_err(|error| format!("cannot finish proof stream: {error}"))?;
    let identity = sha256_open_file(&mut file, "pending proof")?;
    if identity.0 > MAX_PROOF_BYTES {
        drop(file);
        fs::remove_file(pending)
            .map_err(|error| format!("cannot remove oversized pending proof: {error}"))?;
        return Err(format!(
            "generated proof exceeds the {MAX_PROOF_BYTES}-byte verifier limit"
        ));
    }
    file.set_permissions(Permissions::from_mode(0o400))
        .map_err(|error| format!("cannot make pending proof read-only: {error}"))?;
    file.sync_all()
        .map_err(|error| format!("cannot fsync read-only proof: {error}"))?;
    drop(file);
    fs::hard_link(pending, output)
        .map_err(|error| format!("cannot commit proof without replacement: {error}"))?;
    fs::remove_file(pending)
        .map_err(|error| format!("cannot remove committed pending link: {error}"))?;
    File::open(parent)
        .and_then(|directory| directory.sync_all())
        .map_err(|error| format!("cannot fsync proof directory: {error}"))?;
    Ok(identity)
}

fn load_exact(
    path: &Path,
    expected_bytes: u64,
    expected_sha256: &str,
) -> Result<SP1ProofWithPublicValues, String> {
    if expected_bytes > MAX_PROOF_BYTES {
        return Err(format!(
            "proof exceeds the {MAX_PROOF_BYTES}-byte verifier limit"
        ));
    }
    let mut file = open_regular_nofollow(path, Some(expected_bytes))?;
    let (bytes, digest) = sha256_open_file(&mut file, "proof")?;
    if bytes != expected_bytes || digest != expected_sha256 {
        return Err("proof artifact identity differs".to_string());
    }
    let proof = {
        let mut reader = BufReader::new(&mut file);
        let proof = bincode::DefaultOptions::new()
            .with_fixint_encoding()
            .with_limit(expected_bytes)
            .reject_trailing_bytes()
            .deserialize_from(&mut reader)
            .map_err(|error| format!("proof framing is not exact: {error}"))?;
        require_reader_eof(&mut reader)?;
        proof
    };
    let identity_after = sha256_open_file(&mut file, "proof after decoding")?;
    if identity_after != (bytes, digest) {
        return Err("proof artifact changed while it was decoded".to_string());
    }
    Ok(proof)
}

fn prove(raw: &Path, output: &Path) -> Result<(), String> {
    reject_ambient_sp1_controls()?;
    pinned_circuit_version()?;
    let raw = checked_raw(raw)?;
    let expected = public_oracle()?;
    let (parent, pending) = output_paths(output)?;
    let client = ProverClient::builder().cpu().build();

    let setup_started = Instant::now();
    let proving_key = client
        .setup(ZEEBEAM_ELF)
        .map_err(|error| format!("SP1 setup failed: {error}"))?;
    let vkey = pinned_program_identity(&proving_key)?;
    let setup_ms = setup_started.elapsed().as_millis();

    println!("classification={CLASSIFICATION}");
    println!("mode=groth16");
    println!("prover=local_cpu_explicit");
    println!("ambient_sp1_controls=absent");
    println!("sp1_groth16_opts=pinned_6.4.0_defaults");
    println!("proof_nonce_u32x4=0,0,0,0");
    println!("prove_cycle_limit={PROVE_CYCLE_LIMIT}");
    println!("raw_bytes={RAW_BYTES}");
    println!("raw_sha256={RAW_SHA256}");
    println!("guest_elf_bytes={ELF_BYTES}");
    println!("guest_elf_sha256={ELF_SHA256}");
    println!("public_values_bytes={PUBLIC_BYTES}");
    println!("public_values_sha256={PUBLIC_SHA256}");
    println!("sp1_vkey={vkey}");
    println!("sp1_circuit_version={EXPECTED_CIRCUIT_VERSION}");
    println!("setup_elapsed_ms={setup_ms}");

    let prove_started = Instant::now();
    let mut proof = client
        .prove(&proving_key, stdin_for(raw))
        .groth16()
        .cycle_limit(PROVE_CYCLE_LIMIT)
        .with_proof_nonce([0, 0, 0, 0])
        .run()
        .map_err(|error| format!("SP1 Groth16 proof failed: {error}"))?;
    let prove_ms = prove_started.elapsed().as_millis();
    if !matches!(&proof.proof, SP1Proof::Groth16(_)) {
        return Err("SP1 returned a non-Groth16 proof".to_string());
    }
    if proof.sp1_version != EXPECTED_CIRCUIT_VERSION {
        return Err("SP1 returned a proof under a different circuit version".to_string());
    }

    let verify_started = Instant::now();
    verify_original_and_tamper(&client, &mut proof, proving_key.verifying_key(), &expected)?;
    let verify_ms = verify_started.elapsed().as_millis();
    let (proof_bytes, proof_sha256) = save_exact(&proof, output, &parent, &pending)?;

    println!("proof_mode=Groth16");
    println!("sp1_version={}", proof.sp1_version);
    println!("prove_elapsed_ms={prove_ms}");
    println!("verify_and_tamper_elapsed_ms={verify_ms}");
    println!("verified_proof=true");
    println!("tamper_publics_bit_at_byte{TAMPER_PUBLIC_BYTE}_outcome=REJECT");
    println!("tamper_groth16_proof_byte_at_offset{TAMPER_GROTH16_PROOF_BYTE}_outcome=REJECT");
    println!("wrong_verifying_key_outcome=REJECT");
    println!("proof_output={}", output.display());
    println!("proof_bytes={proof_bytes}");
    println!("proof_sha256={proof_sha256}");
    println!("proof_generated=true");
    Ok(())
}

fn verify(path: &Path, expected_bytes: u64, expected_sha256: &str) -> Result<(), String> {
    reject_ambient_sp1_controls()?;
    pinned_circuit_version()?;
    let expected = public_oracle()?;
    let client = ProverClient::builder().cpu().build();
    let setup_started = Instant::now();
    let proving_key = client
        .setup(ZEEBEAM_ELF)
        .map_err(|error| format!("SP1 setup failed: {error}"))?;
    let vkey = pinned_program_identity(&proving_key)?;
    let setup_ms = setup_started.elapsed().as_millis();
    let mut proof = load_exact(path, expected_bytes, expected_sha256)?;
    if !matches!(&proof.proof, SP1Proof::Groth16(_)) {
        return Err("loaded proof is not Groth16 mode".to_string());
    }
    if proof.sp1_version != EXPECTED_CIRCUIT_VERSION {
        return Err("loaded proof uses a different circuit version".to_string());
    }
    let verify_started = Instant::now();
    verify_original_and_tamper(&client, &mut proof, proving_key.verifying_key(), &expected)?;
    println!("classification={CLASSIFICATION}");
    println!("mode=cold_verify_groth16");
    println!("ambient_sp1_controls=absent");
    println!("sp1_groth16_opts=pinned_6.4.0_defaults");
    println!("guest_elf_bytes={ELF_BYTES}");
    println!("guest_elf_sha256={ELF_SHA256}");
    println!("sp1_vkey={vkey}");
    println!("sp1_circuit_version={EXPECTED_CIRCUIT_VERSION}");
    println!("setup_elapsed_ms={setup_ms}");
    println!("proof_bytes={expected_bytes}");
    println!("proof_sha256={expected_sha256}");
    println!("public_values_bytes={PUBLIC_BYTES}");
    println!("public_values_sha256={PUBLIC_SHA256}");
    println!("verify_elapsed_ms={}", verify_started.elapsed().as_millis());
    println!("verified_proof=true");
    println!("tamper_publics_bit_at_byte{TAMPER_PUBLIC_BYTE}_outcome=REJECT");
    println!("tamper_groth16_proof_byte_at_offset{TAMPER_GROTH16_PROOF_BYTE}_outcome=REJECT");
    println!("wrong_verifying_key_outcome=REJECT");
    Ok(())
}

fn main() {
    if let Err(error) = reject_ambient_sp1_controls() {
        eprintln!("ERROR: {error}");
        std::process::exit(1);
    }
    sp1_sdk::utils::setup_logger();
    let command = parse_arguments(std::env::args_os().skip(1).collect()).unwrap_or_else(|error| {
        eprintln!("{error}");
        std::process::exit(2);
    });
    let result = match command {
        Command::Prove { raw, output } => prove(&raw, &output),
        Command::Verify {
            proof,
            expected_bytes,
            expected_sha256,
        } => verify(&proof, expected_bytes, &expected_sha256),
    };
    if let Err(error) = result {
        eprintln!("ERROR: {error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn values(items: &[&str]) -> Vec<OsString> {
        items.iter().map(OsString::from).collect()
    }

    #[test]
    fn exact_prove_grammar() {
        assert_eq!(
            parse_arguments(values(&[
                "prove",
                "--raw",
                "/input.raw",
                "--output",
                "/output/groth16.bin",
            ])),
            Ok(Command::Prove {
                raw: PathBuf::from("/input.raw"),
                output: PathBuf::from("/output/groth16.bin"),
            })
        );
    }

    #[test]
    fn exact_verify_grammar() {
        assert_eq!(
            parse_arguments(values(&[
                "verify",
                "--proof",
                "/output/groth16.bin",
                "--expected-bytes",
                "123",
                "--expected-sha256",
                "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            ])),
            Ok(Command::Verify {
                proof: PathBuf::from("/output/groth16.bin"),
                expected_bytes: 123,
                expected_sha256: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
                    .to_string(),
            })
        );
    }

    #[test]
    fn ambiguous_or_noncanonical_arguments_fail() {
        assert!(parse_arguments(values(&["prove", "--raw", "x"])).is_err());
        assert!(parse_arguments(values(&[
            "verify",
            "--proof",
            "p",
            "--expected-bytes",
            "01",
            "--expected-sha256",
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        ]))
        .is_err());
        assert!(parse_digest(&OsString::from(
            "0123456789ABCDEF0123456789abcdef0123456789abcdef0123456789abcdef"
        ))
        .is_err());
    }

    #[test]
    fn ambient_controls_and_exact_endpoint_are_fail_closed() {
        assert!(is_forbidden_ambient_name("SP1_DUMP"));
        assert!(is_forbidden_ambient_name("SP1_FUTURE_CONTROL"));
        assert!(is_forbidden_ambient_name("TRACE_FILE"));
        assert!(is_forbidden_ambient_name("MEMORY_LIMIT"));
        assert!(is_forbidden_ambient_name("RUST_LOGGER"));
        assert!(!is_forbidden_ambient_name("RAYON_NUM_THREADS"));
        assert!(!is_forbidden_ambient_name("RUST_LOG"));
        assert_eq!(pinned_circuit_version(), Ok(()));

        let mut empty = std::io::Cursor::new(Vec::<u8>::new());
        assert_eq!(require_reader_eof(&mut empty), Ok(()));
        let mut trailing = std::io::Cursor::new(vec![0_u8]);
        assert_eq!(
            require_reader_eof(&mut trailing),
            Err("proof framing has trailing bytes".to_string())
        );
    }

    #[test]
    fn deterministic_groth16_proof_byte_mutation_is_exact() {
        let mut proof = SP1ProofWithPublicValues::new(
            SP1Proof::Groth16(Default::default()),
            SP1PublicValues::from(&[] as &[u8]),
            EXPECTED_CIRCUIT_VERSION.to_string(),
        );
        let SP1Proof::Groth16(groth16) = &mut proof.proof else {
            unreachable!();
        };
        groth16.encoded_proof = "ab".repeat(352);
        let before = groth16.encoded_proof.clone();
        mutate_groth16_proof_byte(&mut proof).unwrap();
        let SP1Proof::Groth16(groth16) = &proof.proof else {
            unreachable!();
        };
        assert_eq!(groth16.encoded_proof.len(), before.len());
        assert_eq!(
            groth16
                .encoded_proof
                .bytes()
                .zip(before.bytes())
                .filter(|(left, right)| left != right)
                .count(),
            1
        );
        assert_eq!(
            groth16.encoded_proof.as_bytes()[TAMPER_GROTH16_PROOF_BYTE * 2],
            b'0'
        );
    }
}
