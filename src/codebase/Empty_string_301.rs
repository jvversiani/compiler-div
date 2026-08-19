// Rosetta Code task: Empty string
// Source: https://rosettacode.org/wiki/Empty_string#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// is empty: true
// is empty: false
// is empty: true
// is empty: false
// is not empty: true
// =======================

fn main(){
    let s = "";
    println!("is empty: {}", s.is_empty());
    let t = "x";
    println!("is empty: {}", t.is_empty());
    let a = String::new();
    println!("is empty: {}", a.is_empty());
    let b = "x".to_string();
    println!("is empty: {}", b.is_empty());
    println!("is not empty: {}", !b.is_empty());
}
