// Rosetta Code task: Read entire file
// Source: https://rosettacode.org/wiki/Read_entire_file#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Read 47 bytes
// Hello, world!
// This is the entire file contents.
// =======================

use std::fs::{self, File};
use std::io::{Read, Write};

fn main() {
    let file_path = "somefile.txt";

    // --- Create the file ---
    {
        let mut file = File::create(file_path).unwrap();
        file.write_all(b"Hello, world!\nThis is the entire file contents.").unwrap();
    } // handle closed here

    // --- Read the entire file ---
    let mut file = File::open(file_path).unwrap();
    let mut contents: Vec<u8> = Vec::new();
    // Returns amount of bytes read and append the result to the buffer
    let result = file.read_to_end(&mut contents).unwrap();
    println!("Read {} bytes", result);

    // To print the contents of the file
    let filestr = String::from_utf8(contents).unwrap();
    println!("{}", filestr);

    // --- Delete the file ---
    drop(file); // close before deleting (matters on Windows)
    fs::remove_file(file_path).unwrap();
}
