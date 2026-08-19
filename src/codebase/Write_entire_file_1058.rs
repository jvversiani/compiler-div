// Rosetta Code task: Write entire file
// Source: https://rosettacode.org/wiki/Write_entire_file#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// All good!
// =======================

use std::fs::{File, remove_file};
use std::io::Write;

fn main() -> std::io::Result<()> {
    let data = "Sample text.";
    let mut file = File::create("filename.txt")?;
    write!(file, "{}", data)?;
    drop(file);
    remove_file("filename.txt")?;
    println!("All good!");
    Ok(())
}