// Rosetta Code task: String prepend
// Source: https://rosettacode.org/wiki/String_prepend#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Hello World
// =======================

fn main(){
    let mut s = "World".to_string();
    s.insert_str(0, "Hello ");
    println!("{}", s);
}
