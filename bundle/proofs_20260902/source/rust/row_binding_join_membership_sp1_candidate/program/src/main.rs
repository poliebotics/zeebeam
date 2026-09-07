//! SP1 guest for the v9 row binding plus ordered-session membership relation, extended with the
//! August capture-time anchor: shielded inclusion, memo decryption, prefix join, chameleon
//! opening and trapdoor knowledge, all in one statement.

#![no_main]
sp1_zkvm::entrypoint!(main);

use zeebeam_row_binding_join::{evaluate, RowWitnessHeader};
use zeebeam_row_binding_membership::{
    august, memo, verify_and_encode, zcash, MembershipWitness,
    AUTHORITY_MANIFEST_DIGEST_IN_CONTEXT, CHAIN_LOG_DIGEST_IN_CONTEXT, CLASSIFICATION,
    ROW_MEMBERSHIP_RELATION_ENFORCED, SESSION_IDENTIFIER_IN_RELATION,
};

const MODEL_BLOB: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../../scratch/zeebeam_lambda_launch_prep_20260823/",
    "zeebeam-trained-r32-ptq-v2-sp1-v1/frozen/r32_fit_calibrated_ptq_v2.bin"
));

/// The relation reads the memo of this Ironwood action of the anchor transaction.
const ANCHOR_ACTION_INDEX: usize = 0;

pub fn main() {
    println!("cycle-tracker-report-start: private_input");
    let header_bytes = sp1_zkvm::io::read_vec();
    let membership_bytes = sp1_zkvm::io::read_vec();
    let raw_bayer = sp1_zkvm::io::read_vec();
    let drand_signature = sp1_zkvm::io::read_vec();
    let anchor_tx = sp1_zkvm::io::read_vec();
    let anchor_branch = sp1_zkvm::io::read_vec();
    let anchor_header = sp1_zkvm::io::read_vec();
    let holder_ivk = sp1_zkvm::io::read_vec();
    let chain_prefix = sp1_zkvm::io::read_vec();
    let prefix_json = sp1_zkvm::io::read_vec();
    let aug_receipt = sp1_zkvm::io::read_vec();
    let aug_opening = sp1_zkvm::io::read_vec();
    let aug_trapdoor = sp1_zkvm::io::read_vec();
    let header = RowWitnessHeader::parse(&header_bytes)
        .unwrap_or_else(|error| panic!("private row header rejected: {error}"));
    let membership = MembershipWitness::parse(&membership_bytes)
        .unwrap_or_else(|error| panic!("private membership witness rejected: {error}"));
    println!("cycle-tracker-report-end: private_input");

    println!("cycle-tracker-report-start: combined_v9_row_relation");
    let base_public = evaluate(&header, &raw_bayer, MODEL_BLOB, &drand_signature)
        .unwrap_or_else(|error| panic!("candidate row relation rejected: {error}"));
    println!("cycle-tracker-report-end: combined_v9_row_relation");

    println!("cycle-tracker-report-start: ordered_session_membership");
    let public = verify_and_encode(&base_public, &membership)
        .unwrap_or_else(|error| panic!("ordered session membership rejected: {error}"));
    println!("cycle-tracker-report-end: ordered_session_membership");

    println!("classification={CLASSIFICATION}");
    println!("row_membership_relation_enforced={ROW_MEMBERSHIP_RELATION_ENFORCED}");
    println!("session_identifier_in_relation={SESSION_IDENTIFIER_IN_RELATION}");
    println!("authority_manifest_digest_in_context={AUTHORITY_MANIFEST_DIGEST_IN_CONTEXT}");
    println!("chain_log_digest_in_context={CHAIN_LOG_DIGEST_IN_CONTEXT}");

    // ---- the previous row's beacon: the one that bounds THIS frame's emission from below ----
    println!("cycle-tracker-report-start: previous_advance_leg");
    let previous = {
        let row_index = u32::from_le_bytes(base_public[16..20].try_into().unwrap());
        let s_t: [u8; 32] = base_public[32..64].try_into().unwrap();
        august::verify_previous_advance(&chain_prefix, row_index, &s_t)
            .unwrap_or_else(|e| panic!("previous-row advance rejected: {e}"))
    };
    println!("previous_drand_round={}", previous.previous_round);
    println!("cycle-tracker-report-end: previous_advance_leg");

    // ---- the August anchor transaction: ZIP-244 txid, merkle inclusion, header hash ----
    println!("cycle-tracker-report-start: zcash_anchor_leg");
    let inclusion = zcash::verify_anchor_inclusion(&anchor_tx, &anchor_branch, &anchor_header)
        .unwrap_or_else(|error| panic!("zcash anchor inclusion rejected: {error}"));
    let mut full = public;
    full.extend_from_slice(zcash::ZCASH_PUBLIC_MAGIC);
    full.extend_from_slice(&inclusion.txid);
    full.extend_from_slice(&inclusion.header_hash);
    println!("cycle-tracker-report-end: zcash_anchor_leg");

    // ---- the memo of that same transaction, decrypted under the holder's viewing key ----
    println!("cycle-tracker-report-start: memo_leg");
    let action = inclusion
        .ironwood_actions
        .get(ANCHOR_ACTION_INDEX)
        .unwrap_or_else(|| panic!("anchor transaction has no action {ANCHOR_ACTION_INDEX}"));
    let anchored = memo::decrypt_binding(action, &holder_ivk)
        .unwrap_or_else(|error| panic!("memo leg rejected: {error}"));
    println!("anchor_note_value_zat={}", anchored.value);
    println!("cycle-tracker-report-end: memo_leg");

    // ---- prefix join, receipt, chameleon opening, trapdoor: all chained to the memo binding ----
    println!("cycle-tracker-report-start: august_anchor_leg");
    {
        // The relation's own verified values, taken from the base journal it just produced,
        // hex-encoded for comparison with the anchored chain-log prefix. Not constants.
        fn hexs(b: &[u8]) -> String {
            let mut s = String::new();
            for x in b {
                s.push(char::from_digit((x >> 4) as u32, 16).unwrap());
                s.push(char::from_digit((x & 15) as u32, 16).unwrap());
            }
            s
        }
        let s_t_hex = hexs(&base_public[32..64]);
        let bayer_hex = hexs(&base_public[96..128]);
        let round = u64::from_be_bytes(base_public[284..292].try_into().unwrap());
        let value_hex = hexs(&base_public[292..324]);
        let row_index = u32::from_le_bytes(base_public[16..20].try_into().unwrap());
        let round_ascii = format!("{round}");
        let row_tag = format!("{row_index},");
        let facts = august::RowFacts {
            row_tag: row_tag.as_bytes(),
            s_t_hex: s_t_hex.as_bytes(),
            bayer_hex: bayer_hex.as_bytes(),
            round_ascii: round_ascii.as_bytes(),
            value_hex: value_hex.as_bytes(),
        };
        let anchor = august::verify_august_anchor(
            &chain_prefix,
            &prefix_json,
            &aug_receipt,
            &anchored.binding,
            &aug_opening,
            &aug_trapdoor,
            &facts,
        )
        .unwrap_or_else(|e| panic!("august anchor rejected: {e}"));
        full.extend_from_slice(august::AUGUST_PUBLIC_MAGIC);
        full.extend_from_slice(&anchor.content_root);
        full.extend_from_slice(&anchor.receipt_digest);
        full.extend_from_slice(&anchor.commitment_x);
        full.extend_from_slice(&anchor.commitment_y);
        full.extend_from_slice(&previous.previous_round.to_be_bytes());
        full.extend_from_slice(&round.to_be_bytes()); // the row's own verified round, bounding S_{t+1}
        full.push(if anchor.trapdoor_proved { 1 } else { 0 });
    }
    println!("cycle-tracker-report-end: august_anchor_leg");

    println!("cycle-tracker-report-start: public_commitment");
    sp1_zkvm::io::commit_slice(&full);
    println!("cycle-tracker-report-end: public_commitment");
}
