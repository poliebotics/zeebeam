//! Exact integer inference for the frozen ZeeBeam trained-r32 PTQ-v2 blob.
//!
//! This crate parses the fixed 90,552-byte interchange blob and reproduces
//! the Python candidate's uint8 input quantization, five convolutions, ReLU,
//! signed nearest-even fixed-shift requantization, and 2-by-2 score sum.

use std::fmt;

pub const BLOB_BYTES: usize = 90_552;
pub const BLOB_MAGIC: &[u8; 8] = b"ZBR32Q2\0";
pub const BLOB_SHA256: &str = "0188b5c0f4ea08e81940aef1a8cb303c00b24e7f8b7730a64bf5080d4030353a";
pub const BLOB_MANIFEST_SHA256: &str =
    "0a2cbe6992e1f1f0911b7ab6e09184bc96408dda0684657ec6c0a21125305290";
pub const PARITY_REPORT_SHA256: &str =
    "784bd57327fbec4a1ea509025b795c63ca8789bb6d6899fe6a42b831d9cdc347";
pub const SHA256SUMS_SHA256: &str =
    "eb3b85a2cfaa006b0f921f07ff69ca18387a7073ed6559084262e0a69e19dc31";
pub const MANIFEST_SELF_SHA256: &str =
    "8af5d93232fb9718fca3e50df68c23f65d4d9a1517df8c65031f5c48df5ad04b";
pub const REPORT_SELF_SHA256: &str =
    "54b3f2a4d895ba97bd7397f80186698a3aed00b8ff701a179454f39b43976140";
pub const MODEL_SPEC_SHA256: &str =
    "907879d2edc5b902dd79c57a491bb315f9003e6409602e5fa8c6f634e4e53fe2";
pub const CHECKPOINT_SHA256: &str =
    "95e669745a3809391f4db9da75adf54d2a597421b4a66fba0217f918ffa706f7";
pub const FINAL_MODEL_STATE_SHA256: &str =
    "f754be088b7253ce6d2e48de9fc429167e27df28ba05ec514e86a19578110d84";
pub const PTQ_SOURCE_STATE_SHA256: &str =
    "b00a31cd7c3d4eb36d2f3e57ec9605b0b3605264df91ad498378a1f8b6d8828e";
pub const INTEGER_ARTIFACT_SHA256: &str =
    "6036631cd27dab369d4494c5f225c13bbdd9f640a9068dde399f98e4a854cfcf";

pub const FULL_SIDE: usize = 256;
pub const REDUCED_SIDE: usize = 32;
pub const CAMERA_CHANNELS: usize = 4;
pub const EMISSION_CHANNELS: usize = 3;
pub const TOTAL_CHANNELS: usize = CAMERA_CHANNELS + EMISSION_CHANNELS;
pub const CAMERA_REDUCED_BYTES: usize = CAMERA_CHANNELS * REDUCED_SIDE * REDUCED_SIDE;
pub const EMISSION_REDUCED_BYTES: usize = EMISSION_CHANNELS * REDUCED_SIDE * REDUCED_SIDE;
pub const REDUCED_PAIR_BYTES: usize = CAMERA_REDUCED_BYTES + EMISSION_REDUCED_BYTES;
pub const CAMERA_PRIMARY_BYTES: usize = CAMERA_CHANNELS * FULL_SIDE * FULL_SIDE;
pub const EMISSION_PRIMARY_BYTES: usize = EMISSION_CHANNELS * FULL_SIDE * FULL_SIDE;
pub const PRIMARY_PAIR_BYTES: usize = CAMERA_PRIMARY_BYTES + EMISSION_PRIMARY_BYTES;
pub const SCORE_ABS_BOUND: i64 = 618_556_948;

const ACTIVATION_QMIN: i64 = -32_767;
const ACTIVATION_QMAX: i64 = 32_767;
const REQUANT_SHIFT: u32 = 30;
const INPUT_MULTIPLIER: i64 = 137_973_719_008;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModelError {
    BlobLength,
    BlobMagic,
    InputLength,
    ArithmeticOverflow,
    ScoreBound,
}

impl fmt::Display for ModelError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::BlobLength => "PTQ-v2 blob length differs",
            Self::BlobMagic => "PTQ-v2 blob magic differs",
            Self::InputLength => "input tensor length differs",
            Self::ArithmeticOverflow => "integer inference overflowed i64",
            Self::ScoreBound => "integer score escaped its frozen bound",
        })
    }
}

impl std::error::Error for ModelError {}

#[derive(Clone, Copy)]
struct LayerLayout {
    input_channels: usize,
    output_channels: usize,
    input_side: usize,
    output_side: usize,
    stride: usize,
    weight: (usize, usize),
    bias: (usize, usize),
    multiplier: (usize, usize),
}

const LAYERS: [LayerLayout; 5] = [
    LayerLayout {
        input_channels: 7,
        output_channels: 24,
        input_side: 32,
        output_side: 32,
        stride: 1,
        weight: (8, 1_520),
        bias: (1_520, 1_712),
        multiplier: (1_712, 1_904),
    },
    LayerLayout {
        input_channels: 24,
        output_channels: 32,
        input_side: 32,
        output_side: 16,
        stride: 2,
        weight: (1_904, 8_816),
        bias: (8_816, 9_072),
        multiplier: (9_072, 9_328),
    },
    LayerLayout {
        input_channels: 32,
        output_channels: 48,
        input_side: 16,
        output_side: 8,
        stride: 2,
        weight: (9_328, 23_152),
        bias: (23_152, 23_536),
        multiplier: (23_536, 23_920),
    },
    LayerLayout {
        input_channels: 48,
        output_channels: 64,
        input_side: 8,
        output_side: 4,
        stride: 2,
        weight: (23_920, 51_568),
        bias: (51_568, 52_080),
        multiplier: (52_080, 52_592),
    },
    LayerLayout {
        input_channels: 64,
        output_channels: 64,
        input_side: 4,
        output_side: 2,
        stride: 2,
        weight: (52_592, 89_456),
        bias: (89_456, 89_968),
        multiplier: (89_968, 90_480),
    },
];

pub struct BlobModel<'a> {
    blob: &'a [u8],
}

impl<'a> BlobModel<'a> {
    pub fn parse(blob: &'a [u8]) -> Result<Self, ModelError> {
        if blob.len() != BLOB_BYTES {
            return Err(ModelError::BlobLength);
        }
        if &blob[..BLOB_MAGIC.len()] != BLOB_MAGIC {
            return Err(ModelError::BlobMagic);
        }
        Ok(Self { blob })
    }

    pub fn score_reduced_pair(&self, camera: &[u8], emission: &[u8]) -> Result<i64, ModelError> {
        if camera.len() != CAMERA_REDUCED_BYTES || emission.len() != EMISSION_REDUCED_BYTES {
            return Err(ModelError::InputLength);
        }
        let mut value = Vec::with_capacity(REDUCED_PAIR_BYTES);
        for byte in camera.iter().chain(emission.iter()) {
            let product = (*byte as i64)
                .checked_mul(INPUT_MULTIPLIER)
                .ok_or(ModelError::ArithmeticOverflow)?;
            value.push(round_shift_nearest_even(product)? as i16);
        }
        for layout in LAYERS {
            value = self.run_block(&value, layout)?;
        }
        self.run_head(&value)
    }

    pub fn score_primary_pair(&self, camera: &[u8], emission: &[u8]) -> Result<i64, ModelError> {
        if camera.len() != CAMERA_PRIMARY_BYTES || emission.len() != EMISSION_PRIMARY_BYTES {
            return Err(ModelError::InputLength);
        }
        let reduced_camera = reduce_primary_8x8(camera, CAMERA_CHANNELS)?;
        let reduced_emission = reduce_primary_8x8(emission, EMISSION_CHANNELS)?;
        self.score_reduced_pair(&reduced_camera, &reduced_emission)
    }

    fn run_block(&self, input: &[i16], layout: LayerLayout) -> Result<Vec<i16>, ModelError> {
        if input.len() != layout.input_channels * layout.input_side * layout.input_side {
            return Err(ModelError::InputLength);
        }
        let weights = &self.blob[layout.weight.0..layout.weight.1];
        let biases = &self.blob[layout.bias.0..layout.bias.1];
        let multipliers = &self.blob[layout.multiplier.0..layout.multiplier.1];
        let mut output =
            vec![0_i16; layout.output_channels * layout.output_side * layout.output_side];
        for output_channel in 0..layout.output_channels {
            let bias = read_i64_le(biases, output_channel);
            let multiplier = read_i64_le(multipliers, output_channel);
            for output_y in 0..layout.output_side {
                for output_x in 0..layout.output_side {
                    let mut accumulator = bias;
                    for input_channel in 0..layout.input_channels {
                        for kernel_y in 0..3 {
                            let padded_y = output_y * layout.stride + kernel_y;
                            if padded_y == 0 || padded_y > layout.input_side {
                                continue;
                            }
                            let input_y = padded_y - 1;
                            for kernel_x in 0..3 {
                                let padded_x = output_x * layout.stride + kernel_x;
                                if padded_x == 0 || padded_x > layout.input_side {
                                    continue;
                                }
                                let input_x = padded_x - 1;
                                let input_index = (input_channel * layout.input_side + input_y)
                                    * layout.input_side
                                    + input_x;
                                let weight_index = (((output_channel * layout.input_channels
                                    + input_channel)
                                    * 3
                                    + kernel_y)
                                    * 3)
                                    + kernel_x;
                                let product = (input[input_index] as i64)
                                    .checked_mul(weights[weight_index] as i8 as i64)
                                    .ok_or(ModelError::ArithmeticOverflow)?;
                                accumulator = accumulator
                                    .checked_add(product)
                                    .ok_or(ModelError::ArithmeticOverflow)?;
                            }
                        }
                    }
                    let product = accumulator
                        .checked_mul(multiplier)
                        .ok_or(ModelError::ArithmeticOverflow)?;
                    let rounded = round_shift_nearest_even(product)?.max(0);
                    let clipped = rounded.clamp(ACTIVATION_QMIN, ACTIVATION_QMAX) as i16;
                    let output_index = (output_channel * layout.output_side + output_y)
                        * layout.output_side
                        + output_x;
                    output[output_index] = clipped;
                }
            }
        }
        Ok(output)
    }

    fn run_head(&self, input: &[i16]) -> Result<i64, ModelError> {
        const CHANNELS: usize = 64;
        const SIDE: usize = 2;
        if input.len() != CHANNELS * SIDE * SIDE {
            return Err(ModelError::InputLength);
        }
        let weights = &self.blob[90_480..90_544];
        let bias = read_i64_le(&self.blob[90_544..90_552], 0);
        let mut score = 0_i64;
        for y in 0..SIDE {
            for x in 0..SIDE {
                let mut accumulator = bias;
                for channel in 0..CHANNELS {
                    let index = (channel * SIDE + y) * SIDE + x;
                    let product = (input[index] as i64)
                        .checked_mul(weights[channel] as i8 as i64)
                        .ok_or(ModelError::ArithmeticOverflow)?;
                    accumulator = accumulator
                        .checked_add(product)
                        .ok_or(ModelError::ArithmeticOverflow)?;
                }
                score = score
                    .checked_add(accumulator)
                    .ok_or(ModelError::ArithmeticOverflow)?;
            }
        }
        if score.unsigned_abs() > SCORE_ABS_BOUND as u64 {
            return Err(ModelError::ScoreBound);
        }
        Ok(score)
    }
}

fn read_i64_le(bytes: &[u8], index: usize) -> i64 {
    let start = index * 8;
    i64::from_le_bytes(
        bytes[start..start + 8]
            .try_into()
            .expect("fixed i64 segment"),
    )
}

fn round_shift_nearest_even(value: i64) -> Result<i64, ModelError> {
    if value == i64::MIN {
        return Err(ModelError::ArithmeticOverflow);
    }
    let negative = value < 0;
    let magnitude = value.unsigned_abs();
    let denominator = 1_u64 << REQUANT_SHIFT;
    let quotient = magnitude / denominator;
    let remainder = magnitude - quotient * denominator;
    let half = denominator >> 1;
    let increment = remainder > half || (remainder == half && quotient & 1 == 1);
    let rounded = quotient + u64::from(increment);
    let signed = i64::try_from(rounded).map_err(|_| ModelError::ArithmeticOverflow)?;
    Ok(if negative { -signed } else { signed })
}

pub fn reduce_primary_8x8(input: &[u8], channels: usize) -> Result<Vec<u8>, ModelError> {
    if channels == 0 || input.len() != channels * FULL_SIDE * FULL_SIDE {
        return Err(ModelError::InputLength);
    }
    let mut output = vec![0_u8; channels * REDUCED_SIDE * REDUCED_SIDE];
    for channel in 0..channels {
        for output_y in 0..REDUCED_SIDE {
            for output_x in 0..REDUCED_SIDE {
                let mut sum = 0_u32;
                for tile_y in 0..8 {
                    for tile_x in 0..8 {
                        let y = output_y * 8 + tile_y;
                        let x = output_x * 8 + tile_x;
                        sum += input[(channel * FULL_SIDE + y) * FULL_SIDE + x] as u32;
                    }
                }
                let quotient = sum / 64;
                let remainder = sum % 64;
                let increment = remainder > 32 || (remainder == 32 && quotient & 1 == 1);
                output[(channel * REDUCED_SIDE + output_y) * REDUCED_SIDE + output_x] =
                    (quotient + u32::from(increment)) as u8;
            }
        }
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signed_nearest_even_covers_ties() {
        let half = 1_i64 << 29;
        assert_eq!(round_shift_nearest_even(half).unwrap(), 0);
        assert_eq!(round_shift_nearest_even(3 * half).unwrap(), 2);
        assert_eq!(round_shift_nearest_even(-half).unwrap(), 0);
        assert_eq!(round_shift_nearest_even(-3 * half).unwrap(), -2);
    }

    #[test]
    fn parser_rejects_length_and_magic_changes() {
        let mut blob = vec![0_u8; BLOB_BYTES];
        blob[..8].copy_from_slice(BLOB_MAGIC);
        assert!(BlobModel::parse(&blob).is_ok());
        assert_eq!(
            BlobModel::parse(&blob[..BLOB_BYTES - 1]).err(),
            Some(ModelError::BlobLength)
        );
        blob[0] ^= 1;
        assert_eq!(BlobModel::parse(&blob).err(), Some(ModelError::BlobMagic));
    }
}
