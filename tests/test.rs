#![allow(
    clippy::approx_constant,
    clippy::unreadable_literal,
    clippy::unseparated_literal_suffix
)]

use std::fs::File;
use std::io::{Read, Result};
use std::path::PathBuf;
use std::{f32, f64};

#[test]
#[ignore] // nlohmann json changes breaks this
fn test_f64() {
    test_write(1.234e20f64, "123400000000000000000.0");
    test_write(1.234e21f64, "1.234e21");
    test_write(2.71828f64, "2.71828");
    test_write(0.0f64, "0.0");
    test_write(-0.0f64, "-0.0");
    test_write(1.1e128f64, "1.1e128");
    test_write(1.1e-64f64, "1.1e-64");
    test_write(2.718281828459045f64, "2.718281828459045");
    test_write(5e-324f64, "5e-324");
    test_write(f64::MAX, "1.7976931348623157e308");
    test_write(f64::MIN, "-1.7976931348623157e308");
}

#[test]
#[ignore] // nlohmann json changes breaks this
fn test_f32() {
    test_write(1.234e20f32, "123400000000000000000.0");
    test_write(1.234e21f32, "1.234e21");
    test_write(2.71828f32, "2.71828");
    test_write(0.0f32, "0.0");
    test_write(-0.0f32, "-0.0");
    test_write(1.1e32f32, "1.1e32");
    test_write(1.1e-32f32, "1.1e-32");
    test_write(2.7182817f32, "2.7182817");
    test_write(1e-45f32, "1e-45");
    test_write(f32::MAX, "3.4028235e38");
    test_write(f32::MIN, "-3.4028235e38");
}

fn test_write<F: dtoa::Float>(value: F, expected: &'static str) {
    let mut buffer = dtoa::Buffer::new();
    let string = buffer.format(value);
    assert_eq!(string, expected);
}

fn read_one_f64(file: &mut File) -> Result<(f64, String)> {
    let mut float_buffer = [0; 8]; // Buffer to store the f64
    file.read_exact(&mut float_buffer)?;

    let float_value = f64::from_le_bytes(float_buffer);

    let mut int_buffer = [0; 4]; // Buffer to store the i32
    file.read_exact(&mut int_buffer)?;

    let int_value = i32::from_le_bytes(int_buffer);

    let mut string_buffer = vec![0; int_value as usize]; // Buffer to store the string
    file.read_exact(&mut string_buffer)?;

    let string_value = String::from_utf8_lossy(&string_buffer);
    Ok((float_value, string_value.to_string()))
}

fn read_one_f32(file: &mut File) -> Result<(f32, String)> {
    let mut float_buffer = [0; 4]; // Buffer to store the f64
    file.read_exact(&mut float_buffer)?;

    let float_value = f32::from_le_bytes(float_buffer);

    let mut int_buffer = [0; 4]; // Buffer to store the i32
    file.read_exact(&mut int_buffer)?;

    let int_value = i32::from_le_bytes(int_buffer);

    let mut string_buffer = vec![0; int_value as usize]; // Buffer to store the string
    file.read_exact(&mut string_buffer)?;

    let string_value = String::from_utf8_lossy(&string_buffer);
    Ok((float_value, string_value.to_string()))
}

#[test]
fn test_f64_nholmann_json_match() -> Result<()> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("nholmann_json_f64.txt");
    let mut file = File::open(path)?;

    loop {
        let res = read_one_f64(&mut file);
        let (float_value, nholmann_string) = match res {
            Ok(values) => values,
            Err(e) => {
                if e.kind() == std::io::ErrorKind::UnexpectedEof {
                    break;
                } else {
                    return Err(e);
                }
            }
        };

        let orig: f64 = float_value;
        let mut buffer = dtoa::Buffer::new();
        let dtoa_string = buffer.format(orig);
        assert_eq!(dtoa_string, nholmann_string);
    }
    Ok(())
}

#[test]
fn test_f32_nholmann_json_match() -> Result<()> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("nholmann_json_f32.txt");
    let mut file = File::open(path)?;
    loop {
        let res = read_one_f32(&mut file);
        let (float_value, nholmann_string) = match res {
            Ok(values) => values,
            Err(e) => {
                if e.kind() == std::io::ErrorKind::UnexpectedEof {
                    break;
                } else {
                    return Err(e);
                }
            }
        };

        let orig: f32 = float_value;
        let mut buffer = dtoa::Buffer::new();
        let dtoa_string = buffer.format(orig as f64);
        assert_eq!(dtoa_string, nholmann_string);
    }
    Ok(())
}
