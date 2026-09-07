//! Standard-library reference implementation of PREPROCESS_V1_CANDIDATE.
//!
//! The candidate is deliberately unfrozen. This crate implements the exact
//! current Python byte relation without granting scientific or production
//! clearance.

use std::error::Error;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PreprocessError(String);

impl PreprocessError {
    fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

impl fmt::Display for PreprocessError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl Error for PreprocessError {}

pub type Result<T> = std::result::Result<T, PreprocessError>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Crop {
    pub y0: usize,
    pub x0: usize,
    pub height: usize,
    pub width: usize,
}

include!(concat!(env!("OUT_DIR"), "/preprocess_spec.rs"));

fn exact_len(height: usize, width: usize, channels: usize) -> Result<usize> {
    height
        .checked_mul(width)
        .and_then(|pixels| pixels.checked_mul(channels))
        .ok_or_else(|| PreprocessError::new("geometry byte count overflow"))
}

fn ensure_crop(crop: Crop, height: usize, width: usize) -> Result<()> {
    if crop.height == 0 || crop.width == 0 {
        return Err(PreprocessError::new("crop dimensions must be positive"));
    }
    let y1 = crop
        .y0
        .checked_add(crop.height)
        .ok_or_else(|| PreprocessError::new("crop height overflow"))?;
    let x1 = crop
        .x0
        .checked_add(crop.width)
        .ok_or_else(|| PreprocessError::new("crop width overflow"))?;
    if y1 > height || x1 > width {
        return Err(PreprocessError::new("crop exceeds input geometry"));
    }
    Ok(())
}

/// Pack headerless row-major RGGB bytes into channel-major R, G1, G2, B.
pub fn pack_bayer_rggb(raw: &[u8], height: usize, width: usize) -> Result<Vec<u8>> {
    if height == 0 || width == 0 || height % 2 != 0 || width % 2 != 0 {
        return Err(PreprocessError::new(
            "raw RGGB geometry must be positive and even",
        ));
    }
    let expected = exact_len(height, width, 1)?;
    if raw.len() != expected {
        return Err(PreprocessError::new(format!(
            "raw Bayer byte count differs: expected {expected}, got {}",
            raw.len()
        )));
    }
    let packed_height = height / 2;
    let packed_width = width / 2;
    let plane_len = exact_len(packed_height, packed_width, 1)?;
    let mut packed = vec![0_u8; exact_len(packed_height, packed_width, 4)?];
    for packed_y in 0..packed_height {
        let raw_y = packed_y * 2;
        for packed_x in 0..packed_width {
            let raw_x = packed_x * 2;
            let plane_index = packed_y * packed_width + packed_x;
            let top_left = raw_y * width + raw_x;
            packed[plane_index] = raw[top_left];
            packed[plane_len + plane_index] = raw[top_left + 1];
            packed[2 * plane_len + plane_index] = raw[top_left + width];
            packed[3 * plane_len + plane_index] = raw[top_left + width + 1];
        }
    }
    Ok(packed)
}

/// Convert decoded row-major interleaved RGB8 bytes to channel-major RGB.
pub fn load_emission_rgb(pixels: &[u8], height: usize, width: usize) -> Result<Vec<u8>> {
    let expected = exact_len(height, width, 3)?;
    if pixels.len() != expected {
        return Err(PreprocessError::new(format!(
            "emission RGB byte count differs: expected {expected}, got {}",
            pixels.len()
        )));
    }
    let plane_len = exact_len(height, width, 1)?;
    let mut channels = vec![0_u8; expected];
    for pixel_index in 0..plane_len {
        let source = pixel_index * 3;
        channels[pixel_index] = pixels[source];
        channels[plane_len + pixel_index] = pixels[source + 1];
        channels[2 * plane_len + pixel_index] = pixels[source + 2];
    }
    Ok(channels)
}

/// Apply nonnegative round-half-up before an exact right shift.
pub fn round_half_up_right_shift(value: u32, shift: u32) -> Result<u32> {
    if shift == 0 {
        return Ok(value);
    }
    if shift >= u32::BITS {
        return Err(PreprocessError::new("right shift exceeds uint32 width"));
    }
    let bias = 1_u32 << (shift - 1);
    let biased = value
        .checked_add(bias)
        .ok_or_else(|| PreprocessError::new("rounding bias overflowed uint32"))?;
    Ok(biased >> shift)
}

/// Crop and exact-average channel-major uint8 values with uint32 sums.
pub fn block_mean_uint8(
    channels: &[u8],
    channel_count: usize,
    height: usize,
    width: usize,
    crop: Crop,
    output_size: usize,
) -> Result<Vec<u8>> {
    if channel_count == 0 || output_size == 0 {
        return Err(PreprocessError::new(
            "channel count and output size must be positive",
        ));
    }
    let expected = exact_len(height, width, channel_count)?;
    if channels.len() != expected {
        return Err(PreprocessError::new(format!(
            "channel-major input byte count differs: expected {expected}, got {}",
            channels.len()
        )));
    }
    ensure_crop(crop, height, width)?;
    if crop.height != crop.width || crop.height % output_size != 0 {
        return Err(PreprocessError::new(
            "square crop must divide exactly by output size",
        ));
    }
    let block = crop.height / output_size;
    let divisor = block
        .checked_mul(block)
        .ok_or_else(|| PreprocessError::new("pool divisor overflow"))?;
    if divisor == 0 || !divisor.is_power_of_two() {
        return Err(PreprocessError::new(
            "pool divisor must be a positive power of two",
        ));
    }
    let shift = divisor.trailing_zeros();
    let plane_len = exact_len(height, width, 1)?;
    let output_plane_len = exact_len(output_size, output_size, 1)?;
    let mut output = vec![0_u8; exact_len(output_size, output_size, channel_count)?];

    for channel in 0..channel_count {
        let source_plane = channel * plane_len;
        let output_plane = channel * output_plane_len;
        for output_y in 0..output_size {
            let source_y0 = crop.y0 + output_y * block;
            for output_x in 0..output_size {
                let source_x0 = crop.x0 + output_x * block;
                let mut sum = 0_u32;
                for block_y in 0..block {
                    let row = source_plane + (source_y0 + block_y) * width + source_x0;
                    for block_x in 0..block {
                        sum = sum.checked_add(u32::from(channels[row + block_x])).ok_or_else(
                            || PreprocessError::new("block sum overflowed uint32"),
                        )?;
                    }
                }
                let rounded = round_half_up_right_shift(sum, shift)?;
                let byte = u8::try_from(rounded).map_err(|_| {
                    PreprocessError::new("rounded block average overflowed uint8")
                })?;
                output[output_plane + output_y * output_size + output_x] = byte;
            }
        }
    }
    Ok(output)
}

/// Return the fixed candidate camera and emission tensors.
/// Identical to `preprocess_candidate` but takes the camera already RGGB-packed
/// (4 x PACKED_HEIGHT x PACKED_WIDTH), so a caller that needs the packed planes for
/// other work packs exactly once. Byte-identical output by construction: the packed
/// buffer it consumes is the same one `preprocess_candidate` builds internally.
pub fn preprocess_candidate_packed(
    camera_packed: &[u8],
    emission_rgb: &[u8],
    output_size: usize,
) -> Result<(Vec<u8>, Vec<u8>)> {
    if !SUPPORTED_OUTPUT_SIZES.contains(&output_size) {
        return Err(PreprocessError::new(format!(
            "unsupported candidate output size: {output_size}"
        )));
    }
    if camera_packed.len() != CAMERA_CHANNELS * PACKED_HEIGHT * PACKED_WIDTH {
        return Err(PreprocessError::new(format!(
            "packed camera byte count differs: got {}",
            camera_packed.len()
        )));
    }
    if emission_rgb.len() != EMISSION_BYTES {
        return Err(PreprocessError::new(format!(
            "emission RGB byte count differs: expected {EMISSION_BYTES}, got {}",
            emission_rgb.len()
        )));
    }
    let emission = load_emission_rgb(emission_rgb, EMISSION_HEIGHT, EMISSION_WIDTH)?;
    let reduced_camera = block_mean_uint8(
        camera_packed,
        CAMERA_CHANNELS,
        PACKED_HEIGHT,
        PACKED_WIDTH,
        CAMERA_CROP,
        output_size,
    )?;
    let reduced_emission = block_mean_uint8(
        &emission,
        EMISSION_CHANNELS,
        EMISSION_HEIGHT,
        EMISSION_WIDTH,
        EMISSION_CROP,
        output_size,
    )?;
    Ok((reduced_camera, reduced_emission))
}

pub fn preprocess_candidate(
    raw_bayer: &[u8],
    emission_rgb: &[u8],
    output_size: usize,
) -> Result<(Vec<u8>, Vec<u8>)> {
    if !SUPPORTED_OUTPUT_SIZES.contains(&output_size) {
        return Err(PreprocessError::new(format!(
            "unsupported candidate output size: {output_size}"
        )));
    }
    if raw_bayer.len() != RAW_BYTES {
        return Err(PreprocessError::new(format!(
            "raw Bayer byte count differs: expected {RAW_BYTES}, got {}",
            raw_bayer.len()
        )));
    }
    if emission_rgb.len() != EMISSION_BYTES {
        return Err(PreprocessError::new(format!(
            "emission RGB byte count differs: expected {EMISSION_BYTES}, got {}",
            emission_rgb.len()
        )));
    }
    let camera = pack_bayer_rggb(raw_bayer, RAW_HEIGHT, RAW_WIDTH)?;
    let emission = load_emission_rgb(emission_rgb, EMISSION_HEIGHT, EMISSION_WIDTH)?;
    let reduced_camera = block_mean_uint8(
        &camera,
        CAMERA_CHANNELS,
        PACKED_HEIGHT,
        PACKED_WIDTH,
        CAMERA_CROP,
        output_size,
    )?;
    let reduced_emission = block_mean_uint8(
        &emission,
        EMISSION_CHANNELS,
        EMISSION_HEIGHT,
        EMISSION_WIDTH,
        EMISSION_CROP,
        output_size,
    )?;
    Ok((reduced_camera, reduced_emission))
}

/// Canonical camera-then-emission channel-major output bytes.
pub fn output_bytes(camera: &[u8], emission: &[u8], output_size: usize) -> Result<Vec<u8>> {
    let expected_camera = exact_len(output_size, output_size, CAMERA_CHANNELS)?;
    let expected_emission = exact_len(output_size, output_size, EMISSION_CHANNELS)?;
    if camera.len() != expected_camera {
        return Err(PreprocessError::new("camera output geometry differs"));
    }
    if emission.len() != expected_emission {
        return Err(PreprocessError::new("emission output geometry differs"));
    }
    let mut output = Vec::with_capacity(expected_camera + expected_emission);
    output.extend_from_slice(camera);
    output.extend_from_slice(emission);
    Ok(output)
}

const SHA256_INITIAL: [u32; 8] = [
    0x6a09e667,
    0xbb67ae85,
    0x3c6ef372,
    0xa54ff53a,
    0x510e527f,
    0x9b05688c,
    0x1f83d9ab,
    0x5be0cd19,
];

const SHA256_ROUND: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1,
    0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
    0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786,
    0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147,
    0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
    0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
    0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a,
    0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
    0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

/// Standard-library SHA-256 used only for the CLI receipt summary.
pub fn sha256(data: &[u8]) -> [u8; 32] {
    let bit_length = (data.len() as u64).wrapping_mul(8);
    let mut padded = data.to_vec();
    padded.push(0x80);
    while padded.len() % 64 != 56 {
        padded.push(0);
    }
    padded.extend_from_slice(&bit_length.to_be_bytes());

    let mut state = SHA256_INITIAL;
    for block in padded.chunks_exact(64) {
        let mut schedule = [0_u32; 64];
        for (index, word) in block.chunks_exact(4).enumerate() {
            schedule[index] = u32::from_be_bytes([word[0], word[1], word[2], word[3]]);
        }
        for index in 16..64 {
            let s0 = schedule[index - 15].rotate_right(7)
                ^ schedule[index - 15].rotate_right(18)
                ^ (schedule[index - 15] >> 3);
            let s1 = schedule[index - 2].rotate_right(17)
                ^ schedule[index - 2].rotate_right(19)
                ^ (schedule[index - 2] >> 10);
            schedule[index] = schedule[index - 16]
                .wrapping_add(s0)
                .wrapping_add(schedule[index - 7])
                .wrapping_add(s1);
        }

        let mut a = state[0];
        let mut b = state[1];
        let mut c = state[2];
        let mut d = state[3];
        let mut e = state[4];
        let mut f = state[5];
        let mut g = state[6];
        let mut h = state[7];

        for index in 0..64 {
            let upper_e = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let choice = (e & f) ^ ((!e) & g);
            let first = h
                .wrapping_add(upper_e)
                .wrapping_add(choice)
                .wrapping_add(SHA256_ROUND[index])
                .wrapping_add(schedule[index]);
            let upper_a = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let majority = (a & b) ^ (a & c) ^ (b & c);
            let second = upper_a.wrapping_add(majority);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(first);
            d = c;
            c = b;
            b = a;
            a = first.wrapping_add(second);
        }

        state[0] = state[0].wrapping_add(a);
        state[1] = state[1].wrapping_add(b);
        state[2] = state[2].wrapping_add(c);
        state[3] = state[3].wrapping_add(d);
        state[4] = state[4].wrapping_add(e);
        state[5] = state[5].wrapping_add(f);
        state[6] = state[6].wrapping_add(g);
        state[7] = state[7].wrapping_add(h);
    }

    let mut digest = [0_u8; 32];
    for (index, word) in state.iter().enumerate() {
        digest[index * 4..index * 4 + 4].copy_from_slice(&word.to_be_bytes());
    }
    digest
}

pub fn sha256_hex(data: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let digest = sha256(data);
    let mut text = String::with_capacity(64);
    for byte in digest {
        text.push(char::from(HEX[usize::from(byte >> 4)]));
        text.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    text
}

pub fn spec_sha256() -> String {
    sha256_hex(SPEC_CANONICAL_JSON.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rggb_channel_order_is_exact() {
        let raw: Vec<u8> = (1..=16).collect();
        let packed = pack_bayer_rggb(&raw, 4, 4).unwrap();
        assert_eq!(
            packed,
            vec![1, 3, 9, 11, 2, 4, 10, 12, 5, 7, 13, 15, 6, 8, 14, 16]
        );
    }

    #[test]
    fn emission_rgb_becomes_channel_major() {
        let channels = load_emission_rgb(&[1, 2, 3, 4, 5, 6], 1, 2).unwrap();
        assert_eq!(channels, vec![1, 4, 2, 5, 3, 6]);
    }

    #[test]
    fn half_up_rounding_matches_candidate() {
        let reduced = block_mean_uint8(
            &[0, 0, 0, 2],
            1,
            2,
            2,
            Crop {
                y0: 0,
                x0: 0,
                height: 2,
                width: 2,
            },
            1,
        )
        .unwrap();
        assert_eq!(reduced, vec![1]);
        assert_eq!(round_half_up_right_shift(6, 2).unwrap(), 2);
    }

    #[test]
    fn constant_channels_survive_primary_and_diagnostic_reduction() {
        let plane = 1_024 * 1_024;
        let mut channels = Vec::with_capacity(4 * plane);
        for value in [0_u8, 1, 127, 255] {
            channels.extend(std::iter::repeat_n(value, plane));
        }
        let crop = Crop {
            y0: 0,
            x0: 0,
            height: 1_024,
            width: 1_024,
        };
        for output_size in [32, 64, 256] {
            let reduced =
                block_mean_uint8(&channels, 4, 1_024, 1_024, crop, output_size).unwrap();
            let output_plane = output_size * output_size;
            assert_eq!(reduced[0], 0);
            assert_eq!(reduced[output_plane], 1);
            assert_eq!(reduced[2 * output_plane], 127);
            assert_eq!(reduced[3 * output_plane], 255);
        }
    }

    #[test]
    fn canonical_output_is_camera_then_emission() {
        let camera: Vec<u8> = (0..16).collect();
        let emission: Vec<u8> = (100..112).collect();
        let output = output_bytes(&camera, &emission, 2).unwrap();
        assert_eq!(&output[..16], camera.as_slice());
        assert_eq!(&output[16..], emission.as_slice());
    }

    #[test]
    fn sha256_matches_standard_vectors_and_canonical_config() {
        assert_eq!(
            sha256_hex(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        assert_eq!(
            spec_sha256(),
            "6345dc412201aacd2e33626159812e6a8bf839142b403e15f4de1c781982bd6d"
        );
        assert_eq!(SPEC_STATUS, "candidate_not_frozen");
        assert_eq!(PRIMARY_OUTPUT_SIZE, 256);
        assert_eq!(DIAGNOSTIC_OUTPUT_SIZES, &[32, 64]);
        assert_eq!(SUPPORTED_OUTPUT_SIZES, &[32, 64, 256]);
    }

    #[test]
    fn wrong_lengths_and_pool_divisors_fail_closed() {
        assert!(pack_bayer_rggb(b"short", 4, 4).is_err());
        assert!(load_emission_rgb(b"short", 1, 2).is_err());
        let bad_pool = block_mean_uint8(
            &[0; 36],
            1,
            6,
            6,
            Crop {
                y0: 0,
                x0: 0,
                height: 6,
                width: 6,
            },
            2,
        );
        assert!(bad_pool.is_err());
    }
}
