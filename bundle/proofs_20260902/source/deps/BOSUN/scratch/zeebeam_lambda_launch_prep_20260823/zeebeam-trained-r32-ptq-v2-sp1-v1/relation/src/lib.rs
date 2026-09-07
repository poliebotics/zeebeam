//! Typed primary-root and public-value contract for trained-r32 inference.

use zeebeam_trained_r32_ptq_v2_model::{
    BLOB_MANIFEST_SHA256, BLOB_SHA256, CAMERA_CHANNELS, CAMERA_PRIMARY_BYTES, CHECKPOINT_SHA256,
    EMISSION_CHANNELS, EMISSION_PRIMARY_BYTES, FINAL_MODEL_STATE_SHA256, FULL_SIDE,
    INTEGER_ARTIFACT_SHA256, MODEL_SPEC_SHA256, PARITY_REPORT_SHA256, PTQ_SOURCE_STATE_SHA256,
};

const TILE: usize = 64;
const CAMERA_BYTES: usize = CAMERA_PRIMARY_BYTES;
const LEAF_COUNT: usize = 32;
const TREE_DEPTH: usize = 5;

pub const PUBLIC_VERSION: u8 = 1;
pub const SCORE_DENOMINATOR: u64 = 4;
pub const PUBLIC_BYTES: usize = 344;
pub const EXPECTED_CONTEXT_DIGEST: [u8; 32] = [
    0x9d, 0x99, 0xa1, 0xf2, 0x94, 0xeb, 0x4e, 0x7d, 0x1d, 0x8b, 0x87, 0xe2, 0x14, 0x98, 0xcf, 0x1b,
    0x59, 0x83, 0x54, 0xa9, 0xf7, 0x9b, 0x13, 0x0a, 0x22, 0x7f, 0xe6, 0x62, 0x1d, 0x79, 0x47, 0x89,
];

const CONTEXT_JSON: &[u8; 712] = br#"{"dtype":"uint8","geometry":{"grid_cols":4,"grid_rows":4,"height":256,"tile_height":64,"tile_width":64,"width":256},"hash_algorithm":"blake3-256","layout":{"leaf_count":32,"leaf_order":"spatial_row_major_camera_then_emission","tile_payload_order":"all_modality_channels_channel_major_c_order"},"modalities":[{"channel_order":["R","G1","G2","B"],"code":0,"name":"camera","shape":[4,256,256]},{"channel_order":["R","G","B"],"code":1,"name":"emission","shape":[3,256,256]}],"preprocess":{"id":"PREPROCESS_V1_CANDIDATE_20260823","spec_sha256":"6345dc412201aacd2e33626159812e6a8bf839142b403e15f4de1c781982bd6d"},"schema":"zeebeam-typed-tile-root/v1-candidate","status":"candidate_reference_no_circuit_or_proof_claim"}"#;
const CONTEXT_DOMAIN: &[u8] = b"ZEEBEAM_TYPED_TILE_CONTEXT_V1\x00";
const LEAF_DOMAIN: &[u8] = b"ZEEBEAM_TYPED_TILE_LEAF_V1\x00";
const NODE_DOMAIN: &[u8] = b"ZEEBEAM_TYPED_TILE_NODE_V1\x00";
const ROOT_DOMAIN: &[u8] = b"ZEEBEAM_TYPED_TILE_ROOT_V1\x00";

pub fn typed_root(input: &[u8]) -> Result<([u8; 32], [u8; 32]), &'static str> {
    if input.len() != CAMERA_PRIMARY_BYTES + EMISSION_PRIMARY_BYTES {
        return Err("private primary tensor length differs");
    }
    let context = context_digest();
    if context != EXPECTED_CONTEXT_DIGEST {
        return Err("typed-root context digest differs");
    }
    let mut nodes = [[0_u8; 32]; LEAF_COUNT];
    for (leaf_index, destination) in nodes.iter_mut().enumerate() {
        *destination = leaf_hash(input, &context, leaf_index);
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
    if width != 1 {
        return Err("typed-root tree width differs");
    }
    Ok((context, final_root(&context, &nodes[0])))
}

pub fn public_values(score: i64, context: &[u8; 32], root: &[u8; 32]) -> Vec<u8> {
    let mut result = Vec::with_capacity(PUBLIC_BYTES);
    result.extend_from_slice(&[
        PUBLIC_VERSION,
        32,
        2,
        CAMERA_CHANNELS as u8,
        EMISSION_CHANNELS as u8,
        0,
        0,
        0,
    ]);
    result.extend_from_slice(&score.to_le_bytes());
    result.extend_from_slice(&SCORE_DENOMINATOR.to_le_bytes());
    result.extend_from_slice(context);
    result.extend_from_slice(root);
    for digest in [
        BLOB_SHA256,
        BLOB_MANIFEST_SHA256,
        MODEL_SPEC_SHA256,
        CHECKPOINT_SHA256,
        FINAL_MODEL_STATE_SHA256,
        PTQ_SOURCE_STATE_SHA256,
        INTEGER_ARTIFACT_SHA256,
        PARITY_REPORT_SHA256,
    ] {
        result.extend_from_slice(&decode_hex_32(digest));
    }
    assert_eq!(result.len(), PUBLIC_BYTES);
    result
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

fn context_digest() -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(CONTEXT_DOMAIN);
    hasher.update(CONTEXT_JSON);
    *hasher.finalize().as_bytes()
}

fn leaf_hash(input: &[u8], context: &[u8; 32], leaf_index: usize) -> [u8; 32] {
    let spatial_index = leaf_index / 2;
    let modality_code = leaf_index % 2;
    let tile_row = spatial_index / 4;
    let tile_col = spatial_index % 4;
    let channels = if modality_code == 0 {
        CAMERA_CHANNELS
    } else {
        EMISSION_CHANNELS
    };
    let modality_base = if modality_code == 0 { 0 } else { CAMERA_BYTES };
    let payload_len = channels * TILE * TILE;
    let mut header = [0_u8; 8];
    header[0] = leaf_index as u8;
    header[1] = modality_code as u8;
    header[2] = tile_row as u8;
    header[3] = tile_col as u8;
    header[4..].copy_from_slice(&(payload_len as u32).to_be_bytes());
    let mut hasher = blake3::Hasher::new();
    hasher.update(LEAF_DOMAIN);
    hasher.update(context);
    hasher.update(&header);
    for channel in 0..channels {
        for row_offset in 0..TILE {
            let y = tile_row * TILE + row_offset;
            let x = tile_col * TILE;
            let start = modality_base + (channel * FULL_SIDE + y) * FULL_SIDE + x;
            hasher.update(&input[start..start + TILE]);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frozen_context_and_public_size_hold() {
        assert_eq!(context_digest(), EXPECTED_CONTEXT_DIGEST);
        assert_eq!(
            public_values(7, &EXPECTED_CONTEXT_DIGEST, &[3_u8; 32]).len(),
            PUBLIC_BYTES
        );
    }
}
