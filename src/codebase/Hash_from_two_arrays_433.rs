// Rosetta Code task: Hash from two arrays
// Source: https://rosettacode.org/wiki/Hash_from_two_arrays#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// {"a": 1, "b": 2, "c": 3}
// =======================

use std::collections::BTreeMap;

fn main() {
    let keys = ["a", "b", "c"];
    let values = [1, 2, 3];

    let hash = keys.iter().zip(values.iter()).collect::<BTreeMap<_, _>>();
    println!("{:?}", hash);
}
