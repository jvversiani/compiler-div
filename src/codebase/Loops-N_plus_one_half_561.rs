// Rosetta Code task: Loops/N plus one half
// Source: https://rosettacode.org/wiki/Loops/N_plus_one_half#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
// =======================

fn main() {
    for i in 1..=10 {
        print!("{}{}", i, if i < 10 { ", " } else { "\n" });
    }
}
