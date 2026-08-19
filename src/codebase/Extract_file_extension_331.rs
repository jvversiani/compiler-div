// Rosetta Code task: Extract file extension
// Source: https://rosettacode.org/wiki/Extract_file_extension#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// http://example.com/download.tar.gz | .gz      | Some("gz")
// CharacterModel.3DS                 | .3DS     | Some("3DS")
// .desktop                           | .desktop | None
// document                           |          | None
// document.txt_backup                |          | Some("txt_backup")
// /etc/pam.d/login                   |          | None
// =======================

use std::path::Path;

fn main() {
    let filenames = &[
        "http://example.com/download.tar.gz",
        "CharacterModel.3DS",
        ".desktop",
        "document",
        "document.txt_backup",
        "/etc/pam.d/login",
    ];

    for filename in filenames {
        println!(
            "{:34} | {:8} | {:?}",
            filename,
            extension(filename),
            Path::new(filename).extension()
        );
    }
}

fn extension(filename: &str) -> &str {
    filename
        .rfind('.')
        .map(|idx| &filename[idx..])
        .filter(|ext| ext.chars().skip(1).all(|c| c.is_ascii_alphanumeric()))
        .unwrap_or("")
}
