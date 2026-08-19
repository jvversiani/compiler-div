// Rosetta Code task: Greatest element of a list
// Source: https://rosettacode.org/wiki/Greatest_element_of_a_list#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Some(39)
// 39
// =======================

fn main() {
    let nums = [1,2,39,34,20];
    println!("{:?}", nums.iter().max());
    println!("{}", nums.iter().max().unwrap());
}
