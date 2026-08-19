// Rosetta Code task: Loops/Downward for
// Source: https://rosettacode.org/wiki/Loops/Downward_for#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 10
// 9
// 8
// 7
// 6
// 5
// 4
// 3
// 2
// 1
// 0
// =======================

fn main() {
    for i in (0..=10).rev() {
        println!("{}", i);
    }
}
