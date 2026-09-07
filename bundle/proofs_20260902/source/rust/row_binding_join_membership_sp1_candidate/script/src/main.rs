//! Execute-only local CPU host for the ordered-membership successor relation with the August
//! capture-time anchor. Witnesses and oracle checks live in witness.rs (shared with the ceremony).

mod witness;

use std::path::PathBuf;
use std::time::Instant;

use sp1_sdk::{
    blocking::{Prover, ProverClient},
    include_elf, Elf,
};
use zeebeam_row_binding_membership::{
    AUTHORITY_MANIFEST_DIGEST_IN_CONTEXT, CHAIN_LOG_DIGEST_IN_CONTEXT, CLASSIFICATION,
    PUBLIC_BYTES, ROW_MEMBERSHIP_RELATION_ENFORCED, SESSION_IDENTIFIER_IN_RELATION,
};
use zeebeam_row_binding_membership_native::TOTAL_PUBLIC_BYTES;

const ZEEBEAM_ELF: Elf = include_elf!("zeebeam-row-binding-membership-program");

fn raw_path() -> PathBuf {
    let arguments = std::env::args_os().skip(1).collect::<Vec<_>>();
    if arguments.len() != 2 || arguments[0] != "--raw" {
        eprintln!("usage: zeebeam-row-binding-membership-execute --raw PATH");
        std::process::exit(2);
    }
    PathBuf::from(&arguments[1])
}

fn main() {
    sp1_sdk::utils::setup_logger();
    let raw = std::fs::read(raw_path()).expect("cannot read the caller-selected raw frame");
    let built = witness::build(raw);

    println!("classification={CLASSIFICATION}");
    println!("row_membership_relation_enforced={ROW_MEMBERSHIP_RELATION_ENFORCED}");
    println!("row_membership_proved=false");
    println!("session_identifier_in_relation={SESSION_IDENTIFIER_IN_RELATION}");
    println!("authority_manifest_digest_in_context={AUTHORITY_MANIFEST_DIGEST_IN_CONTEXT}");
    println!("chain_log_digest_in_context={CHAIN_LOG_DIGEST_IN_CONTEXT}");
    println!("mode=execute_only");
    println!("prover=local_cpu_explicit");
    println!("row={}", built.row);

    let client = ProverClient::builder().cpu().build();
    let started = Instant::now();
    let (output, report) = client
        .execute(ZEEBEAM_ELF, built.stdin.clone())
        .run()
        .expect("SP1 ordered-membership execute failed");
    let actual = output.as_slice();
    match witness::check(actual, &built) {
        Ok(oracle) => {
            println!("oracle={} checked={} circuit_derived={}", oracle.mode, oracle.bytes_checked, oracle.bytes_circuit_derived);
            for n in &oracle.notes {
                println!("oracle_note={n}");
            }
            println!("public_values_hex={}", witness::hex(actual));
            if built.row_json.is_some() {
                println!("verified_public_values={}", if oracle.bytes_circuit_derived == 0 { "true" } else { "partial" });
            } else {
                println!("verified_public_values=true");
            }
        }
        Err(e) => {
            eprintln!("MISMATCH: {e}");
            panic!("public values differ from the oracle");
        }
    }

    println!("public_values_bytes={}", TOTAL_PUBLIC_BYTES);
    println!("pre_zcash_public_bytes={}", PUBLIC_BYTES);
    println!("total_instruction_count={}", report.total_instruction_count());
    println!("total_syscall_count={}", report.total_syscall_count());
    {
        let mut regions: Vec<_> = report.cycle_tracker.iter().collect();
        regions.sort_by(|a, b| b.1.cmp(a.1));
        for (name, cycles) in regions {
            println!("CYCLE_REGION {name} = {cycles}");
        }
    }
    println!("host_elapsed_ms={}", started.elapsed().as_millis());
    println!("membership_opening_verified=true");
    println!("proof_generated=false");
}
