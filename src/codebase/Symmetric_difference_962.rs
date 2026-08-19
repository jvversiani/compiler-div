// Rosetta Code task: Symmetric difference
// Source: https://rosettacode.org/wiki/Symmetric_difference#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// ["Serena", "Jim"]
// =======================

use std::collections::HashSet;

fn main() {
    let a: HashSet<_> = ["John", "Bob", "Mary", "Serena"]
        .iter()
        .collect();
    let b = ["Jim", "Mary", "John", "Bob"]
        .iter()
        .collect();

    let diff = a.symmetric_difference(&b);
    println!("{:?}", diff);
}
