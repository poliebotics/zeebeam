//! Fixed-instance uncropped-r64 pose verdict relation, joined to the anchored typed
//! tile tree. Proves: a private packed sensor frame (4,2300,2660) whose PREPROCESS_V1
//! camera primary sits (with the supplied emission leaf hashes) under the PUBLIC typed
//! pair root of row 52 ALSO yields, under the uncropped-full reduction (spec
//! a3985f69...), the frozen uncr64 integer pose verdict published here.
//! Ceiling: verdict relation over committed pixels only; never physical pose truth.

use sha2::{Digest, Sha256};
use zeebeam_uncr64_pose_model::{
    reduce_primary_4x4, BlobModel, BLOB_SHA256, CAMERA_CHANNELS, CLASS_COUNT, FULL_SIDE,
    INTEGER_ARTIFACT_SHA256, UNCROPPED_SPEC_SHA256,
};

pub const PACKED_H: usize = 2300;
pub const PACKED_W: usize = 2660;
pub const PACKED_BYTES: usize = CAMERA_CHANNELS * PACKED_H * PACKED_W;
pub const EMISSION_LEAF_COUNT: usize = 16;
pub const PRIVATE_INPUT_BYTES: usize = PACKED_BYTES + EMISSION_LEAF_COUNT * 32;
pub const CROP_Y0: usize = 340;
pub const CROP_X0: usize = 782;
pub const CROP_SIDE: usize = 1024;
pub const ROW: u32 = 52;
pub const PUBLIC_VERSION: u32 = 1;
pub const PUBLIC_BYTES: usize = 400;

const TILE: usize = 64;
const LEAF_COUNT: usize = 32;
const TREE_DEPTH: usize = 5;

pub const PREPROCESS_SPEC_SHA256: &str =
    "6345dc412201aacd2e33626159812e6a8bf839142b403e15f4de1c781982bd6d";
pub const CKPT_FILE_SHA256: &str = "b69f7ce917f9be6fca91ceb742e0450d4760d6885947ef70d6d99984616910fb";
pub const MODEL_STATE_SHA256: &str = "e78e5a007b94a89a171411924de6f7469a4d776b47d19662e0694d017539979a";

pub const EXPECTED_CONTEXT_DIGEST: [u8; 32] = [0x9d, 0x99, 0xa1, 0xf2, 0x94, 0xeb, 0x4e, 0x7d, 0x1d, 0x8b, 0x87, 0xe2, 0x14, 0x98, 0xcf, 0x1b, 0x59, 0x83, 0x54, 0xa9, 0xf7, 0x9b, 0x13, 0x0a, 0x22, 0x7f, 0xe6, 0x62, 0x1d, 0x79, 0x47, 0x89];
pub const EXPECTED_TYPED_ROOT: [u8; 32] = [0xef, 0xd3, 0x5d, 0x05, 0x4d, 0xf6, 0x39, 0xc3, 0xb0, 0x7a, 0xcf, 0xf6, 0xeb, 0xab, 0x5d, 0x2e, 0xbe, 0xc5, 0x9a, 0x85, 0xed, 0x0d, 0xcd, 0xe5, 0xc0, 0xe9, 0x9a, 0xa9, 0x50, 0x06, 0xd3, 0x95];
pub const EXPECTED_UNCROPPED_COMMITMENT: [u8; 32] = [0xfa, 0x5f, 0x5e, 0x09, 0xd2, 0xef, 0x05, 0xe5, 0x83, 0xb0, 0x0b, 0xd8, 0x7c, 0x96, 0xfa, 0xa1, 0x0d, 0x60, 0xb1, 0x0c, 0x10, 0xcc, 0x79, 0x2d, 0xa5, 0x99, 0x81, 0xe7, 0x39, 0x21, 0x0c, 0xf7];
pub const EXPECTED_SUMS: [i64; 11] = [15920, 17697, 8569, -5875, -18570, -2396, -31817, -1193, -21262, -7977, 781];
pub const EXPECTED_VERDICT: u8 = 1;

const CONTEXT_JSON: &[u8; 712] = br#"{"dtype":"uint8","geometry":{"grid_cols":4,"grid_rows":4,"height":256,"tile_height":64,"tile_width":64,"width":256},"hash_algorithm":"blake3-256","layout":{"leaf_count":32,"leaf_order":"spatial_row_major_camera_then_emission","tile_payload_order":"all_modality_channels_channel_major_c_order"},"modalities":[{"channel_order":["R","G1","G2","B"],"code":0,"name":"camera","shape":[4,256,256]},{"channel_order":["R","G","B"],"code":1,"name":"emission","shape":[3,256,256]}],"preprocess":{"id":"PREPROCESS_V1_CANDIDATE_20260823","spec_sha256":"6345dc412201aacd2e33626159812e6a8bf839142b403e15f4de1c781982bd6d"},"schema":"zeebeam-typed-tile-root/v1-candidate","status":"candidate_reference_no_circuit_or_proof_claim"}"#;
const CONTEXT_DOMAIN: &[u8] = b"ZEEBEAM_TYPED_TILE_CONTEXT_V1\x00";
const LEAF_DOMAIN: &[u8] = b"ZEEBEAM_TYPED_TILE_LEAF_V1\x00";
const NODE_DOMAIN: &[u8] = b"ZEEBEAM_TYPED_TILE_NODE_V1\x00";
const ROOT_DOMAIN: &[u8] = b"ZEEBEAM_TYPED_TILE_ROOT_V1\x00";
pub const UNCROPPED_COMMITMENT_DOMAIN: &[u8] = b"zeebeam.camera-pose.uncropped-full.v1\x00";
pub const UNCROPPED_FORMAT_TAG: &[u8] = b"camera:uint8:4x256x256\x00";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UncrPoseStatement {
    pub context: [u8; 32],
    pub typed_root: [u8; 32],
    pub uncropped_commitment: [u8; 32],
    pub logit_sums: [i64; CLASS_COUNT],
    pub verdict: u8,
    pub head_saturation: u32,
}

pub fn evaluate(input: &[u8], model_blob: &[u8]) -> Result<UncrPoseStatement, &'static str> {
    if input.len() != PRIVATE_INPUT_BYTES {
        return Err("private input length differs");
    }
    let mut blob_digest = Sha256::new();
    blob_digest.update(model_blob);
    if blob_digest.finalize().as_slice() != decode_hex_32(BLOB_SHA256) {
        return Err("model blob sha differs");
    }
    let model = BlobModel::parse(model_blob).map_err(|_| "embedded model blob differs")?;
    let packed = &input[..PACKED_BYTES];
    let emission_leaves = &input[PACKED_BYTES..];

    let camera_primary = crop_halfup_mean(packed);
    let (context, typed_root) = typed_root_join(&camera_primary, emission_leaves)?;
    if context != EXPECTED_CONTEXT_DIGEST {
        return Err("typed-root context differs");
    }
    if typed_root != EXPECTED_TYPED_ROOT {
        return Err("row typed root differs");
    }

    let camera_uncropped = uncropped_halfup_mean(packed);
    let uncropped_commitment = uncropped_commitment(ROW, &camera_uncropped);
    if uncropped_commitment != EXPECTED_UNCROPPED_COMMITMENT {
        return Err("uncropped commitment differs");
    }
    let reduced = reduce_primary_4x4(&camera_uncropped, CAMERA_CHANNELS)
        .map_err(|_| "uncropped 256->64 reduction failed")?;
    let out = model.run_reduced(&reduced).map_err(|_| "integer pose inference failed")?;
    if out.logit_sums != EXPECTED_SUMS {
        return Err("fixed integer logit sums differ");
    }
    if out.verdict != EXPECTED_VERDICT {
        return Err("fixed first-maximum verdict differs");
    }
    Ok(UncrPoseStatement {
        context,
        typed_root,
        uncropped_commitment,
        logit_sums: out.logit_sums,
        verdict: out.verdict,
        head_saturation: out.head_saturation,
    })
}

pub fn public_values(statement: &UncrPoseStatement) -> Vec<u8> {
    let mut result = Vec::with_capacity(PUBLIC_BYTES);
    result.extend_from_slice(b"ZBUPOSE1");
    result.extend_from_slice(&PUBLIC_VERSION.to_le_bytes());
    result.extend_from_slice(&ROW.to_le_bytes());
    result.extend_from_slice(&(statement.verdict as u32).to_le_bytes());
    result.extend_from_slice(&statement.head_saturation.to_le_bytes());
    for value in statement.logit_sums {
        result.extend_from_slice(&value.to_le_bytes());
    }
    for digest in [
        statement.context,
        statement.typed_root,
        statement.uncropped_commitment,
    ] {
        result.extend_from_slice(&digest);
    }
    for digest in [
        UNCROPPED_SPEC_SHA256,
        PREPROCESS_SPEC_SHA256,
        BLOB_SHA256,
        INTEGER_ARTIFACT_SHA256,
        CKPT_FILE_SHA256,
        MODEL_STATE_SHA256,
    ] {
        result.extend_from_slice(&decode_hex_32(digest));
    }
    assert_eq!(result.len(), PUBLIC_BYTES);
    result
}

/// PREPROCESS_V1 camera stage on the packed plane: fixed 1024-square crop then 4x4 block
/// mean with HALF-UP rounding ((sum + 8) / 16), exactly the spec's block_mean_uint8.
fn crop_halfup_mean(packed: &[u8]) -> Vec<u8> {
    let mut out = vec![0_u8; CAMERA_CHANNELS * FULL_SIDE * FULL_SIDE];
    for channel in 0..CAMERA_CHANNELS {
        for oy in 0..FULL_SIDE {
            for ox in 0..FULL_SIDE {
                let mut sum = 0_u32;
                for ty in 0..4 {
                    let y = CROP_Y0 + oy * 4 + ty;
                    let base = (channel * PACKED_H + y) * PACKED_W + CROP_X0 + ox * 4;
                    for tx in 0..4 {
                        sum += packed[base + tx] as u32;
                    }
                }
                out[(channel * FULL_SIDE + oy) * FULL_SIDE + ox] = ((sum + 8) / 16) as u8;
            }
        }
    }
    out
}

/// PREPROCESS_UNCROPPED_V1: whole-plane variable rectangular block mean, HALF-UP
/// (floor((sum + count/2) / count)), boundaries ys[i]=(i*2300)/256, xs[j]=(j*2660)/256.
pub fn uncropped_halfup_mean(packed: &[u8]) -> Vec<u8> {
    assert_eq!(
        packed.len(),
        PACKED_BYTES,
        "uncropped_halfup_mean requires the full RGGB-packed camera"
    );
    let mut ys = [0_usize; FULL_SIDE + 1];
    let mut xs = [0_usize; FULL_SIDE + 1];
    for i in 0..=FULL_SIDE {
        ys[i] = i * PACKED_H / FULL_SIDE;
        xs[i] = i * PACKED_W / FULL_SIDE;
    }
    let mut out = vec![0_u8; CAMERA_CHANNELS * FULL_SIDE * FULL_SIDE];
    for channel in 0..CAMERA_CHANNELS {
        for oy in 0..FULL_SIDE {
            for ox in 0..FULL_SIDE {
                let mut sum = 0_u32;
                for y in ys[oy]..ys[oy + 1] {
                    let base = (channel * PACKED_H + y) * PACKED_W;
                    for x in xs[ox]..xs[ox + 1] {
                        sum += packed[base + x] as u32;
                    }
                }
                let count = ((ys[oy + 1] - ys[oy]) * (xs[ox + 1] - xs[ox])) as u32;
                out[(channel * FULL_SIDE + oy) * FULL_SIDE + ox] =
                    ((sum + count / 2) / count) as u8;
            }
        }
    }
    out
}

pub fn uncropped_commitment(row: u32, camera: &[u8]) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(UNCROPPED_COMMITMENT_DOMAIN);
    digest.update(UNCROPPED_SPEC_SHA256.as_bytes());
    digest.update(row.to_be_bytes());
    digest.update(UNCROPPED_FORMAT_TAG);
    digest.update(camera);
    digest.finalize().into()
}

fn typed_root_join(
    camera: &[u8],
    emission_leaves: &[u8],
) -> Result<([u8; 32], [u8; 32]), &'static str> {
    if camera.len() != CAMERA_CHANNELS * FULL_SIDE * FULL_SIDE
        || emission_leaves.len() != EMISSION_LEAF_COUNT * 32
    {
        return Err("typed join input lengths differ");
    }
    let context = context_digest();
    let mut nodes = [[0_u8; 32]; LEAF_COUNT];
    for (leaf_index, destination) in nodes.iter_mut().enumerate() {
        if leaf_index % 2 == 0 {
            *destination = camera_leaf_hash(camera, &context, leaf_index);
        } else {
            let start = (leaf_index / 2) * 32;
            destination.copy_from_slice(&emission_leaves[start..start + 32]);
        }
    }
    let mut width = LEAF_COUNT;
    for level in 1..=TREE_DEPTH {
        let parent_width = width / 2;
        for parent_index in 0..parent_width {
            let left = nodes[2 * parent_index];
            let right = nodes[2 * parent_index + 1];
            nodes[parent_index] = node_hash(&context, level, parent_index, &left, &right);
        }
        width = parent_width;
    }
    Ok((context, final_root(&context, &nodes[0])))
}

fn context_digest() -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(CONTEXT_DOMAIN);
    hasher.update(CONTEXT_JSON);
    *hasher.finalize().as_bytes()
}

fn camera_leaf_hash(camera: &[u8], context: &[u8; 32], leaf_index: usize) -> [u8; 32] {
    let spatial_index = leaf_index / 2;
    let tile_row = spatial_index / 4;
    let tile_col = spatial_index % 4;
    let payload_len = CAMERA_CHANNELS * TILE * TILE;
    let mut header = [0_u8; 8];
    header[0] = leaf_index as u8;
    header[1] = 0;
    header[2] = tile_row as u8;
    header[3] = tile_col as u8;
    header[4..].copy_from_slice(&(payload_len as u32).to_be_bytes());
    let mut hasher = blake3::Hasher::new();
    hasher.update(LEAF_DOMAIN);
    hasher.update(context);
    hasher.update(&header);
    for channel in 0..CAMERA_CHANNELS {
        for row_offset in 0..TILE {
            let y = tile_row * TILE + row_offset;
            let x = tile_col * TILE;
            let start = (channel * FULL_SIDE + y) * FULL_SIDE + x;
            hasher.update(&camera[start..start + TILE]);
        }
    }
    *hasher.finalize().as_bytes()
}

fn node_hash(
    context: &[u8; 32],
    level: usize,
    parent_index: usize,
    left: &[u8; 32],
    right: &[u8; 32],
) -> [u8; 32] {
    let mut header = [0_u8; 5];
    header[0] = level as u8;
    header[1..].copy_from_slice(&(parent_index as u32).to_be_bytes());
    let mut hasher = blake3::Hasher::new();
    hasher.update(NODE_DOMAIN);
    hasher.update(context);
    hasher.update(&header);
    hasher.update(left);
    hasher.update(right);
    *hasher.finalize().as_bytes()
}

fn final_root(context: &[u8; 32], tree_root: &[u8; 32]) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(ROOT_DOMAIN);
    hasher.update(context);
    hasher.update(&(LEAF_COUNT as u32).to_be_bytes());
    hasher.update(tree_root);
    *hasher.finalize().as_bytes()
}

pub fn decode_hex_32(value: &str) -> [u8; 32] {
    assert_eq!(value.len(), 64, "SHA-256 text length differs");
    let bytes = value.as_bytes();
    let mut result = [0_u8; 32];
    for index in 0..32 {
        result[index] = (hex_nibble(bytes[index * 2]) << 4) | hex_nibble(bytes[index * 2 + 1]);
    }
    result
}

fn hex_nibble(value: u8) -> u8 {
    match value {
        b'0'..=b'9' => value - b'0',
        b'a'..=b'f' => value - b'a' + 10,
        _ => panic!("SHA-256 text is not lowercase hexadecimal"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn context_digest_matches_the_anchored_tree() {
        assert_eq!(context_digest(), EXPECTED_CONTEXT_DIGEST);
    }

    #[test]
    fn fixed_instance_evaluates_and_publics_hold() {
        let base = env!("CARGO_MANIFEST_DIR");
        let witness = fs::read(format!("{base}/../fixtures/uncr64_row52_witness.bin"))
            .expect("row-52 witness fixture");
        let blob = fs::read(format!("{base}/../frozen/uncr64_pose_ptq.bin")).expect("blob");
        let statement = evaluate(&witness, &blob).expect("fixed instance");
        assert_eq!(statement.typed_root, EXPECTED_TYPED_ROOT);
        assert_eq!(statement.logit_sums, EXPECTED_SUMS);
        assert_eq!(statement.verdict, EXPECTED_VERDICT);
        let publics = public_values(&statement);
        assert_eq!(publics.len(), PUBLIC_BYTES);
        assert_eq!(&publics[..8], b"ZBUPOSE1");
    }

    #[test]
    fn tampered_witness_fails_closed() {
        // The relation binds the DERIVED tensors and the typed root, not raw witness
        // bytes: a low-bit flip outside every committed derivation is absorbed by the
        // block means by design. Tampering that must propagate is what fails closed.
        let base = env!("CARGO_MANIFEST_DIR");
        let witness = fs::read(format!("{base}/../fixtures/uncr64_row52_witness.bin"))
            .expect("row-52 witness fixture");
        let blob = fs::read(format!("{base}/../frozen/uncr64_pose_ptq.bin")).expect("blob");
        // (a) flip one emission leaf-hash byte: the typed root must differ
        let mut w1 = witness.clone();
        w1[PACKED_BYTES + 5] ^= 1;
        assert_eq!(evaluate(&w1, &blob), Err("row typed root differs"));
        // (b) hard-flip a camera byte inside the PREPROCESS_V1 crop: primary changes,
        // so the recomputed camera leaves and the root must differ
        let mut w2 = witness.clone();
        let inside = (0 * PACKED_H + 1000) * PACKED_W + 1500;
        w2[inside] ^= 0xff;
        assert_eq!(evaluate(&w2, &blob).err(), Some("row typed root differs"));
        // (c) hard-flip a camera byte OUTSIDE the crop: the root holds, but the
        // uncropped commitment must differ
        let mut w3 = witness.clone();
        let outside = (0 * PACKED_H + 4) * PACKED_W + 1705;
        w3[outside] ^= 0xff;
        assert_eq!(evaluate(&w3, &blob).err(), Some("uncropped commitment differs"));
        // (d) ACCEPTED absorbed flip: a low bit whose block sums round identically in
        // both reductions leaves every committed value unchanged, so evaluate accepts
        let mut w4 = witness;
        w4[12345] ^= 1;
        assert!(evaluate(&w4, &blob).is_ok());
    }
}
