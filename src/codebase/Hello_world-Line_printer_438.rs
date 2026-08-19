// Rosetta Code task: Hello world/Line printer
// Source: https://rosettacode.org/wiki/Hello_world/Line_printer#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// All good!
// =======================

use std::fs::{OpenOptions, remove_file};
use std::io::Write;

fn main() {
    let path = "lp0_output.txt";

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .unwrap();
    file.write_all(b"Hello, World!\n").unwrap();

    drop(file);

    remove_file(path).unwrap();

    println!("All good!");
}