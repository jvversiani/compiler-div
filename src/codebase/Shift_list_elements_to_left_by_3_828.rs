// Rosetta Code task: Shift list elements to left by 3
// Source: https://rosettacode.org/wiki/Shift_list_elements_to_left_by_3#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Before: [1, 2, 3, 4, 5, 6, 7, 8, 9]
//  After: [4, 5, 6, 7, 8, 9, 1, 2, 3]
// =======================

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("Before: {:?}", v);
    v.rotate_left(3);
    println!(" After: {:?}", v);
}
