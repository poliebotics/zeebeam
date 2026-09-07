//! Shared witness construction and oracle checks for every host binary (execute, ceremony).
//!
//! Default row is 96 against the frozen full oracle. With `ROW_JSON=<path>` (written by
//! `session_tree.py`) any row of the session is proved: header and Merkle siblings come from the
//! file, and the statement is checked against a PARTIAL oracle: every row-independent region
//! byte-exact against the frozen row-96 expectations, the pose verdict and logit sums against the
//! audited parity vectors carried in the file, the remaining row-specific bytes reported as
//! circuit-derived.
//!
//! Environment overrides (negative controls and the ceremony):
//!   TX, FIX, IVK, IVK_FLIP, PREFIX, PREFIX_JSON, RECEIPT, OPENING_LE_HEX, TRAPDOOR_LE_HEX,
//!   EXPECT_TRAPDOOR_FLAG.

use sp1_sdk::SP1Stdin;
use zeebeam_row_binding_join::RowWitnessHeader;
use zeebeam_row_binding_membership::PUBLIC_PREFIX_BYTES;
use zeebeam_row_binding_membership_native::{
    expected_row96_total_public_values, row96_header_bytes, row96_membership_witness,
    row96_membership_witness_bytes, TOTAL_PUBLIC_BYTES,
};

pub const AUG: &str = "/home/c/Documents/BOSUN/scratch/august_leg_20260901";
const ROW96_SIGNATURE_HEX: &str =
    "86efb9051b5f44e9c5e8e8e6ed30eff6a3ed1a3e11c91e42f6da0fa505db5e51385bda5892ae44ae40a5783a147d3322";

pub fn env_or(key: &str, default: String) -> String {
    std::env::var(key).unwrap_or(default)
}

pub fn unhex(s: &str) -> Vec<u8> {
    let s = s.trim();
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).expect("hex"))
        .collect()
}

pub fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

fn hex_arr(s: &str) -> [u8; 32] {
    arr32(s)
}

fn arr32(s: &str) -> [u8; 32] {
    unhex(s).try_into().expect("32-byte hex")
}

/// A little-endian 32-byte scalar from a hex string of any even length up to 64 characters.
fn scalar_le_32(hex: &str) -> Vec<u8> {
    let mut v = unhex(hex);
    assert!(v.len() <= 32, "scalar hex longer than 32 bytes");
    v.resize(32, 0);
    v
}

/// Everything the guest reads, plus what the host expects back.
pub struct Built {
    pub stdin: SP1Stdin,
    pub row: u32,
    /// Frozen full expectation for row 96 (also the source of the row-independent regions).
    pub frozen96: Vec<u8>,
    pub row_json: Option<serde_json::Value>,
    pub trapdoor_flag: u8,
}

pub fn build(raw: Vec<u8>) -> Built {
    let trapdoor_flag: u8 = env_or("EXPECT_TRAPDOOR_FLAG", "0".into()).parse().expect("flag");
    let frozen96 = expected_row96_total_public_values(trapdoor_flag);
    assert_eq!(frozen96.len(), TOTAL_PUBLIC_BYTES);

    let (header, membership, signature, row_json, row) = match std::env::var("ROW_JSON") {
        Err(_) => (
            row96_header_bytes().to_vec(),
            row96_membership_witness_bytes(),
            unhex(ROW96_SIGNATURE_HEX),
            None,
            96u32,
        ),
        Ok(path) => {
            let j: serde_json::Value =
                serde_json::from_str(&std::fs::read_to_string(&path).expect("ROW_JSON file")).unwrap();
            let row = j["row"].as_u64().expect("row") as u32;
            let header = RowWitnessHeader {
                row_index: row,
                s_t: arr32(j["s_t"].as_str().unwrap()),
                meta: unhex(j["meta"].as_str().unwrap()).try_into().expect("28-byte meta"),
                drand_round: j["drand_round"].as_u64().expect("round"),
                drand_value: arr32(j["drand_value"].as_str().unwrap()),
            }
            .encode()
            .expect("row header encodes");
            let mut witness = row96_membership_witness();
            witness.siblings_blake3 = j["siblings"]
                .as_array()
                .expect("siblings")
                .iter()
                .map(|s| arr32(s.as_str().unwrap()))
                .collect();
            assert_eq!(witness.siblings_blake3.len(), 10, "ten siblings expected");
            let signature = unhex(j["drand_signature"].as_str().unwrap());
            (header.to_vec(), witness.encode().expect("membership witness encodes"), signature, Some(j), row)
        }
    };

    // ---- the real August anchor: transaction bytes, merkle branch, block header ----
    let anchor_tx = unhex(&std::fs::read_to_string(env_or("TX", format!("{AUG}/anchor_tx.hex"))).expect("anchor tx hex"));
    let fixture: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(env_or("FIX", format!("{AUG}/merkle_fixture_3456294.json"))).expect("fixture"),
    )
    .unwrap();
    let anchor_header = unhex(fixture["header_hex"].as_str().unwrap());
    let mut anchor_branch: Vec<u8> = Vec::new();
    for level in fixture["branch"].as_array().unwrap() {
        anchor_branch.extend(unhex(level["hash"].as_str().unwrap()));
        anchor_branch.push(level["leaf_is_right"].as_u64().expect("flag") as u8);
    }
    // ---- the holder's external Orchard incoming viewing key (private witness) ----
    let mut holder_ivk = std::fs::read(env_or("IVK", format!("{AUG}/holder_ufvk_extract/holder_ivk_external.bin"))).expect("ivk file");
    if let Ok(flip) = std::env::var("IVK_FLIP") {
        let i: usize = flip.parse().expect("IVK_FLIP index");
        holder_ivk[i] ^= 1;
    }
    // ---- chameleon witnesses: opening from the ceremony record, trapdoor only at proving time ----
    let opening = scalar_le_32(&env_or("OPENING_LE_HEX", "01".into()));
    let trapdoor = scalar_le_32(&env_or("TRAPDOOR_LE_HEX", "00".into()));

    let mut stdin = SP1Stdin::new();
    stdin.write_vec(header);
    stdin.write_vec(membership);
    stdin.write_vec(raw);
    stdin.write_vec(signature);
    stdin.write_vec(anchor_tx);
    stdin.write_vec(anchor_branch);
    stdin.write_vec(anchor_header);
    stdin.write_vec(holder_ivk);
    stdin.write_vec(std::fs::read(env_or("PREFIX", format!("{AUG}/chain_prefix_260.bin"))).expect("chain prefix"));
    stdin.write_vec(std::fs::read(env_or("PREFIX_JSON", format!("{AUG}/prefix_json.bin"))).expect("prefix json"));
    stdin.write_vec(std::fs::read(env_or("RECEIPT", format!("{AUG}/binding_core_canonical.bin"))).expect("binding receipt"));
    stdin.write_vec(opening);
    stdin.write_vec(trapdoor);
    Built { stdin, row, frozen96, row_json, trapdoor_flag }
}

pub struct OracleReport {
    pub mode: String,
    pub bytes_checked: usize,
    pub bytes_circuit_derived: usize,
    pub notes: Vec<String>,
}

/// Check the committed public values against the oracle the row admits. Err on any mismatch.
pub fn check(actual: &[u8], built: &Built) -> Result<OracleReport, String> {
    if actual.len() != TOTAL_PUBLIC_BYTES {
        return Err(format!("public value length {} differs from {TOTAL_PUBLIC_BYTES}", actual.len()));
    }
    match &built.row_json {
        None => {
            if actual != built.frozen96.as_slice() {
                let i = actual.iter().zip(built.frozen96.iter()).position(|(a, b)| a != b).unwrap_or(0);
                let s = i.saturating_sub(8);
                let e = (i + 24).min(actual.len());
                return Err(format!(
                    "public values differ from the frozen row-96 oracle at byte {i}: actual[{s}..{e}]={} expected={}",
                    hex(&actual[s..e]),
                    hex(&built.frozen96[s..e])
                ));
            }
            Ok(OracleReport {
                mode: "full_frozen_row96".into(),
                bytes_checked: TOTAL_PUBLIC_BYTES,
                bytes_circuit_derived: 0,
                notes: vec![],
            })
        }
        Some(j) => {
            let frozen = &built.frozen96;
            let mut mism = Vec::new();
            let mut checked = 0usize;
            let mut derived = 0usize;
            let row = built.row;
            if actual[16..20] != row.to_le_bytes() {
                mism.push(format!("published row index differs from {row}"));
            }
            if actual[..16] != frozen[..16] || actual[20..PUBLIC_PREFIX_BYTES] != frozen[20..PUBLIC_PREFIX_BYTES] {
                mism.push("membership prefix differs from the frozen session prefix outside the row index".into());
            }
            checked += PUBLIC_PREFIX_BYTES;
            // scorer block: header, score, denominator, typed context, typed root, eight model digests.
            // Header, denominator and digests are row-independent; context, root and the pose
            // commitment come from the independent Python oracle (row_oracle.py); only the score
            // numerator and the head saturation remain circuit-derived.
            let scorer = &actual[PUBLIC_PREFIX_BYTES..PUBLIC_PREFIX_BYTES + 344];
            let scorer96 = &frozen[PUBLIC_PREFIX_BYTES..PUBLIC_PREFIX_BYTES + 344];
            if scorer[..8] != scorer96[..8] || scorer[16..24] != scorer96[16..24] || scorer[88..] != scorer96[88..] {
                mism.push("coupling block constants differ from the frozen row-96 block".into());
            }
            let score = i64::from_le_bytes(scorer[8..16].try_into().unwrap());
            let exp_ctx = hex_arr(j["expected_typed_context"].as_str().expect("expected_typed_context"));
            let exp_root = hex_arr(j["expected_typed_root"].as_str().expect("expected_typed_root"));
            if scorer[24..56] != exp_ctx {
                mism.push("typed context differs from the Python oracle".into());
            }
            if scorer[56..88] != exp_root {
                mism.push("typed root differs from the Python oracle".into());
            }
            checked += 8 + 8 + 64 + 256;
            match j.get("expected_coupling_score_numerator").and_then(|v| v.as_i64()) {
                Some(exp) => {
                    if score != exp {
                        mism.push(format!("coupling score numerator {score} differs from the Python re-run {exp}"));
                    }
                    checked += 8;
                }
                None => derived += 8, // coupling score numerator, no independent value supplied
            }
            let scorer_end = PUBLIC_PREFIX_BYTES + 344;
            let pose = &actual[scorer_end..scorer_end + 136];
            if &pose[..8] != b"ZBJPOSE1" {
                mism.push("pose magic differs".into());
            }
            let verdict = u32::from_le_bytes(pose[8..12].try_into().unwrap());
            let head_saturation = u32::from_le_bytes(pose[12..16].try_into().unwrap());
            let sums: Vec<i64> = (0..11).map(|k| i64::from_le_bytes(pose[16 + 8 * k..24 + 8 * k].try_into().unwrap())).collect();
            let exp_verdict = j["expected_pose_verdict"].as_u64().expect("expected verdict") as u32;
            let exp_sums: Vec<i64> = j["expected_pose_logit_sums"].as_array().unwrap().iter().map(|v| v.as_i64().unwrap()).collect();
            if verdict != exp_verdict {
                mism.push(format!("pose verdict {verdict} differs from the audited parity verdict {exp_verdict}"));
            }
            if sums != exp_sums {
                mism.push(format!("pose logit sums {sums:?} differ from the audited parity sums {exp_sums:?}"));
            }
            let exp_commit = hex_arr(j["expected_uncropped_commitment"].as_str().expect("expected_uncropped_commitment"));
            if pose[104..136] != exp_commit {
                mism.push("uncropped pose commitment differs from the Python oracle".into());
            }
            checked += 8 + 4 + 88 + 32;
            match j.get("expected_pose_head_saturation").and_then(|v| v.as_u64()) {
                Some(exp) => {
                    if head_saturation as u64 != exp {
                        mism.push(format!("pose head saturation {head_saturation} differs from the Python re-run {exp}"));
                    }
                    checked += 4;
                }
                None => derived += 4, // head saturation, no independent value supplied
            }
            let tail_start = scorer_end + 136;
            // the august block's two round fields (16 bytes before the final flag) are row-specific
            let prev_at = actual.len() - 17;
            if actual[tail_start..prev_at] != frozen[tail_start..prev_at] || actual[prev_at + 16..] != frozen[prev_at + 16..] {
                mism.push("beacon/zcash/august tail differs from the frozen expectations".into());
            }
            let prev_round = u64::from_be_bytes(actual[prev_at..prev_at + 8].try_into().unwrap());
            let own_round = u64::from_be_bytes(actual[prev_at + 8..prev_at + 16].try_into().unwrap());
            let exp_prev = j["expected_previous_drand_round"].as_u64().expect("expected_previous_drand_round");
            let exp_own = j["drand_round"].as_u64().expect("drand_round");
            if prev_round != exp_prev {
                mism.push(format!("published previous round {prev_round} differs from the chain log's {exp_prev}"));
            }
            if own_round != exp_own {
                mism.push(format!("published row round {own_round} differs from the chain log's {exp_own}"));
            }
            checked += actual.len() - tail_start;
            if !mism.is_empty() {
                return Err(mism.join("; "));
            }
            Ok(OracleReport {
                mode: if derived == 0 { format!("independent_row_{row}") } else { format!("partial_row_{row}") },
                bytes_checked: checked,
                bytes_circuit_derived: derived,
                notes: vec![
                    format!("pose verdict {verdict}, head saturation {head_saturation}, logit sums {sums:?} match the audited parity vectors"),
                    format!("typed context, typed root and uncropped commitment match row_oracle.py; coupling score numerator {score} (denominator 4) is circuit-derived"),
                ],
            })
        }
    }
}
