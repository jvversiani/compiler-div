// Rosetta Code task: Enumerations
// Source: https://rosettacode.org/wiki/Enumerations#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 27
// =======================

enum Fruits {
    Apple,
    Banana,
    Cherry
}

enum FruitsWithNumbers {
    Strawberry = 0,
    Pear = 27,
}

fn main() {
    // Access to numerical value by conversion
    println!("{}", FruitsWithNumbers::Pear as u8);
}
