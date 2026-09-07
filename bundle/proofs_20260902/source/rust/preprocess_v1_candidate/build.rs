use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Copy)]
struct CropValues {
    y0: usize,
    x0: usize,
    height: usize,
    width: usize,
}

fn value_for<'a>(text: &'a str, key: &str) -> Result<&'a str, String> {
    let needle = format!("\"{key}\":");
    let mut matches = text.match_indices(&needle);
    let (start, _) = matches
        .next()
        .ok_or_else(|| format!("canonical config lacks {key}"))?;
    if matches.next().is_some() {
        return Err(format!("canonical config repeats {key}"));
    }
    let tail = &text[start + needle.len()..];
    let first = *tail
        .as_bytes()
        .first()
        .ok_or_else(|| format!("canonical config has empty value for {key}"))?;
    let end = match first {
        b'"' => {
            let mut closing = None;
            for (index, byte) in tail.as_bytes().iter().copied().enumerate().skip(1) {
                if byte == b'\\' {
                    return Err(format!("canonical config uses an escaped string for {key}"));
                }
                if byte == b'"' {
                    closing = Some(index + 1);
                    break;
                }
            }
            closing.ok_or_else(|| format!("canonical config has unterminated string for {key}"))?
        }
        b'[' | b'{' => {
            let closing = if first == b'[' { b']' } else { b'}' };
            let mut depth = 0_usize;
            let mut in_string = false;
            let mut end = None;
            for (index, byte) in tail.as_bytes().iter().copied().enumerate() {
                if byte == b'\\' {
                    return Err(format!("canonical config uses an escape in {key}"));
                }
                if byte == b'"' {
                    in_string = !in_string;
                    continue;
                }
                if in_string {
                    continue;
                }
                if byte == first {
                    depth += 1;
                } else if byte == closing {
                    depth = depth
                        .checked_sub(1)
                        .ok_or_else(|| format!("canonical config has malformed {key}"))?;
                    if depth == 0 {
                        end = Some(index + 1);
                        break;
                    }
                }
            }
            if in_string {
                return Err(format!("canonical config has unterminated string in {key}"));
            }
            end.ok_or_else(|| format!("canonical config has unterminated value for {key}"))?
        }
        _ => tail
            .find([',', '}', ']', '\n'])
            .unwrap_or(tail.len()),
    };
    Ok(&tail[..end])
}

fn parse_quoted(value: &str, where_: &str) -> Result<String, String> {
    if value.len() < 2 || !value.starts_with('"') || !value.ends_with('"') {
        return Err(format!("canonical config {where_} must be a string"));
    }
    let inner = &value[1..value.len() - 1];
    if inner.is_empty()
        || !inner
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'/'))
    {
        return Err(format!("canonical config {where_} is not a safe ASCII token"));
    }
    Ok(inner.to_owned())
}

fn string_value(text: &str, key: &str) -> Result<String, String> {
    parse_quoted(value_for(text, key)?, key)
}

fn usize_value(text: &str, key: &str) -> Result<usize, String> {
    let value = value_for(text, key)?;
    if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(format!("canonical config {key} must be a non-negative integer"));
    }
    value
        .parse::<usize>()
        .map_err(|_| format!("canonical config {key} exceeds usize"))
}

fn string_array(text: &str, key: &str) -> Result<Vec<String>, String> {
    let value = value_for(text, key)?;
    let inner = value
        .strip_prefix('[')
        .and_then(|item| item.strip_suffix(']'))
        .ok_or_else(|| format!("canonical config {key} must be an array"))?;
    if inner.is_empty() {
        return Err(format!("canonical config {key} may not be empty"));
    }
    inner
        .split(',')
        .map(|item| parse_quoted(item, key))
        .collect()
}

fn usize_array(text: &str, key: &str) -> Result<Vec<usize>, String> {
    let value = value_for(text, key)?;
    let inner = value
        .strip_prefix('[')
        .and_then(|item| item.strip_suffix(']'))
        .ok_or_else(|| format!("canonical config {key} must be an array"))?;
    if inner.is_empty() {
        return Err(format!("canonical config {key} may not be empty"));
    }
    inner
        .split(',')
        .map(|item| {
            if !item.bytes().all(|byte| byte.is_ascii_digit()) {
                return Err(format!("canonical config {key} must contain integers"));
            }
            item.parse::<usize>()
                .map_err(|_| format!("canonical config {key} exceeds usize"))
        })
        .collect()
}

fn crop_value(text: &str, key: &str) -> Result<CropValues, String> {
    let value = value_for(text, key)?;
    Ok(CropValues {
        y0: usize_value(value, "y0")?,
        x0: usize_value(value, "x0")?,
        height: usize_value(value, "height")?,
        width: usize_value(value, "width")?,
    })
}

fn json_string_array(values: &[String]) -> String {
    let items = values
        .iter()
        .map(|value| format!("\"{value}\""))
        .collect::<Vec<_>>()
        .join(",");
    format!("[{items}]")
}

fn json_usize_array(values: &[usize]) -> String {
    let items = values
        .iter()
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join(",");
    format!("[{items}]")
}

fn json_crop(crop: CropValues) -> String {
    format!(
        "{{\"height\":{},\"width\":{},\"x0\":{},\"y0\":{}}}",
        crop.height, crop.width, crop.x0, crop.y0
    )
}

fn checked_end(start: usize, extent: usize, limit: usize, where_: &str) -> Result<(), String> {
    let end = start
        .checked_add(extent)
        .ok_or_else(|| format!("canonical config {where_} overflows"))?;
    if extent == 0 || end > limit {
        return Err(format!("canonical config {where_} exceeds its input"));
    }
    Ok(())
}

fn canonical_config_path() -> Result<PathBuf, String> {
    let manifest = PathBuf::from(
        env::var_os("CARGO_MANIFEST_DIR")
            .ok_or_else(|| "CARGO_MANIFEST_DIR is absent".to_owned())?,
    );
    let path = manifest.join("../../src/zeebeam_science/preprocess_v1.candidate.json");
    let metadata = fs::symlink_metadata(&path)
        .map_err(|error| format!("cannot stat canonical preprocessing config: {error}"))?;
    if metadata.file_type().is_symlink() || !metadata.is_file() || !(1..=64 * 1024).contains(&metadata.len()) {
        return Err(
            "canonical preprocessing config must be a bounded regular non-symlink file".to_owned(),
        );
    }
    path.canonicalize()
        .map_err(|error| format!("cannot resolve canonical preprocessing config: {error}"))
}

fn generate(spec_path: &Path) -> Result<String, String> {
    let config = fs::read_to_string(spec_path)
        .map_err(|error| format!("cannot read canonical preprocessing config: {error}"))?;

    let accumulator_dtype = string_value(&config, "accumulator_dtype")?;
    let bayer_pattern = string_value(&config, "bayer_pattern")?;
    let camera_channel_order = string_array(&config, "camera_channel_order")?;
    let camera_crop = crop_value(&config, "camera_crop")?;
    let diagnostic_output_sizes = usize_array(&config, "diagnostic_output_sizes")?;
    let emission_channel_order = string_array(&config, "emission_channel_order")?;
    let emission_crop = crop_value(&config, "emission_crop")?;
    let emission_height = usize_value(&config, "emission_height")?;
    let emission_width = usize_value(&config, "emission_width")?;
    let input_dtype = string_value(&config, "input_dtype")?;
    let mask = string_value(&config, "mask")?;
    let output_dtype = string_value(&config, "output_dtype")?;
    let packed_height = usize_value(&config, "packed_height")?;
    let packed_width = usize_value(&config, "packed_width")?;
    let padding = string_value(&config, "padding")?;
    let preprocess_id = string_value(&config, "preprocess_id")?;
    let primary_output_size = usize_value(&config, "primary_output_size")?;
    let raw_height = usize_value(&config, "raw_height")?;
    let raw_width = usize_value(&config, "raw_width")?;
    let rounding = string_value(&config, "rounding")?;
    let schema = string_value(&config, "schema")?;
    let status = string_value(&config, "status")?;
    let supported_output_sizes = usize_array(&config, "supported_output_sizes")?;
    let zero_point = usize_value(&config, "zero_point")?;

    let fields = [
        format!("\"accumulator_dtype\":\"{accumulator_dtype}\""),
        format!("\"bayer_pattern\":\"{bayer_pattern}\""),
        format!(
            "\"camera_channel_order\":{}",
            json_string_array(&camera_channel_order)
        ),
        format!("\"camera_crop\":{}", json_crop(camera_crop)),
        format!(
            "\"diagnostic_output_sizes\":{}",
            json_usize_array(&diagnostic_output_sizes)
        ),
        format!(
            "\"emission_channel_order\":{}",
            json_string_array(&emission_channel_order)
        ),
        format!("\"emission_crop\":{}", json_crop(emission_crop)),
        format!("\"emission_height\":{emission_height}"),
        format!("\"emission_width\":{emission_width}"),
        format!("\"input_dtype\":\"{input_dtype}\""),
        format!("\"mask\":\"{mask}\""),
        format!("\"output_dtype\":\"{output_dtype}\""),
        format!("\"packed_height\":{packed_height}"),
        format!("\"packed_width\":{packed_width}"),
        format!("\"padding\":\"{padding}\""),
        format!("\"preprocess_id\":\"{preprocess_id}\""),
        format!("\"primary_output_size\":{primary_output_size}"),
        format!("\"raw_height\":{raw_height}"),
        format!("\"raw_width\":{raw_width}"),
        format!("\"rounding\":\"{rounding}\""),
        format!("\"schema\":\"{schema}\""),
        format!("\"status\":\"{status}\""),
        format!(
            "\"supported_output_sizes\":{}",
            json_usize_array(&supported_output_sizes)
        ),
        format!("\"zero_point\":{zero_point}"),
    ];
    let expected = format!("{{{}}}\n", fields.join(","));
    if config != expected {
        return Err(
            "canonical preprocessing config has unknown, repeated, reordered, or non-canonical fields"
                .to_owned(),
        );
    }

    if schema != "zeebeam-preprocess/v1-candidate"
        || preprocess_id != "PREPROCESS_V1_CANDIDATE_20260823"
        || status != "candidate_not_frozen"
    {
        return Err("canonical preprocessing identity or candidate status differs".to_owned());
    }
    if bayer_pattern != "RGGB"
        || camera_channel_order != ["R", "G1", "G2", "B"]
        || emission_channel_order != ["R", "G", "B"]
    {
        return Err("canonical preprocessing channel semantics are unsupported".to_owned());
    }
    if input_dtype != "uint8"
        || accumulator_dtype != "uint32"
        || output_dtype != "uint8"
        || zero_point != 0
        || rounding != "nonnegative_round_half_up_then_right_shift"
        || padding != "none"
        || mask != "implicit_all_valid"
    {
        return Err("canonical preprocessing arithmetic semantics are unsupported".to_owned());
    }
    if raw_height.checked_mul(raw_width) != Some(24_472_000)
        || raw_height % 2 != 0
        || raw_width % 2 != 0
        || packed_height != raw_height / 2
        || packed_width != raw_width / 2
    {
        return Err("canonical preprocessing raw geometry is unsupported".to_owned());
    }
    checked_end(camera_crop.y0, camera_crop.height, packed_height, "camera crop height")?;
    checked_end(camera_crop.x0, camera_crop.width, packed_width, "camera crop width")?;
    checked_end(
        emission_crop.y0,
        emission_crop.height,
        emission_height,
        "emission crop height",
    )?;
    checked_end(
        emission_crop.x0,
        emission_crop.width,
        emission_width,
        "emission crop width",
    )?;
    if camera_crop.height != camera_crop.width
        || emission_crop.height != emission_crop.width
        || camera_crop.height != emission_crop.height
        || diagnostic_output_sizes != [32, 64]
        || primary_output_size != 256
        || supported_output_sizes != [32, 64, 256]
    {
        return Err("canonical preprocessing crop or output geometry is unsupported".to_owned());
    }
    for output_size in &supported_output_sizes {
        if camera_crop.height % output_size != 0 {
            return Err("canonical crop does not divide by an output size".to_owned());
        }
        let block = camera_crop.height / output_size;
        let divisor = block
            .checked_mul(block)
            .ok_or_else(|| "canonical pooling divisor overflows".to_owned())?;
        if !divisor.is_power_of_two() {
            return Err("canonical pooling divisor must be a power of two".to_owned());
        }
    }

    let rust_camera_order = camera_channel_order
        .iter()
        .map(|value| format!("{value:?}"))
        .collect::<Vec<_>>()
        .join(", ");
    let rust_emission_order = emission_channel_order
        .iter()
        .map(|value| format!("{value:?}"))
        .collect::<Vec<_>>()
        .join(", ");
    let rust_output_sizes = supported_output_sizes
        .iter()
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join(", ");
    let rust_diagnostic_output_sizes = diagnostic_output_sizes
        .iter()
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join(", ");
    Ok(format!(
        "// @generated by build.rs from src/zeebeam_science/preprocess_v1.candidate.json; do not edit.\n\
pub const RAW_HEIGHT: usize = {raw_height};\n\
pub const RAW_WIDTH: usize = {raw_width};\n\
pub const RAW_BYTES: usize = RAW_HEIGHT * RAW_WIDTH;\n\
pub const PACKED_HEIGHT: usize = {packed_height};\n\
pub const PACKED_WIDTH: usize = {packed_width};\n\
pub const EMISSION_HEIGHT: usize = {emission_height};\n\
pub const EMISSION_WIDTH: usize = {emission_width};\n\
pub const EMISSION_BYTES: usize = EMISSION_HEIGHT * EMISSION_WIDTH * 3;\n\
pub const CAMERA_CHANNEL_ORDER: &[&str] = &[{rust_camera_order}];\n\
pub const EMISSION_CHANNEL_ORDER: &[&str] = &[{rust_emission_order}];\n\
pub const CAMERA_CHANNELS: usize = CAMERA_CHANNEL_ORDER.len();\n\
pub const EMISSION_CHANNELS: usize = EMISSION_CHANNEL_ORDER.len();\n\
pub const PREPROCESS_ID: &str = {preprocess_id:?};\n\
pub const SPEC_SCHEMA: &str = {schema:?};\n\
pub const SPEC_STATUS: &str = {status:?};\n\
pub const CLASSIFICATION: &str = \"CANDIDATE_NOT_FROZEN\";\n\
pub const BAYER_PATTERN: &str = {bayer_pattern:?};\n\
pub const INPUT_DTYPE: &str = {input_dtype:?};\n\
pub const ACCUMULATOR_DTYPE: &str = {accumulator_dtype:?};\n\
pub const OUTPUT_DTYPE: &str = {output_dtype:?};\n\
pub const ROUNDING: &str = {rounding:?};\n\
pub const PADDING: &str = {padding:?};\n\
pub const MASK: &str = {mask:?};\n\
pub const ZERO_POINT: usize = {zero_point};\n\
pub const PRIMARY_OUTPUT_SIZE: usize = {primary_output_size};\n\
pub const DIAGNOSTIC_OUTPUT_SIZES: &[usize] = &[{rust_diagnostic_output_sizes}];\n\
pub const SUPPORTED_OUTPUT_SIZES: &[usize] = &[{rust_output_sizes}];\n\
pub const SPEC_CANONICAL_JSON: &str = {config:?};\n\
pub const CAMERA_CROP: Crop = Crop {{ y0: {}, x0: {}, height: {}, width: {} }};\n\
pub const EMISSION_CROP: Crop = Crop {{ y0: {}, x0: {}, height: {}, width: {} }};\n",
        camera_crop.y0,
        camera_crop.x0,
        camera_crop.height,
        camera_crop.width,
        emission_crop.y0,
        emission_crop.x0,
        emission_crop.height,
        emission_crop.width,
    ))
}

fn main() {
    let spec_path = canonical_config_path().unwrap_or_else(|error| panic!("{error}"));
    println!("cargo:rerun-if-changed={}", spec_path.display());
    let generated = generate(&spec_path).unwrap_or_else(|error| panic!("{error}"));
    let output = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is absent"))
        .join("preprocess_spec.rs");
    fs::write(output, generated).expect("cannot write generated preprocessing constants");
}
