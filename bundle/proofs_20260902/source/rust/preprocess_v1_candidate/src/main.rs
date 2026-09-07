use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;

use zeebeam_preprocess_v1_candidate::{
    CLASSIFICATION, PREPROCESS_ID, SPEC_CANONICAL_JSON, SUPPORTED_OUTPUT_SIZES, output_bytes,
    preprocess_candidate, sha256_hex, spec_sha256,
};

const USAGE: &str = "Usage: zeebeam-preprocess-v1-candidate \\
  --expected-spec-sha256 SHA256 \\
  --raw RAW_BAYER \\
  --emission-rgb DECODED_RGB8 \\
  --output-size 32|64|256 \\
  --output OUTPUT_BYTES\n\
       zeebeam-preprocess-v1-candidate --print-spec";

struct Arguments {
    expected_spec_sha256: String,
    raw: PathBuf,
    emission_rgb: PathBuf,
    output_size: usize,
    output: PathBuf,
}

fn next_value(iterator: &mut impl Iterator<Item = OsString>, flag: &str) -> Result<OsString, String> {
    iterator
        .next()
        .ok_or_else(|| format!("{flag} requires one value"))
}

fn set_once<T>(slot: &mut Option<T>, value: T, flag: &str) -> Result<(), String> {
    if slot.is_some() {
        return Err(format!("{flag} may appear only once"));
    }
    *slot = Some(value);
    Ok(())
}

fn parse_arguments() -> Result<Option<Arguments>, String> {
    let mut expected_spec_sha256 = None;
    let mut raw = None;
    let mut emission_rgb = None;
    let mut output_size = None;
    let mut output = None;
    let mut iterator = env::args_os().skip(1);

    while let Some(flag_value) = iterator.next() {
        let flag = flag_value
            .to_str()
            .ok_or_else(|| "argument flags must be UTF-8".to_owned())?;
        match flag {
            "-h" | "--help" => return Ok(None),
            "--expected-spec-sha256" => {
                let value = next_value(&mut iterator, flag)?;
                let text = value
                    .to_str()
                    .ok_or_else(|| "--expected-spec-sha256 must be UTF-8".to_owned())?;
                if text.len() != 64
                    || !text
                        .bytes()
                        .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
                {
                    return Err(
                        "--expected-spec-sha256 must be one lowercase SHA-256 digest".to_owned(),
                    );
                }
                set_once(&mut expected_spec_sha256, text.to_owned(), flag)?;
            }
            "--raw" => {
                let value = next_value(&mut iterator, flag)?;
                set_once(&mut raw, PathBuf::from(value), flag)?;
            }
            "--emission-rgb" => {
                let value = next_value(&mut iterator, flag)?;
                set_once(&mut emission_rgb, PathBuf::from(value), flag)?;
            }
            "--output-size" => {
                let value = next_value(&mut iterator, flag)?;
                let text = value
                    .to_str()
                    .ok_or_else(|| "--output-size must be UTF-8".to_owned())?;
                let parsed = text
                    .parse::<usize>()
                    .map_err(|_| "--output-size must be 32, 64, or 256".to_owned())?;
                if !SUPPORTED_OUTPUT_SIZES.contains(&parsed) {
                    return Err("--output-size must be 32, 64, or 256".to_owned());
                }
                set_once(&mut output_size, parsed, flag)?;
            }
            "--output" => {
                let value = next_value(&mut iterator, flag)?;
                set_once(&mut output, PathBuf::from(value), flag)?;
            }
            _ => return Err(format!("unknown argument: {flag}")),
        }
    }

    Ok(Some(Arguments {
        expected_spec_sha256: expected_spec_sha256
            .ok_or_else(|| "--expected-spec-sha256 is required".to_owned())?,
        raw: raw.ok_or_else(|| "--raw is required".to_owned())?,
        emission_rgb: emission_rgb.ok_or_else(|| "--emission-rgb is required".to_owned())?,
        output_size: output_size.ok_or_else(|| "--output-size is required".to_owned())?,
        output: output.ok_or_else(|| "--output is required".to_owned())?,
    }))
}

fn run(arguments: Arguments) -> Result<(), String> {
    let computed_spec_sha256 = spec_sha256();
    if arguments.expected_spec_sha256 != computed_spec_sha256 {
        return Err("caller-pinned preprocessing specification SHA-256 differs".to_owned());
    }
    let raw = fs::read(&arguments.raw)
        .map_err(|error| format!("failed to read raw Bayer file: {error}"))?;
    let emission = fs::read(&arguments.emission_rgb)
        .map_err(|error| format!("failed to read decoded RGB file: {error}"))?;
    let (camera, emission) = preprocess_candidate(&raw, &emission, arguments.output_size)
        .map_err(|error| error.to_string())?;
    let payload = output_bytes(&camera, &emission, arguments.output_size)
        .map_err(|error| error.to_string())?;
    fs::write(&arguments.output, &payload)
        .map_err(|error| format!("failed to write output bytes: {error}"))?;
    let output_sha256 = sha256_hex(&payload);
    println!(
        "{{\"classification\":\"{}\",\"output_bytes\":{},\"output_sha256\":\"{}\",\"output_size\":{},\"preprocess_id\":\"{}\",\"spec_sha256\":\"{}\"}}",
        CLASSIFICATION,
        payload.len(),
        output_sha256,
        arguments.output_size,
        PREPROCESS_ID,
        computed_spec_sha256,
    );
    Ok(())
}

fn main() {
    let raw_arguments = env::args_os().skip(1).collect::<Vec<_>>();
    if raw_arguments == [OsString::from("--print-spec")] {
        print!("{SPEC_CANONICAL_JSON}");
        return;
    }
    let arguments = match parse_arguments() {
        Ok(Some(arguments)) => arguments,
        Ok(None) => {
            println!("{USAGE}");
            return;
        }
        Err(error) => {
            eprintln!("error: {error}\n{USAGE}");
            std::process::exit(2);
        }
    };
    if let Err(error) = run(arguments) {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}
