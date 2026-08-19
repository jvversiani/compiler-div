// Rosetta Code task: Copy a string
// Source: https://rosettacode.org/wiki/Copy_a_string#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// s1 = A String, s2 = Another String
// =======================

fn main() {
    let s1 = "A String";
    let mut s2 = s1;

    s2 = "Another String";

    println!("s1 = {}, s2 = {}", s1, s2);
}
