// Rosetta Code task: Rot-13
// Source: https://rosettacode.org/wiki/Rot-13#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// all good!
// =======================

fn rot13(string: &str) -> String {
    string.chars().map(|c| {
        match c {
            'a'..='m' | 'A'..='M' => ((c as u8) + 13) as char,
            'n'..='z' | 'N'..='Z' => ((c as u8) - 13) as char,
            _ => c
        }
    }).collect()
}

fn main () {
    assert_eq!(rot13("abc"), "nop");

    println!("all good!");
}
