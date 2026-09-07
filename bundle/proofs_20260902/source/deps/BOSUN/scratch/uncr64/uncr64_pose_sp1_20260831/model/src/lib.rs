//! Exact integer inference for the frozen uncropped-r64 pose PTQ blob.
//!
//! Parses the fixed 13,312-byte blob (magic ZBU64Q1\0) and reproduces the audited Python
//! candidate exactly: uint8 input quantization (multiplier 137973719008, shift 30,
//! signed nearest-even, clamp +-32767), five stride-2 3x3 convolutions with ReLU, a
//! requantized 11-class 1x1 head on the 2x2 field, and the exact int64 SUM per class with
//! the first-maximum verdict. Artifact binding: 87f6cd241f918523275469786605176c4bf3ff16fe2b734658e8a6731c50b937.

use std::fmt;

pub const BLOB_BYTES: usize = 13312;
pub const BLOB_MAGIC: &[u8; 8] = b"ZBU64Q1\0";
pub const BLOB_SHA256: &str = "c95b00724f801e9ea8b841c060ac23d2b4267c77691c1755cc6a111550197ed7";
pub const INTEGER_ARTIFACT_SHA256: &str = "87f6cd241f918523275469786605176c4bf3ff16fe2b734658e8a6731c50b937";
pub const UNCROPPED_SPEC_SHA256: &str = "a3985f693e1c017037fda13e7b6d15f8896c4d195e02a6298f198552560e8e08";

pub const FULL_SIDE: usize = 256;
pub const REDUCED_SIDE: usize = 64;
pub const CAMERA_CHANNELS: usize = 4;
pub const CLASS_COUNT: usize = 11;
pub const HEAD_SIDE: usize = 2;
pub const CAMERA_PRIMARY_BYTES: usize = CAMERA_CHANNELS * FULL_SIDE * FULL_SIDE;
pub const CAMERA_REDUCED_BYTES: usize = CAMERA_CHANNELS * REDUCED_SIDE * REDUCED_SIDE;

const ACTIVATION_QMIN: i64 = -32_767;
const ACTIVATION_QMAX: i64 = 32_767;
const REQUANT_SHIFT: u32 = 30;
const INPUT_MULTIPLIER: i64 = 137973719008;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModelError {
    BlobLength,
    BlobMagic,
    InputLength,
    ArithmeticOverflow,
}

impl fmt::Display for ModelError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::BlobLength => "uncr64 pose blob length differs",
            Self::BlobMagic => "uncr64 pose blob magic differs",
            Self::InputLength => "input tensor length differs",
            Self::ArithmeticOverflow => "integer inference overflowed i64",
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
    weight: (usize, usize),
    bias: (usize, usize),
    multiplier: (usize, usize),
}

const LAYERS: [LayerLayout; 5] = [
    LayerLayout {
        input_channels: 4,
        output_channels: 8,
        input_side: 64,
        output_side: 32,
        weight: (8, 296),
        bias: (296, 360),
        multiplier: (360, 424),
    },
    LayerLayout {
        input_channels: 8,
        output_channels: 12,
        input_side: 32,
        output_side: 16,
        weight: (424, 1288),
        bias: (1288, 1384),
        multiplier: (1384, 1480),
    },
    LayerLayout {
        input_channels: 12,
        output_channels: 16,
        input_side: 16,
        output_side: 8,
        weight: (1480, 3208),
        bias: (3208, 3336),
        multiplier: (3336, 3464),
    },
    LayerLayout {
        input_channels: 16,
        output_channels: 24,
        input_side: 8,
        output_side: 4,
        weight: (3464, 6920),
        bias: (6920, 7112),
        multiplier: (7112, 7304),
    },
    LayerLayout {
        input_channels: 24,
        output_channels: 24,
        input_side: 4,
        output_side: 2,
        weight: (7304, 12488),
        bias: (12488, 12680),
        multiplier: (12680, 12872),
    },
];

const HEAD_WEIGHT: (usize, usize) = (12872, 13136);
const HEAD_BIAS: (usize, usize) = (13136, 13224);
const HEAD_MULTIPLIER: (usize, usize) = (13224, 13312);

pub struct PoseOutput {
    pub logit_sums: [i64; CLASS_COUNT],
    pub verdict: u8,
    pub head_saturation: u32,
}

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

    pub fn run_reduced(&self, camera: &[u8]) -> Result<PoseOutput, ModelError> {
        if camera.len() != CAMERA_REDUCED_BYTES {
            return Err(ModelError::InputLength);
        }
        let mut value = Vec::with_capacity(CAMERA_REDUCED_BYTES);
        for byte in camera {
            let product = (*byte as i64)
                .checked_mul(INPUT_MULTIPLIER)
                .ok_or(ModelError::ArithmeticOverflow)?;
            let rounded = round_shift_nearest_even(product)?;
            value.push(rounded.clamp(ACTIVATION_QMIN, ACTIVATION_QMAX) as i16);
        }
        for layout in LAYERS {
            value = self.run_block(&value, layout)?;
        }
        self.run_head(&value)
    }

    pub fn run_primary(&self, camera: &[u8]) -> Result<PoseOutput, ModelError> {
        if camera.len() != CAMERA_PRIMARY_BYTES {
            return Err(ModelError::InputLength);
        }
        let reduced = reduce_primary_4x4(camera, CAMERA_CHANNELS)?;
        self.run_reduced(&reduced)
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
                            let padded_y = output_y * 2 + kernel_y;
                            if padded_y == 0 || padded_y > layout.input_side {
                                continue;
                            }
                            let input_y = padded_y - 1;
                            for kernel_x in 0..3 {
                                let padded_x = output_x * 2 + kernel_x;
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

    fn run_head(&self, input: &[i16]) -> Result<PoseOutput, ModelError> {
        const IN_CH: usize = 24;
        if input.len() != IN_CH * HEAD_SIDE * HEAD_SIDE {
            return Err(ModelError::InputLength);
        }
        let weights = &self.blob[HEAD_WEIGHT.0..HEAD_WEIGHT.1];
        let biases = &self.blob[HEAD_BIAS.0..HEAD_BIAS.1];
        let multipliers = &self.blob[HEAD_MULTIPLIER.0..HEAD_MULTIPLIER.1];
        let mut sums = [0_i64; CLASS_COUNT];
        let mut saturation = 0_u32;
        for class in 0..CLASS_COUNT {
            let bias = read_i64_le(biases, class);
            let multiplier = read_i64_le(multipliers, class);
            for y in 0..HEAD_SIDE {
                for x in 0..HEAD_SIDE {
                    let mut accumulator = bias;
                    for channel in 0..IN_CH {
                        let index = (channel * HEAD_SIDE + y) * HEAD_SIDE + x;
                        let product = (input[index] as i64)
                            .checked_mul(weights[class * IN_CH + channel] as i8 as i64)
                            .ok_or(ModelError::ArithmeticOverflow)?;
                        accumulator = accumulator
                            .checked_add(product)
                            .ok_or(ModelError::ArithmeticOverflow)?;
                    }
                    let product = accumulator
                        .checked_mul(multiplier)
                        .ok_or(ModelError::ArithmeticOverflow)?;
                    let rounded = round_shift_nearest_even(product)?;
                    if rounded < ACTIVATION_QMIN || rounded > ACTIVATION_QMAX {
                        saturation += 1;
                    }
                    let clipped = rounded.clamp(ACTIVATION_QMIN, ACTIVATION_QMAX);
                    sums[class] = sums[class]
                        .checked_add(clipped)
                        .ok_or(ModelError::ArithmeticOverflow)?;
                }
            }
        }
        let mut best = sums[0];
        let mut verdict = 0_u8;
        for (class, value) in sums.iter().enumerate().skip(1) {
            if *value > best {
                best = *value;
                verdict = class as u8;
            }
        }
        Ok(PoseOutput {
            logit_sums: sums,
            verdict,
            head_saturation: saturation,
        })
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

/// Published 256->64 reduction: 4x4 block mean with ties-to-even (unsigned).
pub fn reduce_primary_4x4(input: &[u8], channels: usize) -> Result<Vec<u8>, ModelError> {
    if channels == 0 || input.len() != channels * FULL_SIDE * FULL_SIDE {
        return Err(ModelError::InputLength);
    }
    let mut output = vec![0_u8; channels * REDUCED_SIDE * REDUCED_SIDE];
    for channel in 0..channels {
        for output_y in 0..REDUCED_SIDE {
            for output_x in 0..REDUCED_SIDE {
                let mut sum = 0_u32;
                for tile_y in 0..4 {
                    for tile_x in 0..4 {
                        let y = output_y * 4 + tile_y;
                        let x = output_x * 4 + tile_x;
                        sum += input[(channel * FULL_SIDE + y) * FULL_SIDE + x] as u32;
                    }
                }
                let quotient = sum / 16;
                let remainder = sum % 16;
                let increment = remainder > 8 || (remainder == 8 && quotient & 1 == 1);
                output[(channel * REDUCED_SIDE + output_y) * REDUCED_SIDE + output_x] =
                    (quotient + u32::from(increment)) as u8;
            }
        }
    }
    Ok(output)
}
