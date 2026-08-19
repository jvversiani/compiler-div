// Rosetta Code task: Reverse a string
// Source: https://rosettacode.org/wiki/Reverse_a_string#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Normal: "abcdef"
// Reversed: "fedcba"
// =======================

fn main(){
    let mut buffer = b"abcdef".to_vec();
    println!("Normal: {:?}", String::from_utf8_lossy(&buffer));
    buffer.reverse();
    println!("Reversed: {:?}", String::from_utf8_lossy(&buffer));
}
