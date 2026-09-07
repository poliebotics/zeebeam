//! Candidate-only single-row ZeeBeam v9 binding relation.
//!
//! This joins mandatory raw Bayer hashing, the exact v9 transition, same-row
//! BLAKE3-XOF rendering, PREPROCESS_V1_CANDIDATE, the existing typed root, and
//! the frozen trained-r32 score. It deliberately contains no session identifier,
//! chain-log commitment, or membership opening. Consequently
//! `ROW_MEMBERSHIP_PROVED` is permanently false for this ABI.

use core::fmt;

use sha2::{Digest as ShaDigest, Sha256};
use bls12_381::hash_to_curve::{ExpandMsgXmd, HashToCurve};
use bls12_381::{pairing, G1Affine, G1Projective, G2Affine};
use zeebeam_b3xof_relation as projector;
use zeebeam_preprocess_v1_candidate as preprocess;
use zeebeam_trained_r32_ptq_v2_model::{
    BlobModel, BLOB_SHA256, CAMERA_PRIMARY_BYTES, EMISSION_PRIMARY_BYTES,
};
use zeebeam_trained_r32_ptq_v2_relation::{
    public_values as scorer_public_values, typed_root, PUBLIC_BYTES as SCORER_PUBLIC_BYTES,
};
// POSE_LEG: the uncropped pose scorer, joined to this relation.
use zeebeam_uncr64_pose_model::{
    reduce_primary_4x4 as pose_reduce_4x4, BlobModel as PoseBlobModel,
    BLOB_SHA256 as POSE_BLOB_SHA256, CAMERA_CHANNELS as POSE_CAMERA_CHANNELS,
};
use zeebeam_uncr64_pose_relation::{uncropped_commitment, uncropped_halfup_mean};

pub const POSE_PUBLIC_MAGIC: &[u8; 8] = b"ZBJPOSE1";
// DRAND_LEG: verify the quicknet beacon signature in circuit and bind it to the
// drand_value that advance_chain already folds into S_next.
pub const BEACON_PUBLIC_MAGIC: &[u8; 8] = b"ZBJBEAC1";
/// drand quicknet chain hash (the network identifier every drand client pins).
pub const QUICKNET_CHAIN_HASH: [u8; 32] = [
    0x52, 0xdb, 0x9b, 0xa7, 0x0e, 0x0c, 0xc0, 0xf6, 0xea, 0xf7, 0x80, 0x3d, 0xd0, 0x74, 0x47, 0xa1,
    0xf5, 0x47, 0x77, 0x35, 0xfd, 0x3f, 0x66, 0x17, 0x92, 0xba, 0x94, 0x60, 0x0c, 0x84, 0xe9, 0x71,
];
pub const BEACON_PUBLIC_BYTES: usize = 8 + 4 + 32;
pub const QUICKNET_DST: &[u8] = b"BLS_SIG_BLS12381G1_XMD:SHA-256_SSWU_RO_NUL_";
pub const QUICKNET_PUBLIC_KEY: [u8; 96] = [
    0x83,0xcf,0x0f,0x28,0x96,0xad,0xee,0x7e,0xb8,0xb5,0xf0,0x1f,0xca,0xd3,0x91,0x22,
    0x12,0xc4,0x37,0xe0,0x07,0x3e,0x91,0x1f,0xb9,0x00,0x22,0xd3,0xe7,0x60,0x18,0x3c,
    0x8c,0x4b,0x45,0x0b,0x6a,0x0a,0x6c,0x3a,0xc6,0xa5,0x77,0x6a,0x2d,0x10,0x64,0x51,
    0x0d,0x1f,0xec,0x75,0x8c,0x92,0x1c,0xc2,0x2b,0x0e,0x17,0xe6,0x3a,0xaf,0x4b,0xcb,
    0x5e,0xd6,0x63,0x04,0xde,0x9c,0xf8,0x09,0xbd,0x27,0x4c,0xa7,0x3b,0xab,0x4a,0xf5,
    0xa6,0xe9,0xc7,0x6a,0x4b,0xc0,0x9e,0x76,0xea,0xe8,0x99,0x1e,0xf5,0xec,0xe4,0x5a,
];
pub const POSE_MODEL_BLOB: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../../scratch/uncr64/uncr64_pose_sp1_20260831/frozen/uncr64_pose_ptq.bin"
));

pub const CLASSIFICATION: &str = "CANDIDATE_ONLY";
pub const ROW_MEMBERSHIP_PROVED: bool = false;
pub const PRIVATE_MAGIC: &[u8; 8] = b"ZBROWW01";
pub const PRIVATE_ABI_VERSION: u8 = 1;
pub const PROTOCOL_V9_CODE: u8 = 9;
pub const PRIVATE_HEADER_BYTES: usize = 120;
pub const PUBLIC_MAGIC: &[u8; 8] = b"ZBROWJ01";
pub const PUBLIC_ABI_VERSION: u8 = 2; // v2: pose block + beacon block appended (audit 1 Sep)
pub const PUBLIC_PREFIX_BYTES: usize = 328;
pub const POSE_PUBLIC_BYTES: usize = 8 + 4 + 4 + 11 * 8 + 32; // 136
pub const PUBLIC_BYTES: usize =
    PUBLIC_PREFIX_BYTES + SCORER_PUBLIC_BYTES + POSE_PUBLIC_BYTES + BEACON_PUBLIC_BYTES;
pub const PRIMARY_OUTPUT_BYTES: usize = CAMERA_PRIMARY_BYTES + EMISSION_PRIMARY_BYTES;
pub const PREPROCESS_SPEC_SHA256_HEX: &str =
    "6345dc412201aacd2e33626159812e6a8bf839142b403e15f4de1c781982bd6d";
pub const PREPROCESS_SPEC_SHA256: [u8; 32] = [
    0x63, 0x45, 0xdc, 0x41, 0x22, 0x01, 0xaa, 0xcd, 0x2e, 0x33, 0x62, 0x61, 0x59, 0x81, 0x2e, 0x6a,
    0x8b, 0xf8, 0x39, 0x14, 0x2b, 0x40, 0x3e, 0x15, 0xf4, 0xde, 0x1c, 0x78, 0x19, 0x82, 0xbd, 0x6d,
];

const PRIMARY_PAIR_DOMAIN: &[u8] = b"zeebeam.resolution-ladder.primary-tensor-sha256.v1\0";
const CAMERA_TYPE: &[u8] = b"camera:uint8:4x256x256\0";
const EMISSION_TYPE: &[u8] = b"emission:uint8:3x256x256\0";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JoinError(&'static str);

impl JoinError {
    pub const fn message(self) -> &'static str {
        self.0
    }
}

impl fmt::Display for JoinError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0)
    }
}

impl std::error::Error for JoinError {}

pub type Result<T> = core::result::Result<T, JoinError>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RowWitnessHeader {
    pub row_index: u32,
    pub s_t: [u8; 32],
    pub meta: [u8; projector::META_BYTES],
    pub drand_round: u64,
    pub drand_value: [u8; 32],
}

impl RowWitnessHeader {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != PRIVATE_HEADER_BYTES {
            return Err(JoinError("private header must be exactly 120 bytes"));
        }
        if &bytes[0..8] != PRIVATE_MAGIC {
            return Err(JoinError("private header magic differs"));
        }
        if bytes[8] != PRIVATE_ABI_VERSION {
            return Err(JoinError("private ABI version differs"));
        }
        if bytes[9] != PROTOCOL_V9_CODE {
            return Err(JoinError("private protocol code must be TB-v0.9"));
        }
        if bytes[10..12] != [0; 2] || bytes[116..120] != [0; 4] {
            return Err(JoinError("private reserved bytes must be zero"));
        }

        let row_index = u32::from_le_bytes(bytes[12..16].try_into().expect("fixed row index"));
        let mut s_t = [0_u8; 32];
        s_t.copy_from_slice(&bytes[16..48]);
        let mut meta = [0_u8; projector::META_BYTES];
        meta.copy_from_slice(&bytes[48..76]);
        let drand_round = u64::from_le_bytes(bytes[76..84].try_into().expect("fixed drand round"));
        let mut drand_value = [0_u8; 32];
        drand_value.copy_from_slice(&bytes[84..116]);

        let meta_row = u32::from_be_bytes(meta[0..4].try_into().expect("fixed metadata row"));
        if meta_row != row_index {
            return Err(JoinError("metadata row index differs from private header"));
        }
        Ok(Self {
            row_index,
            s_t,
            meta,
            drand_round,
            drand_value,
        })
    }

    pub fn encode(&self) -> Result<[u8; PRIVATE_HEADER_BYTES]> {
        let meta_row = u32::from_be_bytes(self.meta[0..4].try_into().expect("fixed metadata row"));
        if meta_row != self.row_index {
            return Err(JoinError("metadata row index differs from private header"));
        }
        let mut bytes = [0_u8; PRIVATE_HEADER_BYTES];
        bytes[0..8].copy_from_slice(PRIVATE_MAGIC);
        bytes[8] = PRIVATE_ABI_VERSION;
        bytes[9] = PROTOCOL_V9_CODE;
        bytes[12..16].copy_from_slice(&self.row_index.to_le_bytes());
        bytes[16..48].copy_from_slice(&self.s_t);
        bytes[48..76].copy_from_slice(&self.meta);
        bytes[76..84].copy_from_slice(&self.drand_round.to_le_bytes());
        bytes[84..116].copy_from_slice(&self.drand_value);
        Ok(bytes)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RowPublicInputs {
    pub row_index: u32,
    pub s_t: [u8; 32],
    pub s_next: [u8; 32],
    pub raw_blake3: [u8; 32],
    pub emission_blake3: [u8; 32],
    pub preprocess_output_sha256: [u8; 32],
    pub primary_pair_sha256: [u8; 32],
    pub meta: [u8; projector::META_BYTES],
    pub drand_round: u64,
    pub drand_value: [u8; 32],
}

pub fn primary_pair_sha256(row_index: u32, camera: &[u8], emission: &[u8]) -> Result<[u8; 32]> {
    if camera.len() != CAMERA_PRIMARY_BYTES || emission.len() != EMISSION_PRIMARY_BYTES {
        return Err(JoinError("primary-pair tensor lengths differ"));
    }
    let mut digest = Sha256::new();
    digest.update(PRIMARY_PAIR_DOMAIN);
    digest.update(preprocess::PREPROCESS_ID.as_bytes());
    digest.update(b"\0");
    digest.update(PREPROCESS_SPEC_SHA256);
    digest.update(row_index.to_be_bytes());
    digest.update(CAMERA_TYPE);
    digest.update(camera);
    digest.update(EMISSION_TYPE);
    digest.update(emission);
    Ok(digest.finalize().into())
}

pub struct PosePublic {
    pub verdict: u8,
    pub head_saturation: u32,
    pub logit_sums: [i64; 11],
    pub commitment: [u8; 32],
}

pub fn encode_public_values(
    row: &RowPublicInputs,
    score: i64,
    context: &[u8; 32],
    root: &[u8; 32],
    pose: &PosePublic,
) -> Vec<u8> {
    let mut public = Vec::with_capacity(PUBLIC_BYTES);
    public.extend_from_slice(PUBLIC_MAGIC);
    public.push(PUBLIC_ABI_VERSION);
    public.push(PROTOCOL_V9_CODE);
    public.extend_from_slice(&[0_u8; 2]);
    public.extend_from_slice(&(PUBLIC_BYTES as u32).to_le_bytes());
    public.extend_from_slice(&row.row_index.to_le_bytes());
    public.extend_from_slice(&(preprocess::RAW_BYTES as u32).to_le_bytes());
    public.extend_from_slice(&(preprocess::EMISSION_BYTES as u32).to_le_bytes());
    public.extend_from_slice(&(PRIMARY_OUTPUT_BYTES as u32).to_le_bytes());
    public.extend_from_slice(&row.s_t);
    public.extend_from_slice(&row.s_next);
    public.extend_from_slice(&row.raw_blake3);
    public.extend_from_slice(&row.emission_blake3);
    public.extend_from_slice(&PREPROCESS_SPEC_SHA256);
    public.extend_from_slice(&row.preprocess_output_sha256);
    public.extend_from_slice(&row.primary_pair_sha256);
    public.extend_from_slice(&row.meta);
    public.extend_from_slice(&row.drand_round.to_be_bytes());
    public.extend_from_slice(&row.drand_value);
    public.extend_from_slice(&[0_u8; 4]);
    assert_eq!(public.len(), PUBLIC_PREFIX_BYTES);
    let scorer = scorer_public_values(score, context, root);
    assert_eq!(scorer.len(), SCORER_PUBLIC_BYTES);
    public.extend_from_slice(&scorer);
    assert_eq!(public.len(), PUBLIC_PREFIX_BYTES + SCORER_PUBLIC_BYTES);
    public.extend_from_slice(POSE_PUBLIC_MAGIC);
    public.extend_from_slice(&(pose.verdict as u32).to_le_bytes());
    public.extend_from_slice(&pose.head_saturation.to_le_bytes());
    for value in pose.logit_sums {
        public.extend_from_slice(&value.to_le_bytes());
    }
    public.extend_from_slice(&pose.commitment);
    public.extend_from_slice(BEACON_PUBLIC_MAGIC);
    public.extend_from_slice(&1_u32.to_le_bytes()); // verified; the relation errors otherwise
    public.extend_from_slice(&QUICKNET_CHAIN_HASH);
    assert_eq!(public.len(), PUBLIC_BYTES);
    public
}

/// Verify a drand quicknet beacon signature for `round` under the compiled-in public key and
/// return SHA-256(signature), the beacon's randomness. Shared by the row's own beacon check and
/// by the previous-row advance leg in the membership guest.
pub fn verify_quicknet_beacon(round: u64, drand_signature: &[u8]) -> Result<[u8; 32]> {
    if drand_signature.len() != 48 {
        return Err(JoinError("drand quicknet signature must be 48 bytes"));
    }
    let mut sig_bytes = [0_u8; 48];
    sig_bytes.copy_from_slice(drand_signature);
    let sig_point = G1Affine::from_compressed(&sig_bytes);
    if bool::from(sig_point.is_none()) {
        return Err(JoinError("drand signature is not a valid G1 point"));
    }
    let sig_point = sig_point.unwrap();
    if bool::from(sig_point.is_identity()) {
        return Err(JoinError("drand signature is the identity point"));
    }
    let pk_point = G2Affine::from_compressed(&QUICKNET_PUBLIC_KEY);
    if bool::from(pk_point.is_none()) {
        return Err(JoinError("quicknet public key is not a valid G2 point"));
    }
    let pk_point = pk_point.unwrap();
    if bool::from(pk_point.is_identity()) {
        return Err(JoinError("quicknet public key is the identity point"));
    }
    let beacon_message: [u8; 32] = Sha256::digest(round.to_be_bytes()).into();
    let hashed: G1Affine = <G1Projective as HashToCurve<ExpandMsgXmd<Sha256>>>::hash_to_curve(
        [beacon_message.as_slice()],
        QUICKNET_DST,
    )
    .into();
    if pairing(&sig_point, &G2Affine::generator()) != pairing(&hashed, &pk_point) {
        return Err(JoinError("drand quicknet signature does not verify"));
    }
    Ok(Sha256::digest(drand_signature).into())
}

pub fn evaluate(
    header: &RowWitnessHeader,
    raw_bayer: &[u8],
    model_blob: &[u8],
    drand_signature: &[u8],
) -> Result<Vec<u8>> {
    if raw_bayer.len() != preprocess::RAW_BYTES {
        return Err(JoinError(
            "raw Bayer input must be exactly 24,472,000 bytes",
        ));
    }

    let actual_spec: [u8; 32] = Sha256::digest(preprocess::SPEC_CANONICAL_JSON.as_bytes()).into();
    if actual_spec != PREPROCESS_SPEC_SHA256 {
        return Err(JoinError(
            "embedded PREPROCESS_V1_CANDIDATE specification differs",
        ));
    }
    let expected_model = zeebeam_trained_r32_ptq_v2_relation::decode_hex_32(BLOB_SHA256);
    let actual_model: [u8; 32] = Sha256::digest(model_blob).into();
    if actual_model != expected_model {
        return Err(JoinError("embedded trained-r32 model blob SHA-256 differs"));
    }
    let model = BlobModel::parse(model_blob)
        .map_err(|_| JoinError("embedded trained-r32 model blob cannot be parsed"))?;

    println!("cycle-tracker-report-start: raw_blake3_24MB");
    // DRAND_LEG: quicknet is bls-unchained-g1-rfc9380. Verify
    //   e(signature, G2 generator) == e(hash_to_G1(sha256(round_be8), DST), quicknet_pk)
    // then bind it by requiring sha256(signature) == the drand_value this row's chain
    // advance consumes. Without that second check the pairing would be free-floating.
    println!("cycle-tracker-report-start: drand_verify");
    let signature_digest = verify_quicknet_beacon(header.drand_round, drand_signature)?;
    if signature_digest != header.drand_value {
        return Err(JoinError("drand randomness is not sha256 of the verified signature"));
    }
    println!("cycle-tracker-report-end: drand_verify");

    let raw_blake3 = projector::blake3p::hash(raw_bayer);
    println!("cycle-tracker-report-end: raw_blake3_24MB");

    let s_next = projector::advance_chain(
        &header.s_t,
        &raw_blake3,
        &header.meta,
        header.drand_round,
        &header.drand_value,
    );

    println!("cycle-tracker-report-start: xof_expand");
    let conditioning = projector::expand_all(&header.s_t);
    println!("cycle-tracker-report-end: xof_expand");

    println!("cycle-tracker-report-start: emission_render_1920x1080");
    let tables = projector::RenderTables::new();
    let mut emission_rgb = vec![0_u8; projector::TILE_BYTES];
    let mut emission_hasher = projector::blake3p::Hasher::new();
    let row_bytes = projector::TILE_W * projector::CHANNELS;
    for y in 0..projector::TILE_H {
        let start = y * row_bytes;
        let row = &mut emission_rgb[start..start + row_bytes];
        projector::render_row_rgb(&conditioning, &tables, y, 0, projector::TILE_W, row);
        emission_hasher.update(row);
    }
    let emission_blake3 = emission_hasher.finalize();
    println!("cycle-tracker-report-end: emission_render_1920x1080");

    drop(conditioning);
    drop(tables);

    println!("cycle-tracker-report-start: preprocess_and_pack");
    // RGGB planes, packed once for both consumers: PREPROCESS_V1 and the pose leg.
    println!("cycle-tracker-report-start: rggb_pack");
    let packed_planes = preprocess::pack_bayer_rggb(
        raw_bayer,
        preprocess::RAW_HEIGHT,
        preprocess::RAW_WIDTH,
    )
    .map_err(|_| JoinError("RGGB packing failed"))?;
    println!("cycle-tracker-report-end: rggb_pack");
    let (camera, emission) = preprocess::preprocess_candidate_packed(
        &packed_planes,
        &emission_rgb,
        preprocess::PRIMARY_OUTPUT_SIZE,
    )
    .map_err(|_| JoinError("PREPROCESS_V1_CANDIDATE evaluation failed"))?;
    drop(emission_rgb);
    let primary = preprocess::output_bytes(&camera, &emission, preprocess::PRIMARY_OUTPUT_SIZE)
        .map_err(|_| JoinError("PREPROCESS_V1_CANDIDATE output packing failed"))?;
    if primary.len() != PRIMARY_OUTPUT_BYTES {
        return Err(JoinError(
            "PREPROCESS_V1_CANDIDATE primary output length differs",
        ));
    }
    let preprocess_output_sha256: [u8; 32] = Sha256::digest(&primary).into();
    println!("cycle-tracker-report-end: preprocess_and_pack");

    let primary_pair_sha256 = primary_pair_sha256(header.row_index, &camera, &emission)?;
    println!("cycle-tracker-report-start: typed_root");
    let (context, root) = typed_root(&primary).map_err(JoinError)?;
    println!("cycle-tracker-report-end: typed_root");

    println!("cycle-tracker-report-start: r32_coupling_score");
    let score = model
        .score_primary_pair(&camera, &emission)
        .map_err(|_| JoinError("trained-r32 score evaluation failed"))?;
    println!("cycle-tracker-report-end: r32_coupling_score");


    // POSE_LEG: same raw_bayer, uncropped whole-plane reduction, frozen uncr64 integer scorer.
    println!("cycle-tracker-report-start: pose_leg");
    let pose_blob_actual: [u8; 32] = Sha256::digest(POSE_MODEL_BLOB).into();
    if pose_blob_actual != zeebeam_trained_r32_ptq_v2_relation::decode_hex_32(POSE_BLOB_SHA256) {
        return Err(JoinError("embedded uncr64 pose model blob SHA-256 differs"));
    }
    let pose_model = PoseBlobModel::parse(POSE_MODEL_BLOB)
        .map_err(|_| JoinError("embedded uncr64 pose model blob cannot be parsed"))?;
    // The pose scorer consumes the RGGB-packed planes (4 x 2300 x 2660), not the raw
    // 4600 x 5320 Bayer plane. Both are 24,472,000 bytes, so passing raw_bayer straight in
    // compiles and runs while silently scoring the wrong image. Verified against the audited
    // row-52 parity fixture: raw -> pack_bayer_rggb -> uncropped mean -> 4x4 reduce is
    // bit-exact on 5 of 5 rows tested.
    let camera_uncropped = uncropped_halfup_mean(&packed_planes);
    let pose_commitment = uncropped_commitment(header.row_index, &camera_uncropped);
    let pose_reduced = pose_reduce_4x4(&camera_uncropped, POSE_CAMERA_CHANNELS)
        .map_err(|_| JoinError("uncropped 256->64 reduction failed"))?;
    let pose_out = pose_model
        .run_reduced(&pose_reduced)
        .map_err(|_| JoinError("uncr64 integer pose inference failed"))?;
    println!("cycle-tracker-report-end: pose_leg");

    let row = RowPublicInputs {
        row_index: header.row_index,
        s_t: header.s_t,
        s_next,
        raw_blake3,
        emission_blake3,
        preprocess_output_sha256,
        primary_pair_sha256,
        meta: header.meta,
        drand_round: header.drand_round,
        drand_value: header.drand_value,
    };
    let pose = PosePublic {
        verdict: pose_out.verdict,
        head_saturation: pose_out.head_saturation,
        logit_sums: pose_out.logit_sums,
        commitment: pose_commitment,
    };
    Ok(encode_public_values(&row, score, &context, &root, &pose))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_header() -> RowWitnessHeader {
        let mut meta = [0_u8; projector::META_BYTES];
        meta[0..4].copy_from_slice(&96_u32.to_be_bytes());
        RowWitnessHeader {
            row_index: 96,
            s_t: [0x11; 32],
            meta,
            drand_round: 31_521_620,
            drand_value: [0x22; 32],
        }
    }

    #[test]
    fn private_header_round_trips_exactly() {
        let expected = example_header();
        let encoded = expected.encode().unwrap();
        assert_eq!(encoded.len(), PRIVATE_HEADER_BYTES);
        assert_eq!(RowWitnessHeader::parse(&encoded).unwrap(), expected);
    }

    #[test]
    fn private_header_framing_and_reserved_bytes_fail_closed() {
        let honest = example_header().encode().unwrap();
        for index in [0_usize, 8, 9, 10, 116] {
            let mut changed = honest;
            changed[index] ^= 1;
            assert!(RowWitnessHeader::parse(&changed).is_err(), "byte {index}");
        }
        assert!(RowWitnessHeader::parse(&honest[..PRIVATE_HEADER_BYTES - 1]).is_err());
        let mut wrong_row = honest;
        wrong_row[12..16].copy_from_slice(&97_u32.to_le_bytes());
        assert!(RowWitnessHeader::parse(&wrong_row).is_err());
    }

    #[test]
    fn public_layout_is_fixed_and_candidate_scope_is_explicit() {
        let row = RowPublicInputs {
            row_index: 96,
            s_t: [1; 32],
            s_next: [2; 32],
            raw_blake3: [3; 32],
            emission_blake3: [4; 32],
            preprocess_output_sha256: [5; 32],
            primary_pair_sha256: [6; 32],
            meta: example_header().meta,
            drand_round: 7,
            drand_value: [8; 32],
        };
        let pose = PosePublic {
            verdict: 2,
            head_saturation: 0,
            logit_sums: [0; 11],
            commitment: [12; 32],
        };
        let public = encode_public_values(&row, 9, &[10; 32], &[11; 32], &pose);
        assert_eq!(public.len(), PUBLIC_BYTES);
        assert_eq!(public.len(), 852);
        assert_eq!(&public[0..8], PUBLIC_MAGIC);
        assert_eq!(public[8], PUBLIC_ABI_VERSION);
        assert_eq!(u32::from_le_bytes(public[12..16].try_into().unwrap()), 852);
        assert_eq!(&public[324..328], &[0; 4]);
        assert_eq!(&public[328..336], &[1, 32, 2, 4, 3, 0, 0, 0]);
        assert_eq!(CLASSIFICATION, "CANDIDATE_ONLY");
        assert!(!ROW_MEMBERSHIP_PROVED);
    }

    #[test]
    fn exact_raw_length_is_mandatory_before_expensive_work() {
        let error = evaluate(&example_header(), b"short", b"", &[0_u8; 48]).unwrap_err();
        assert_eq!(
            error.message(),
            "raw Bayer input must be exactly 24,472,000 bytes"
        );
    }

    #[test]
    fn primary_pair_binding_changes_with_row_and_tensor_bytes() {
        let camera = vec![1_u8; CAMERA_PRIMARY_BYTES];
        let emission = vec![2_u8; EMISSION_PRIMARY_BYTES];
        let honest = primary_pair_sha256(96, &camera, &emission).unwrap();
        assert_ne!(honest, primary_pair_sha256(97, &camera, &emission).unwrap());
        let mut changed = camera;
        changed[12345] ^= 1;
        assert_ne!(
            honest,
            primary_pair_sha256(96, &changed, &emission).unwrap()
        );
    }
}
