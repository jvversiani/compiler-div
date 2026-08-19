// Rosetta Code task: Order two numerical lists
// Source: https://rosettacode.org/wiki/Order_two_numerical_lists#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// false
// true
// =======================

fn main(){
    println!("{}", vec![1, 2, 1, 3, 2] < vec![1, 2, 0, 4, 4, 0, 0, 0]);
    println!("{}", vec![1, 2, 1, 3, 2] >= vec![1, 2, 0, 4, 4, 0, 0, 0]);
}
