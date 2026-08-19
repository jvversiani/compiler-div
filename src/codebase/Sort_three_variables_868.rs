// Rosetta Code task: Sort three variables
// Source: https://rosettacode.org/wiki/Sort_three_variables#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Sorted: [1, 3, 5]
// Reverse sorted: [5, 3, 1]
// =======================

fn main() {
    let mut array = [5, 1, 3];
    array.sort();
    println!("Sorted: {:?}", array);
    array.sort_by(|a, b| b.cmp(a));
    println!("Reverse sorted: {:?}", array);
}
