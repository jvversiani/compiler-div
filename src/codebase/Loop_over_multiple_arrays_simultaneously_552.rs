// Rosetta Code task: Loop over multiple arrays simultaneously
// Source: https://rosettacode.org/wiki/Loop_over_multiple_arrays_simultaneously#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// aA1
// bB2
// cC3
// =======================

fn main() {
    let a1 = ["a", "b", "c"];
    let a2 = ["A", "B", "C"];
    let a3 = [1, 2, 3];

    for ((&x, &y), &z) in a1.iter().zip(a2.iter()).zip(a3.iter()) {
        println!("{}{}{}", x, y, z);
    }
}
