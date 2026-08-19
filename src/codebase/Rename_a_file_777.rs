// Rosetta Code task: Rename a file
// Source: https://rosettacode.org/wiki/Rename_a_file#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// All good!
// =======================

use std::fs::{self, File};
use std::io::Write;

fn main() {
    let err = "File move error";

    // --- Create a file and a folder ---
    {
        let mut file = File::create("input.txt").expect("could not create input.txt");
        file.write_all(b"sample contents\n").expect("could not write input.txt");
    } // handle closed here

    fs::create_dir("docs").expect("could not create docs/");
    // put something inside the folder so it isn't empty
    {
        let mut file = File::create("docs/note.txt").expect("could not create docs/note.txt");
        file.write_all(b"a note inside docs\n").expect("could not write note");
    }

    // --- Rename them ---
    fs::rename("input.txt", "output.txt").ok().expect(err);
    fs::rename("docs", "mydocs").ok().expect(err);

    // --- Delete them ---
    fs::remove_file("output.txt").expect("could not remove output.txt");
    fs::remove_dir_all("mydocs").expect("could not remove mydocs/");

    println!("All good!");
}
