// Rosetta Code task: Idiomatically determine all the lowercase and uppercase letters
// Source: https://rosettacode.org/wiki/Idiomatically_determine_all_the_lowercase_and_uppercase_letters#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Lowercase letters: abcdefghijklmnopqrstuvwxyz
// Uppercase letters: ABCDEFGHIJKLMNOPQRSTUVWXYZ
// =======================

fn main() {
    println!(
        "Lowercase letters: {}",
        (b'a'..=b'z').map(|c| c as char).collect::<String>()
    );
    println!(
        "Uppercase letters: {}",
        (b'A'..=b'Z').map(|c| c as char).collect::<String>()
    );
}
