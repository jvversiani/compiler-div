// Rosetta Code task: Binary digits
// Source: https://rosettacode.org/wiki/Binary_digits#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 0
// 1
// 10
// 11
// 100
// 101
// 110
// 111
// =======================

fn main() {
    for i in 0..8 {
        println!("{:b}", i)
    }
}
