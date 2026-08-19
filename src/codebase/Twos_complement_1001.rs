// Rosetta Code task: Two's complement
// Source: https://rosettacode.org/wiki/Two's_complement#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// i = 1, -(i) = -1, !(i) + 1 = -1
// =======================

fn main() {
    let i = 1;
    println!("i = {:#?}, -(i) = {:#?}, !(i) + 1 = {:#?}", i, -i, !i + 1);
    return (); // i = 1, -(i) = -1, !(i) + 1 = -1
}
