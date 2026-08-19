// Rosetta Code task: Tokenize a string
// Source: https://rosettacode.org/wiki/Tokenize_a_string#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Hello.How.Are.You.Today
// =======================

fn main() {
    let s = "Hello,How,Are,You,Today";
    let tokens: Vec<&str> = s.split(",").collect();
    println!("{}", tokens.join("."));
}
