// Rosetta Code task: Loops/While
// Source: https://rosettacode.org/wiki/Loops/While#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 1024
// 512
// 256
// 128
// 64
// 32
// 16
// 8
// 4
// 2
// 1
// =======================

fn main() {
    let mut n: i32 = 1024;
    while n > 0 {
        println!("{}", n);
        n /= 2;
    }
}
