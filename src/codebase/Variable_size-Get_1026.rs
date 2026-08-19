// Rosetta Code task: Variable size/Get
// Source: https://rosettacode.org/wiki/Variable_size/Get#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// All good!
// =======================

use std::mem;

fn main() {
    // Specify type
    assert_eq!(4, mem::size_of::<i32>());
    
    // Provide a value
    let arr: [u16; 3] = [1, 2, 3];
    assert_eq!(6, mem::size_of_val(&arr));

    println!("All good!");
}
