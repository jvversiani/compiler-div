// Rosetta Code task: String length
// Source: https://rosettacode.org/wiki/String_length#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Character length: 4
// =======================

fn main() {
    let s = "文字化け";  // UTF-8
    println!("Character length: {}", s.chars().count());
}
