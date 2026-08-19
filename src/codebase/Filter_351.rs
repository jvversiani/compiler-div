// Rosetta Code task: Filter
// Source: https://rosettacode.org/wiki/Filter#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// new vec filtered:
// [2, 4, 6, 8, 10, 12, 14, 16, 18]
// original vec filtered:
// [2, 4, 6, 8, 10, 12, 14, 16, 18]
// =======================

fn main() {
    println!("new vec filtered: ");
    let nums: Vec<i32> = (1..20).collect();
    let evens: Vec<i32> = nums.iter().cloned().filter(|x| x % 2 == 0).collect();
    println!("{:?}", evens);

    // Filter an already existing vector
    println!("original vec filtered: ");
    let mut nums: Vec<i32> = (1..20).collect();
    nums.retain(|x| x % 2 == 0);
    println!("{:?}", nums);
}
