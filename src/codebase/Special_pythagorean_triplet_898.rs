// Rosetta Code task: Special pythagorean triplet
// Source: https://rosettacode.org/wiki/Special_pythagorean_triplet#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// {200, 375, 425}
// The product of {200, 375, 425} is 31875000
// =======================

use std::collections::BTreeSet;

fn main() {
    let mut numbers: BTreeSet<u32> = BTreeSet::new();
    for a in 1u32..=1000u32 {
        for b in 1u32..=1000u32 {
            for c in 1u32..=1000u32 {
                if a + b + c == 1000 && a * a + b * b == c * c {
                    numbers.insert(a);
                    numbers.insert(b);
                    numbers.insert(c);
                }
            }
        }
    }
    let mut product: u32 = 1;
    for k in &numbers {
        product *= *k;
    }
    println!("{:?}", numbers);
    println!("The product of {:?} is {}", numbers, product);
}