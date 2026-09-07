//! Independent row-96 oracle for the ordered-membership successor relation.

use zeebeam_row_binding_join as row_join;
use zeebeam_row_binding_membership::{
    MembershipWitness, PUBLIC_ABI_VERSION, PUBLIC_BYTES, PUBLIC_MAGIC, PUBLIC_PREFIX_BYTES,
};

pub use zeebeam_row_binding_join_native::{
    row96_header, row96_header_bytes, MODEL_BLOB, ROW96_RAW_RELATIVE_PATH,
};

pub const SESSION_ID: &[u8] = b"ZEEBEAM_MAINNET_BLOCKING_TRAINING_300S_20260822_001";
pub const ROW_COUNT: u32 = 712;
pub const TREE_DEPTH: u16 = 10;

const S_0: &str = "74e3a131e1aaadd98e18c5f2a5f28ffaf59b8cd738e10216586efa2a893f384c";
const S_N: &str = "aeea9f4d6a55ebecd900eae187ea70dce05696551f96ae976c9a5243b3a4398e";
const AUTHORITY_MANIFEST_SHA256: &str =
    "740d752d27b70cb63c9501a470562a52616f4a445a7a9c0fb300844f61c7d783";
const CHAIN_LOG_BLAKE3: &str = "754e5716e65a065e5ad3146131609a1fd12e8310d966fb6bf2d3de6c568e7f8b";
pub const CONTEXT_DIGEST_BLAKE3: &str =
    "f5eba65f4a3604bee08207b4af571b97813cbd4dd3f919c110f6c5125ab21ee0";
pub const ROW_LEAF_BLAKE3: &str =
    "2e4e76aa9bfece295ce728ad12e37d1cc40fddfd190fa6012116cb88969f4f8b";
pub const INTERNAL_ROOT_BLAKE3: &str =
    "1d43309ef9a0e2fc6dba98846134280b98cda01840d2204dc41100d5038d3b37";
pub const ORDERED_SESSION_ROOT_BLAKE3: &str =
    "38a484b8793f3afadaf8fc5ba1c04d47e08cdbb4c07a39da09fb4149499a6572";

const SIBLINGS: [&str; 10] = [
    "5044e45b95bb454f818d7c6053cd017c88eb63167e00553ffca90fc4db105ea7",
    "dbf38b9f009639960bb19bb34a49723b86f8dab1124e9957838c53ded27e5f14",
    "e8b97d9ae3e24d60685b5c48137e35173b22951e8338cc785470555bc110025f",
    "25d45e7302f4b9c1785ca604a4ac9d819a920eb76de51e947d2431dd8ee65913",
    "62be39eb75e5d08eb05003a92da6c99949ada732bee04fc1d23963562320747c",
    "881655aed30915bf6e5779e36c284b96ab57e679c935b067d441b6080a4b9cf8",
    "9cf816f0e5cec6347a00959af7a661b7ed783fea126aa75479d5168826683f86",
    "22e5c272ed13d3b3dede9638ea3f8519fbd9a3e90c22014c9fe0c2089e64d5d0",
    "583b3de49e15a665d6aeeaccc7af17165a58fc75a42dea3f6da873f321dadf59",
    "626eacc9816d13823066712b6b8953580e0383460da0ffa0b4a5775501872240",
];

fn nibble(value: u8) -> u8 {
    match value {
        b'0'..=b'9' => value - b'0',
        b'a'..=b'f' => value - b'a' + 10,
        _ => panic!("fixed oracle digest is not lowercase hexadecimal"),
    }
}

pub fn hex_bytes<const N: usize>(value: &str) -> [u8; N] {
    assert_eq!(value.len(), N * 2, "fixed oracle hex length differs");
    let source = value.as_bytes();
    let mut result = [0_u8; N];
    for index in 0..N {
        result[index] = (nibble(source[index * 2]) << 4) | nibble(source[index * 2 + 1]);
    }
    result
}

pub fn row96_membership_witness() -> MembershipWitness {
    MembershipWitness {
        protocol_code: row_join::PROTOCOL_V9_CODE,
        terminal_committed: 1,
        tree_depth: TREE_DEPTH,
        row_count: ROW_COUNT,
        session_id: SESSION_ID.to_vec(),
        s_0: hex_bytes(S_0),
        s_n: hex_bytes(S_N),
        authority_manifest_sha256: hex_bytes(AUTHORITY_MANIFEST_SHA256),
        chain_log_blake3: hex_bytes(CHAIN_LOG_BLAKE3),
        ordered_session_root_blake3: hex_bytes(ORDERED_SESSION_ROOT_BLAKE3),
        siblings_blake3: SIBLINGS.iter().map(|value| hex_bytes(value)).collect(),
    }
}

pub fn row96_membership_witness_bytes() -> Vec<u8> {
    row96_membership_witness()
        .encode()
        .expect("fixed row-96 membership witness differs")
}

/// Assemble the frozen v3 public oracle without calling the membership encoder.
pub fn expected_row96_public_values() -> Vec<u8> {
    let base = zeebeam_row_binding_join_native::expected_row96_public_values();
    let mut public = Vec::with_capacity(PUBLIC_BYTES);
    public.extend_from_slice(PUBLIC_MAGIC);
    public.push(PUBLIC_ABI_VERSION);
    public.push(row_join::PROTOCOL_V9_CODE);
    public.push(1);
    public.push(0);
    public.extend_from_slice(&(PUBLIC_BYTES as u32).to_le_bytes());
    public.extend_from_slice(&96_u32.to_le_bytes());
    public.extend_from_slice(&ROW_COUNT.to_le_bytes());
    public.extend_from_slice(&TREE_DEPTH.to_le_bytes());
    public.extend_from_slice(&(SESSION_ID.len() as u16).to_le_bytes());
    public.extend_from_slice(SESSION_ID);
    public.resize(156, 0);
    public.extend_from_slice(&hex_bytes::<32>(S_0));
    public.extend_from_slice(&hex_bytes::<32>(S_N));
    public.extend_from_slice(&hex_bytes::<32>(AUTHORITY_MANIFEST_SHA256));
    public.extend_from_slice(&hex_bytes::<32>(CHAIN_LOG_BLAKE3));
    public.extend_from_slice(&hex_bytes::<32>(CONTEXT_DIGEST_BLAKE3));
    public.extend_from_slice(&hex_bytes::<32>(ORDERED_SESSION_ROOT_BLAKE3));
    public.extend_from_slice(&[0_u8; 4]);
    assert_eq!(public.len(), PUBLIC_PREFIX_BYTES);
    public.extend_from_slice(&base[row_join::PUBLIC_PREFIX_BYTES..]);
    assert_eq!(public.len(), PUBLIC_BYTES);
    public
}

/// August anchor identifiers, internal (little-endian) byte order, taken from the block and
/// transaction records (raw_block_3456294.json, raw_anchor_tx.json, merkle_fixture_3456294.json):
/// txid 8d1672155e98000498070bd76479e0363b8b9ee0b61f0d32a802d35ef5a4f206 in block 3456294 whose
/// hash is 000000000052f2b0c7e7929e16086c3d15f09922b050109ec722c8576ffddf0b. NOT from the circuit.
const ANCHOR_TXID_ORACLE: [u8; 32] = hex_bytes_const(
    "06f2a4f55ed302a8320d1fb6e09e8b3b36e07964d70b07980400985e1572168d");
const ANCHOR_HEADER_HASH_ORACLE: [u8; 32] = hex_bytes_const(
    "0bdffd6f57c822c79e1050b02299f0153d6c08169e92e7c7b0f2520000000000");

const AUGUST_CONTENT_ROOT_ORACLE: [u8; 32] = hex_bytes_const(
    "7ce89c0f56d4489c13a3750a6e917e61f7c519d3d7da503b6888c1c746c24c0d");
const AUGUST_RECEIPT_DIGEST_ORACLE: [u8; 32] = hex_bytes_const(
    "fae21624c3b88a023722e726b53ab1fb634e104234d0581709f2c7d757d8b30b");
/// Little-endian field encoding of the anchored chameleon commitment's x coordinate.
const AUGUST_COMMITMENT_X_ORACLE: [u8; 32] = [186, 115, 201, 127, 192, 248, 58, 213, 49, 64, 251, 208, 233, 228, 213, 249, 159, 240, 54, 172, 158, 161, 150, 189, 80, 245, 229, 83, 191, 45, 117, 3];
/// Little-endian field encoding of the anchored commitment's y coordinate, from the receipt
/// (sameCommitment.y, decimal 20034...), decoded by an independent Python pass.
const AUGUST_COMMITMENT_Y_ORACLE: [u8; 32] = hex_bytes_const(
    "936214be1d6fd6d46dea1755fd8462b0f8c299f84de29c1dd27b5cd25eab2e2c");

const fn hex_bytes_const(s: &str) -> [u8; 32] {
    let b = s.as_bytes();
    let mut out = [0u8; 32];
    let mut i = 0;
    while i < 32 {
        let hi = b[i * 2];
        let lo = b[i * 2 + 1];
        let h = if hi >= b'a' { hi - b'a' + 10 } else { hi - b'0' };
        let l = if lo >= b'a' { lo - b'a' + 10 } else { lo - b'0' };
        out[i] = h * 16 + l;
        i += 1;
    }
    out
}

/// The pre-Zcash statement plus the appended zcash and august blocks.
/// zcash block (72) + august block (153) appended to the pre-Zcash statement.
pub const TOTAL_PUBLIC_BYTES: usize = PUBLIC_BYTES + 72 + 153;
/// Row 95's beacon round, the one folded into S_96 (chain_log.csv row 95, column drand_round_number).
const ROW96_PREVIOUS_ROUND: u64 = 31_521_620;

pub fn expected_row96_total_public_values(trapdoor_flag: u8) -> Vec<u8> {
    assert!(trapdoor_flag <= 1, "trapdoor flag is a bit");
    let mut public = expected_row96_public_values();
    public.extend_from_slice(b"ZBZCINC3");
    public.extend_from_slice(&ANCHOR_TXID_ORACLE);
    public.extend_from_slice(&ANCHOR_HEADER_HASH_ORACLE);
    public.extend_from_slice(b"ZBAUGST4");
    public.extend_from_slice(&AUGUST_CONTENT_ROOT_ORACLE);
    public.extend_from_slice(&AUGUST_RECEIPT_DIGEST_ORACLE);
    public.extend_from_slice(&AUGUST_COMMITMENT_X_ORACLE);
    public.extend_from_slice(&AUGUST_COMMITMENT_Y_ORACLE);
    public.extend_from_slice(&ROW96_PREVIOUS_ROUND.to_be_bytes());
    public.extend_from_slice(&31_521_620_u64.to_be_bytes()); // row 96's own round (chain_log.csv row 96)
    public.push(trapdoor_flag); // 0 until the ceremony supplies the trapdoor, 1 at proving time
    assert_eq!(public.len(), TOTAL_PUBLIC_BYTES);
    public
}

pub fn check_total_public_values(actual: &[u8], expected: &[u8]) -> Result<(), &'static str> {
    if actual.len() != TOTAL_PUBLIC_BYTES || expected.len() != TOTAL_PUBLIC_BYTES {
        return Err("total public-value length differs");
    }
    if actual != expected {
        return Err("total public values differ from the frozen row-96 oracle");
    }
    Ok(())
}

pub fn check_public_values(actual: &[u8], expected: &[u8]) -> Result<(), &'static str> {
    if actual.len() != PUBLIC_BYTES || expected.len() != PUBLIC_BYTES {
        return Err("membership public-value length differs");
    }
    if actual != expected {
        return Err("membership public values differ from the frozen row-96 oracle");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Digest, Sha256};
    use zeebeam_row_binding_membership::{
        derive_membership_digests, verify_and_encode, MembershipWitness,
    };

    #[test]
    fn frozen_private_witness_matches_cross_language_wire_hash() {
        let wire = row96_membership_witness_bytes();
        assert_eq!(wire.len(), 555);
        assert_eq!(
            MembershipWitness::parse(&wire).unwrap(),
            row96_membership_witness()
        );
        assert_eq!(
            format!("{:x}", Sha256::digest(&wire)),
            "2fb523950e9f204fda2f8f0902414f0215d0cd56e0c81806d933081c6c03db25"
        );
    }

    #[test]
    fn rust_membership_digests_match_the_python_golden() {
        let base = zeebeam_row_binding_join_native::expected_row96_public_values();
        let digests = derive_membership_digests(&base, &row96_membership_witness()).unwrap();
        assert_eq!(
            digests.context_digest_blake3,
            hex_bytes(CONTEXT_DIGEST_BLAKE3)
        );
        assert_eq!(digests.row_leaf_blake3, hex_bytes(ROW_LEAF_BLAKE3));
        assert_eq!(
            digests.internal_root_blake3,
            hex_bytes(INTERNAL_ROOT_BLAKE3)
        );
        assert_eq!(
            digests.wrapped_root_blake3,
            hex_bytes(ORDERED_SESSION_ROOT_BLAKE3)
        );
    }

    #[test]
    fn independent_public_oracle_has_the_frozen_privacy_aligned_hash() {
        let public = expected_row96_public_values();
        assert_eq!(public.len(), 876);
        assert_eq!(
            format!("{:x}", Sha256::digest(&public)),
            "df95a5bf891be3b8573aaf60bbbaf21ae6f18f18de9c60acc2ebf4df403e6abb"
        );
        let base = zeebeam_row_binding_join_native::expected_row96_public_values();
        assert_eq!(&public[352..], &base[328..]);
        assert_eq!(
            format!("{:x}", Sha256::digest(&public[352..])),
            "870634ea0335b87be1f2811a0cdc21932ef451479c2ea09eaada4c7432e420f5"
        );
    }

    #[test]
    fn membership_encoder_matches_the_independent_public_oracle() {
        let base = zeebeam_row_binding_join_native::expected_row96_public_values();
        let actual = verify_and_encode(&base, &row96_membership_witness()).unwrap();
        check_public_values(&actual, &expected_row96_public_values()).unwrap();
    }

    #[test]
    fn every_public_region_is_pinned_by_exact_comparison() {
        let expected = expected_row96_public_values();
        let old_public = zeebeam_row_binding_join_native::expected_row96_public_values();
        assert_eq!(old_public.len(), 852);
        assert!(check_public_values(&old_public, &expected).is_err());
        assert_eq!(&expected[79..156], &[0; 77]);
        for index in [
            0_usize, 8, 9, 10, 11, 12, 16, 20, 24, 26, 28, 78, 155, 156, 188, 220, 252, 284, 316,
            348, 352, 360, 384, 416, 695,
        ] {
            let mut changed = expected.clone();
            changed[index] ^= 1;
            assert!(
                check_public_values(&changed, &expected).is_err(),
                "byte {index}"
            );
        }
        assert!(check_public_values(&expected[..PUBLIC_BYTES - 1], &expected).is_err());
        let mut appended = expected.clone();
        appended.push(0);
        assert!(check_public_values(&appended, &expected).is_err());
    }

    #[test]
    fn coherent_alternate_supplied_root_is_relation_valid_but_not_the_frozen_oracle() {
        let base = zeebeam_row_binding_join_native::expected_row96_public_values();
        let expected = expected_row96_public_values();
        let mut alternate = row96_membership_witness();
        alternate.chain_log_blake3[0] ^= 1;
        alternate.ordered_session_root_blake3 = derive_membership_digests(&base, &alternate)
            .unwrap()
            .wrapped_root_blake3;

        let alternate_public = verify_and_encode(&base, &alternate).unwrap();
        assert_ne!(
            alternate.ordered_session_root_blake3,
            hex_bytes(ORDERED_SESSION_ROOT_BLAKE3)
        );
        assert!(check_public_values(&alternate_public, &expected).is_err());
    }
}
