use std::path::PathBuf;

use zeebeam_row_binding_join::evaluate;
use zeebeam_row_binding_membership::verify_and_encode;
use zeebeam_row_binding_membership_native::{
    check_public_values, expected_row96_public_values, row96_header, row96_membership_witness,
    MODEL_BLOB, ROW96_RAW_RELATIVE_PATH,
};

fn bosun_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../..")
}

#[test]
fn real_row96_relation_and_opening_match_the_independent_v2_oracle() {
    let raw = std::fs::read(bosun_root().join(ROW96_RAW_RELATIVE_PATH))
        .expect("frozen row-96 raw capture is absent");
    let base = evaluate(&row96_header(), &raw, MODEL_BLOB).expect("row relation failed");
    let actual = verify_and_encode(&base, &row96_membership_witness())
        .expect("ordered membership opening failed");
    check_public_values(&actual, &expected_row96_public_values())
        .expect("row-96 v2 public values differ");
}

#[test]
fn one_raw_bit_cannot_open_under_the_frozen_session_root() {
    let mut raw = std::fs::read(bosun_root().join(ROW96_RAW_RELATIVE_PATH))
        .expect("frozen row-96 raw capture is absent");
    raw[12_345_678] ^= 1;
    let changed = evaluate(&row96_header(), &raw, MODEL_BLOB).expect("mutated row relation failed");
    assert!(verify_and_encode(&changed, &row96_membership_witness()).is_err());
}

#[test]
fn every_frozen_sibling_is_individually_bound() {
    let base = zeebeam_row_binding_join_native::expected_row96_public_values();
    let witness = row96_membership_witness();
    for index in 0..witness.siblings_blake3.len() {
        let mut changed = witness.clone();
        changed.siblings_blake3[index][31 - index] ^= 1;
        assert!(
            verify_and_encode(&base, &changed).is_err(),
            "sibling {index}"
        );
    }
}

#[test]
fn session_context_and_wrapped_root_are_individually_bound() {
    let base = zeebeam_row_binding_join_native::expected_row96_public_values();
    let witness = row96_membership_witness();

    let mut mutations = Vec::new();
    let mut session = witness.clone();
    session.session_id[0] ^= 1;
    mutations.push(session);
    let mut count = witness.clone();
    count.row_count += 1;
    mutations.push(count);
    let mut s_0 = witness.clone();
    s_0.s_0[0] ^= 1;
    mutations.push(s_0);
    let mut s_n = witness.clone();
    s_n.s_n[0] ^= 1;
    mutations.push(s_n);
    let mut authority = witness.clone();
    authority.authority_manifest_sha256[0] ^= 1;
    mutations.push(authority);
    let mut chain = witness.clone();
    chain.chain_log_blake3[0] ^= 1;
    mutations.push(chain);
    let mut root = witness;
    root.ordered_session_root_blake3[0] ^= 1;
    mutations.push(root);

    for changed in mutations {
        assert!(verify_and_encode(&base, &changed).is_err());
    }
}
