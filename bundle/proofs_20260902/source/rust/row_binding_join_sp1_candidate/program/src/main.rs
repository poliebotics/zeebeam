//! SP1 guest for the candidate-only v9 row binding join.

#![no_main]
sp1_zkvm::entrypoint!(main);

use zeebeam_row_binding_join::{evaluate, RowWitnessHeader, CLASSIFICATION, ROW_MEMBERSHIP_PROVED};

const MODEL_BLOB: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../../scratch/zeebeam_lambda_launch_prep_20260823/",
    "zeebeam-trained-r32-ptq-v2-sp1-v1/frozen/r32_fit_calibrated_ptq_v2.bin"
));

pub fn main() {
    println!("cycle-tracker-report-start: private_input");
    let header_bytes = sp1_zkvm::io::read_vec();
    let raw_bayer = sp1_zkvm::io::read_vec();
    let header = RowWitnessHeader::parse(&header_bytes)
        .unwrap_or_else(|error| panic!("private candidate header rejected: {error}"));
    println!("cycle-tracker-report-end: private_input");

    println!("cycle-tracker-report-start: combined_v9_row_relation");
    let public = evaluate(&header, &raw_bayer, MODEL_BLOB)
        .unwrap_or_else(|error| panic!("candidate row relation rejected: {error}"));
    println!("cycle-tracker-report-end: combined_v9_row_relation");

    println!("classification={CLASSIFICATION}");
    println!("row_membership_proved={ROW_MEMBERSHIP_PROVED}");
    println!("session_identifier_in_relation=false");
    println!("chain_log_commitment_in_relation=false");
    println!("cycle-tracker-report-start: public_commitment");
    sp1_zkvm::io::commit_slice(&public);
    println!("cycle-tracker-report-end: public_commitment");
}
