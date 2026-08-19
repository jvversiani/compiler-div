// Rosetta Code task: Create a file
// Source: https://rosettacode.org/wiki/Create_a_file#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// All good!
// =======================

use std::io::{self, Write};
use std::fs::{DirBuilder, File, remove_file, remove_dir};
use std::path::Path;
use std::{process,fmt};

const FILE_NAME: &'static str = "output.txt";
const DIR_NAME : &'static str = "docs";

fn main() {
    create(".")
        .and(delete("."))
        .unwrap_or_else(|e| error_handler(e, 1));
    println!("All good!");
}


fn create<P>(root: P) -> io::Result<File>
where
    P: AsRef<Path>,
{
    let f_path = root.as_ref().join(FILE_NAME);
    let d_path = root.as_ref().join(DIR_NAME);
    DirBuilder::new()
        .recursive(true)
        .create(d_path)
        .and(File::create(f_path))
}

fn delete<P>(root: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    remove_file(root.as_ref().join(FILE_NAME))
        .and(remove_dir(root.as_ref().join(DIR_NAME)))
}

fn error_handler<E: fmt::Display>(error: E, code: i32) -> ! {
    let _ = writeln!(&mut io::stderr(), "Error: {}", error);
    process::exit(code)
}
