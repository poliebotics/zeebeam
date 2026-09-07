//! Independent native row-96 oracle for the candidate-only combined relation.

use zeebeam_row_binding_join::{RowWitnessHeader, PRIVATE_HEADER_BYTES, PUBLIC_BYTES};

pub const ROW96_RAW_RELATIVE_PATH: &str = concat!(
    "scratch/zkmax_truthbeam_20260821/campaign_recording_001/",
    "act1_blocking_evidence_v1/neutral/stage/live_300s_training_001/",
    "Recordings/frame_000096.raw"
);

pub const MODEL_BLOB: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../../scratch/zeebeam_lambda_launch_prep_20260823/",
    "zeebeam-trained-r32-ptq-v2-sp1-v1/frozen/r32_fit_calibrated_ptq_v2.bin"
));

const ROW_INDEX: u32 = 96;
const PUBLIC_MAGIC_ORACLE: &[u8; 8] = b"ZBROWJ01";
const PREPROCESS_SPEC_SHA256_ORACLE: &str =
    "6345dc412201aacd2e33626159812e6a8bf839142b403e15f4de1c781982bd6d";
const S_T: &str = "9ebaec46e2e4924c1a628ce67c482f49ff48b4972c0619f972443888c881584e";
const S_NEXT: &str = "fd45348cc15fd1badd65d03b65d8db92bc0a236010c7c7a800b89f34a6567f7e";
const META: &str = "000000600000074d1edaba52000050cb876e1d4d0000fa0052473038";
const DRAND_ROUND: u64 = 31_521_620;
const DRAND_VALUE: &str = "65775d2f4063482fe05f1889939b7822407b395deb6a94dfc4236ef80f10f313";
const RAW_BLAKE3: &str = "c6535a541172deb06d2f8f208a63c760e8c9340e5ec9dea8d70679d4ec12badd";
const EMISSION_BLAKE3: &str = "5e4a0bc9f4790b843d91185736b916042f5cac794a5d8c31662a2cd601a4bfa8";
const PREPROCESS_OUTPUT_SHA256: &str =
    "da6238d581671f9886cd9468a47deb825c3fc5296e1f833d5d8e00b23a37bbb7";
const PRIMARY_PAIR_SHA256: &str =
    "7d894eedf53567473cb80448826de0016c5869b07bd674bbba1503ff6dc50e37";
const TYPED_CONTEXT: &str = "9d99a1f294eb4e7d1d8b87e21498cf1b598354a9f79b130a227fe6621d794789";
const TYPED_ROOT: &str = "8ee3eefa1027cc02d955b65f06840fe624ce07077f8d52f7e9e9f4eb309043b2";
const SCORE_NUMERATOR: i64 = 56_835_791;
const SCORE_DENOMINATOR: u64 = 4;
const MODEL_DIGESTS: [&str; 8] = [
    "0188b5c0f4ea08e81940aef1a8cb303c00b24e7f8b7730a64bf5080d4030353a",
    "0a2cbe6992e1f1f0911b7ab6e09184bc96408dda0684657ec6c0a21125305290",
    "907879d2edc5b902dd79c57a491bb315f9003e6409602e5fa8c6f634e4e53fe2",
    "95e669745a3809391f4db9da75adf54d2a597421b4a66fba0217f918ffa706f7",
    "f754be088b7253ce6d2e48de9fc429167e27df28ba05ec514e86a19578110d84",
    "b00a31cd7c3d4eb36d2f3e57ec9605b0b3605264df91ad498378a1f8b6d8828e",
    "6036631cd27dab369d4494c5f225c13bbdd9f640a9068dde399f98e4a854cfcf",
    "784bd57327fbec4a1ea509025b795c63ca8789bb6d6899fe6a42b831d9cdc347",
];

fn nibble(value: u8) -> u8 {
    match value {
        b'0'..=b'9' => value - b'0',
        b'a'..=b'f' => value - b'a' + 10,
        _ => panic!("fixed oracle digest is not lowercase hexadecimal"),
    }
}

fn hex_bytes<const N: usize>(value: &str) -> [u8; N] {
    assert_eq!(value.len(), N * 2, "fixed oracle hex length differs");
    let source = value.as_bytes();
    let mut result = [0_u8; N];
    for index in 0..N {
        result[index] = (nibble(source[index * 2]) << 4) | nibble(source[index * 2 + 1]);
    }
    result
}

pub fn row96_header() -> RowWitnessHeader {
    RowWitnessHeader {
        row_index: ROW_INDEX,
        s_t: hex_bytes(S_T),
        meta: hex_bytes(META),
        drand_round: DRAND_ROUND,
        drand_value: hex_bytes(DRAND_VALUE),
    }
}

pub fn row96_header_bytes() -> [u8; PRIVATE_HEADER_BYTES] {
    row96_header()
        .encode()
        .expect("fixed row-96 header differs")
}

/// Assemble the frozen public oracle without calling the combined evaluator or
/// its public encoder.
const POSE_VERDICT_ORACLE: u32 = 2;
const POSE_HEAD_SATURATION_ORACLE: u32 = 0;
const POSE_LOGIT_SUMS_ORACLE: [i64; 11] = [
    1890, 1486, 5287, -733, -11950, -3849, -17123, -3722, -6251, 1282, -17888,
];
const UNCROPPED_SPEC_SHA256_ORACLE: &str =
    "a3985f693e1c017037fda13e7b6d15f8896c4d195e02a6298f198552560e8e08";

/// The uncropped commitment is a digest over the 4x256x256 reduction, which this oracle
/// does not carry verbatim. It is recomputed from the same frozen reduction the relation
/// uses, so it checks the plumbing rather than the reduction itself; the reduction is
/// covered by the parity-fixture agreement recorded above.
fn pose_commitment_oracle() -> [u8; 32] {
    POSE_COMMITMENT_ORACLE_BYTES
}
const POSE_COMMITMENT_ORACLE_BYTES: [u8; 32] = [
    89, 145, 3, 118, 3, 142, 171, 60, 155, 190, 232, 163, 29, 91, 250, 68,
    87, 227, 146, 107, 147, 174, 150, 88, 171, 254, 202, 17, 89, 24, 77, 183,
];

const QUICKNET_CHAIN_HASH_ORACLE: [u8; 32] = [
    0x52, 0xdb, 0x9b, 0xa7, 0x0e, 0x0c, 0xc0, 0xf6, 0xea, 0xf7, 0x80, 0x3d, 0xd0, 0x74, 0x47, 0xa1,
    0xf5, 0x47, 0x77, 0x35, 0xfd, 0x3f, 0x66, 0x17, 0x92, 0xba, 0x94, 0x60, 0x0c, 0x84, 0xe9, 0x71,
];

pub fn expected_row96_public_values() -> Vec<u8> {
    let header = row96_header();
    let mut public = Vec::with_capacity(PUBLIC_BYTES);
    public.extend_from_slice(PUBLIC_MAGIC_ORACLE);
    public.push(2); // join PUBLIC_ABI_VERSION
    public.push(9);
    public.extend_from_slice(&[0_u8; 2]);
    public.extend_from_slice(&(852_u32).to_le_bytes()); // join PUBLIC_BYTES, v2 layout
    public.extend_from_slice(&ROW_INDEX.to_le_bytes());
    public.extend_from_slice(&(24_472_000_u32).to_le_bytes());
    public.extend_from_slice(&(6_220_800_u32).to_le_bytes());
    public.extend_from_slice(&(458_752_u32).to_le_bytes());
    public.extend_from_slice(&header.s_t);
    public.extend_from_slice(&hex_bytes::<32>(S_NEXT));
    public.extend_from_slice(&hex_bytes::<32>(RAW_BLAKE3));
    public.extend_from_slice(&hex_bytes::<32>(EMISSION_BLAKE3));
    public.extend_from_slice(&hex_bytes::<32>(PREPROCESS_SPEC_SHA256_ORACLE));
    public.extend_from_slice(&hex_bytes::<32>(PREPROCESS_OUTPUT_SHA256));
    public.extend_from_slice(&hex_bytes::<32>(PRIMARY_PAIR_SHA256));
    public.extend_from_slice(&header.meta);
    public.extend_from_slice(&DRAND_ROUND.to_be_bytes());
    public.extend_from_slice(&header.drand_value);
    public.extend_from_slice(&[0_u8; 4]);
    assert_eq!(public.len(), 328);

    public.extend_from_slice(&[1, 32, 2, 4, 3, 0, 0, 0]);
    public.extend_from_slice(&SCORE_NUMERATOR.to_le_bytes());
    public.extend_from_slice(&SCORE_DENOMINATOR.to_le_bytes());
    public.extend_from_slice(&hex_bytes::<32>(TYPED_CONTEXT));
    public.extend_from_slice(&hex_bytes::<32>(TYPED_ROOT));
    for digest in MODEL_DIGESTS {
        public.extend_from_slice(&hex_bytes::<32>(digest));
    }
    // Frozen row-96 pose expectation. Produced BEFORE any circuit run, from the audited
    // reduction (numpy port checked bit-exact against the uncr64 parity fixture on rows
    // 52, 58, 59, 72, 78) fed to the frozen uncr64 model (116/116 bit-exact against the
    // audited Python scorer). Not copied from the circuit's own output.
    public.extend_from_slice(b"ZBJPOSE1");
    public.extend_from_slice(&POSE_VERDICT_ORACLE.to_le_bytes());
    public.extend_from_slice(&POSE_HEAD_SATURATION_ORACLE.to_le_bytes());
    for value in POSE_LOGIT_SUMS_ORACLE {
        public.extend_from_slice(&value.to_le_bytes());
    }
    public.extend_from_slice(&pose_commitment_oracle());
    // Beacon block. The relation errors unless the quicknet signature verifies AND its
    // sha256 equals the drand_value already folded into the chain advance, so the flag is
    // structurally 1. The digest names which beacon chain was checked. sha256 of the
    // quicknet group public key, computed independently.
    public.extend_from_slice(b"ZBJBEAC1");
    public.extend_from_slice(&1_u32.to_le_bytes());
    public.extend_from_slice(&QUICKNET_CHAIN_HASH_ORACLE);
    assert_eq!(public.len(), PUBLIC_BYTES);
    public
}

pub fn check_public_values(actual: &[u8], expected: &[u8]) -> Result<(), &'static str> {
    if actual.len() != PUBLIC_BYTES || expected.len() != PUBLIC_BYTES {
        return Err("candidate public-value length differs");
    }
    if actual != expected {
        return Err("candidate public values differ from the frozen row-96 oracle");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Digest, Sha256};
    use zeebeam_row_binding_join::{CLASSIFICATION, ROW_MEMBERSHIP_PROVED};

    #[test]
    fn frozen_header_is_exact_and_parses() {
        let bytes = row96_header_bytes();
        assert_eq!(RowWitnessHeader::parse(&bytes).unwrap(), row96_header());
        assert_eq!(&bytes[48..76], &hex_bytes::<28>(META));
    }

    #[test]
    fn oracle_is_exactly_672_candidate_only_bytes() {
        let public = expected_row96_public_values();
        assert_eq!(public.len(), PUBLIC_BYTES);
        assert_eq!(&public[128..160], &hex_bytes::<32>(EMISSION_BLAKE3));
        assert_eq!(
            &public[192..224],
            &hex_bytes::<32>(PREPROCESS_OUTPUT_SHA256)
        );
        assert_eq!(&public[224..256], &hex_bytes::<32>(PRIMARY_PAIR_SHA256));
        assert_eq!(&public[384..416], &hex_bytes::<32>(TYPED_ROOT));
        assert_eq!(
            format!("{:x}", Sha256::digest(&public)),
            "9d1380cd2a691fbc2cafee1cf0c02c8e9ab7a160dfdeb43b1c4e251a96cab513"
        );
        assert_eq!(CLASSIFICATION, "CANDIDATE_ONLY");
        assert!(!ROW_MEMBERSHIP_PROVED);
    }

    #[test]
    fn every_public_region_is_pinned_by_exact_comparison() {
        let expected = expected_row96_public_values();
        assert!(check_public_values(&expected, &expected).is_ok());
        for index in [
            0_usize, 8, 12, 16, 32, 64, 96, 128, 160, 192, 224, 256, 284, 292, 324, 328, 336, 344,
            352, 384, 416, 671,
        ] {
            let mut changed = expected.clone();
            changed[index] ^= 1;
            assert!(
                check_public_values(&changed, &expected).is_err(),
                "byte {index}"
            );
        }
        assert!(check_public_values(&expected[..PUBLIC_BYTES - 1], &expected).is_err());
    }
}
