// Rosetta Code task: Babbage problem
// Source: https://rosettacode.org/wiki/Babbage_problem#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// The smallest number whose square ends in 269696 is 25264
// =======================

fn main() {
    let mut current = 0;
    while (current * current) % 1_000_000 != 269_696 {
        current += 1;
    }
    println!(
        "The smallest number whose square ends in 269696 is {}",
        current
    );
}
