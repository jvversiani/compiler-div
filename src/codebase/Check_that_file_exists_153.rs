// Rosetta Code task: Check that file exists
// Source: https://rosettacode.org/wiki/Check_that_file_exists#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// input.txt does not exist
// docs does not exist
// /input.txt does not exist
// /docs does not exist
// =======================

use std::fs;

fn main() {
    for file in ["input.txt", "docs", "/input.txt", "/docs"].iter() {
        match fs::metadata(file) {
            Ok(attr) => {
                if attr.is_dir() {
                    println!("{} is a directory", file);
                }else {
                    println!("{} is a file", file);
                }
            },
            Err(_) => {
                println!("{} does not exist", file);
            }
        };
    }
}
