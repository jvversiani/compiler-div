// Rosetta Code task: Base64 encode data
// Source: https://rosettacode.org/wiki/Base64_encode_data#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// SGVsbG8sIHRoaXMgaXMgc2FtcGxlIGZhdmljb24gZGF0YSE=
// =======================

use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use std::error::Error;

const CHAR_SET: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const PADDING_CHAR: char = '=';

fn encode(input_bytes: &[u8]) -> String {
    let output_len = ((input_bytes.len() + 2) / 3) * 4;
    let mut encoded = String::with_capacity(output_len);
    let char_set_bytes = CHAR_SET.as_bytes();

    for chunk in input_bytes.chunks(3) {
        let mut combined: u32 = (chunk[0] as u32) << 16;
        if chunk.len() > 1 {
            combined |= (chunk[1] as u32) << 8;
        }
        if chunk.len() > 2 {
            combined |= chunk[2] as u32;
        }

        let idx1 = (combined >> 18) & 63;
        let idx2 = (combined >> 12) & 63;
        let idx3 = (combined >> 6) & 63;
        let idx4 = combined & 63;

        encoded.push(char_set_bytes[idx1 as usize] as char);
        encoded.push(char_set_bytes[idx2 as usize] as char);

        if chunk.len() > 1 {
            encoded.push(char_set_bytes[idx3 as usize] as char);
        } else {
            encoded.push(PADDING_CHAR);
        }

        if chunk.len() > 2 {
            encoded.push(char_set_bytes[idx4 as usize] as char);
        } else {
            encoded.push(PADDING_CHAR);
        }
    }

    encoded
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "favicon.ico";
    let path = Path::new(file_path);

    // --- Create the file ---
    let sample_data: &[u8] = b"Hello, this is sample favicon data!";
    {
        let mut new_file = File::create(&path)
            .map_err(|e| format!("Error creating file '{}': {}", file_path, e))?;
        new_file.write_all(sample_data)
            .map_err(|e| format!("Error writing file '{}': {}", file_path, e))?;
    } // file closed here when it goes out of scope

    // --- Open and read the file ---
    let mut file = File::open(&path)
        .map_err(|e| format!("Error opening file '{}': {}", file_path, e))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|e| format!("Error reading file '{}': {}", file_path, e))?;
    drop(file); // close before deleting (important on Windows)

    // --- Encode ---
    let encoded_string = encode(&buffer);

    // --- Output ---
    io::stdout().write_all(encoded_string.as_bytes())?;
    io::stdout().write_all(b"\n")?;
    io::stdout().flush()?;

    // --- Delete the file ---
    fs::remove_file(&path)
        .map_err(|e| format!("Error deleting file '{}': {}", file_path, e))?;

    Ok(())
}
