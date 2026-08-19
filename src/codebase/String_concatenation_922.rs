// Rosetta Code task: String concatenation
// Source: https://rosettacode.org/wiki/String_concatenation#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// hello
// hello world
// =======================

fn main() {
    let s = "hello".to_owned();
    println!("{}", s);
    
    let s1 = s + " world";
    println!("{}", s1);
}
