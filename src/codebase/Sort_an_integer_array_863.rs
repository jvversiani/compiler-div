// Rosetta Code task: Sort an integer array
// Source: https://rosettacode.org/wiki/Sort_an_integer_array#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
// =======================

fn main() {
    let mut a = vec!(9, 8, 7, 6, 5, 4, 3, 2, 1, 0);

    a.sort();
    println!("{:?}", a);
}
