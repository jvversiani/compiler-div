// Rosetta Code task: Fixed length records
// Source: https://rosettacode.org/wiki/Fixed_length_records#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 8.........7.........6.........5.........4.........3.........2.........1...1 eniL
//                                                                           2 eniL
//                                                                           3 eniL
//                                                                           4 eniL
//
//                                                                           6 eniL
//                                                                           7 eniL
// ............................................................8 enil detnednI
// NIGRAM TR                                                                 9 eniL
// =======================

use std::fs::{self, File};
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};

fn reverse_file(
    input_filename: &str,
    output_filename: &str,
    record_len: usize,
) -> std::io::Result<()> {
    let mut input = BufReader::new(File::open(input_filename)?);
    let mut output = BufWriter::new(File::create(output_filename)?);
    let mut buffer = vec![0; record_len];
    while input.read(&mut buffer)? == record_len {
        buffer.reverse();
        output.write_all(&buffer)?;
    }
    output.flush()?;
    Ok(())
}

/// Write the 9 fixed-length (80-byte) input records that produce the
/// task's expected output once each record is reversed.
fn create_input(filename: &str, record_len: usize) -> std::io::Result<()> {
    let lines = [
        "Line 1...1.........2.........3.........4.........5.........6.........7.........8",
        "Line 2",
        "Line 3",
        "Line 4",
        "", // blank line 5
        "Line 6",
        "Line 7",
        "     Indented line 8............................................................",
        "Line 9                                                                 RT MARGIN",
    ];

    let mut output = BufWriter::new(File::create(filename)?);
    for line in &lines {
        // Pad (or truncate) each line to exactly record_len bytes.
        let mut record = line.as_bytes().to_vec();
        record.resize(record_len, b' ');
        record.truncate(record_len);
        output.write_all(&record)?;
    }
    output.flush()?;
    Ok(())
}

fn main() {
    let record_len = 80;

    if let Err(error) = create_input("infile.dat", record_len) {
        eprintln!("I/O error creating input: {}", error);
        return;
    }

    match reverse_file("infile.dat", "outfile.dat", record_len) {
        Ok(()) => {}
        Err(error) => {
            eprintln!("I/O error: {}", error);
            return;
        }
    }

    // Print outfile.dat so the result is visible.
    match fs::read_to_string("outfile.dat") {
        Ok(contents) => {
            // Insert a newline every record_len bytes for display.
            for chunk in contents.as_bytes().chunks(record_len) {
                println!("{}", String::from_utf8_lossy(chunk).trim_end());
            }
        }
        Err(error) => eprintln!("I/O error reading output: {}", error),
    }

    // Clean up.
    let _ = fs::remove_file("infile.dat");
    let _ = fs::remove_file("outfile.dat");
}