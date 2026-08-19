// Rosetta Code task: File modification time
// Source: https://rosettacode.org/wiki/File_modification_time#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// File size: 36 bytes
// Is file:   true
// Read-only: false
// Deleted foo.txt
// =======================

use std::fs::{self, File};
use std::io::Write;

fn main() -> std::io::Result<()> {
    let file_path = "foo.txt";

    // --- Create the file with some content ---
    {
        let mut file = File::create(file_path)?;
        file.write_all(b"Hello, this is some sample content!\n")?;
    } // file handle closed here

    // --- Read and analyse metadata ---
    let metadata = fs::metadata(file_path)?;

    println!("File size: {} bytes", metadata.len());
    println!("Is file:   {}", metadata.is_file());
    println!("Read-only: {}", metadata.permissions().readonly());

    // --- Delete the file ---
    fs::remove_file(file_path)?;
    println!("Deleted {}", file_path);

    Ok(())
}
