use std::path::PathBuf;

use zeebeam_row_binding_join::{evaluate, primary_pair_sha256};
use zeebeam_row_binding_join_native::{
    check_public_values, expected_row96_public_values, row96_header, MODEL_BLOB,
    ROW96_RAW_RELATIVE_PATH,
};

fn bosun_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../..")
}

#[test]
fn parity_vector_tail_has_the_frozen_row96_primary_pair_digest() {
    const CAMERA_BYTES: usize = 4 * 256 * 256;
    const EMISSION_BYTES: usize = 3 * 256 * 256;
    const TAIL_OFFSET: usize = 517_548;
    let path = bosun_root().join(
        "scratch/zeebeam_lambda_launch_prep_20260823/zeebeam-trained-r32-ptq-v2-sp1-v1/fixtures/architecture_r32_parity_v2.bin",
    );
    let vector = std::fs::read(path).expect("frozen parity vector is absent");
    assert_eq!(vector.len(), TAIL_OFFSET + CAMERA_BYTES + EMISSION_BYTES);
    let camera = &vector[TAIL_OFFSET..TAIL_OFFSET + CAMERA_BYTES];
    let emission = &vector[TAIL_OFFSET + CAMERA_BYTES..];
    assert_eq!(
        primary_pair_sha256(96, camera, emission).unwrap(),
        expected_row96_public_values()[224..256]
    );
}

#[test]
fn real_row96_native_relation_matches_the_independent_public_oracle() {
    let raw_path = bosun_root().join(ROW96_RAW_RELATIVE_PATH);
    let raw = std::fs::read(raw_path).expect("frozen row-96 raw capture is absent");
    let actual = evaluate(&row96_header(), &raw, MODEL_BLOB).expect("row-96 relation failed");
    let expected = expected_row96_public_values();
    check_public_values(&actual, &expected).expect("row-96 public values differ");
}

#[test]
fn one_raw_bit_cannot_match_the_frozen_public_oracle() {
    let raw_path = bosun_root().join(ROW96_RAW_RELATIVE_PATH);
    let mut raw = std::fs::read(raw_path).expect("frozen row-96 raw capture is absent");
    raw[12_345_678] ^= 1;
    let changed = evaluate(&row96_header(), &raw, MODEL_BLOB).expect("mutated relation failed");
    assert!(check_public_values(&changed, &expected_row96_public_values()).is_err());
}

#[test]
fn state_transition_evidence_mutations_cannot_match_the_frozen_oracle() {
    let raw_path = bosun_root().join(ROW96_RAW_RELATIVE_PATH);
    let raw = std::fs::read(raw_path).expect("frozen row-96 raw capture is absent");
    let expected = expected_row96_public_values();

    let mut headers = Vec::new();
    let mut wrong_state = row96_header();
    wrong_state.s_t[0] ^= 1;
    headers.push(wrong_state);
    let mut wrong_meta = row96_header();
    wrong_meta.meta[4] ^= 1;
    headers.push(wrong_meta);
    let mut wrong_round = row96_header();
    wrong_round.drand_round += 1;
    headers.push(wrong_round);
    let mut wrong_value = row96_header();
    wrong_value.drand_value[31] ^= 1;
    headers.push(wrong_value);

    for header in headers {
        let changed = evaluate(&header, &raw, MODEL_BLOB).expect("mutated relation failed");
        assert!(check_public_values(&changed, &expected).is_err());
    }
}

#[test]
fn changed_model_blob_is_rejected_before_rendering() {
    let raw_path = bosun_root().join(ROW96_RAW_RELATIVE_PATH);
    let raw = std::fs::read(raw_path).expect("frozen row-96 raw capture is absent");
    let mut model = MODEL_BLOB.to_vec();
    model[100] ^= 1;
    let error = evaluate(&row96_header(), &raw, &model).unwrap_err();
    assert_eq!(
        error.message(),
        "embedded trained-r32 model blob SHA-256 differs"
    );
}
