//! Execute-only local CPU host for the candidate relation.

use std::path::PathBuf;
use std::time::Instant;

use sp1_sdk::{
    blocking::{Prover, ProverClient},
    include_elf, Elf, SP1Stdin,
};
use zeebeam_row_binding_join::{CLASSIFICATION, PUBLIC_BYTES, ROW_MEMBERSHIP_PROVED};
use zeebeam_row_binding_join_native::{
    check_public_values, expected_row96_public_values, row96_header_bytes,
};

const ZEEBEAM_ELF: Elf = include_elf!("zeebeam-row-binding-join-program");

fn raw_path() -> PathBuf {
    let arguments = std::env::args_os().skip(1).collect::<Vec<_>>();
    if arguments.len() != 2 || arguments[0] != "--raw" {
        eprintln!("usage: zeebeam-row-binding-join-execute --raw PATH");
        std::process::exit(2);
    }
    PathBuf::from(&arguments[1])
}

fn main() {
    sp1_sdk::utils::setup_logger();
    let raw = std::fs::read(raw_path()).expect("cannot read the caller-selected raw frame");
    let expected = expected_row96_public_values();
    assert_eq!(expected.len(), PUBLIC_BYTES);

    let mut stdin = SP1Stdin::new();
    stdin.write_vec(row96_header_bytes().to_vec());
    stdin.write_vec(raw);

    println!("classification={CLASSIFICATION}");
    println!("row_membership_proved={ROW_MEMBERSHIP_PROVED}");
    println!("session_identifier_in_relation=false");
    println!("chain_log_commitment_in_relation=false");
    println!("mode=execute_only");
    println!("prover=local_cpu_explicit");

    let client = ProverClient::builder().cpu().build();
    let started = Instant::now();
    let (output, report) = client
        .execute(ZEEBEAM_ELF, stdin)
        .run()
        .expect("SP1 candidate execute failed");
    check_public_values(output.as_slice(), &expected)
        .expect("SP1 public values differ from the frozen row-96 oracle");

    println!("public_values_bytes={PUBLIC_BYTES}");
    println!(
        "total_instruction_count={}",
        report.total_instruction_count()
    );
    println!("total_syscall_count={}", report.total_syscall_count());
    println!("host_elapsed_ms={}", started.elapsed().as_millis());
    println!("verified_public_values=true");
    println!("proof_generated=false");
}
