// Rosetta Code task: Make directory path
// Source: https://rosettacode.org/wiki/Make_directory_path#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// all good!
// =======================

use std::fs;

fn main() {
    fs::create_dir_all("./path/to/dir").expect("An Error Occured!");

    println!("all good!");
}
