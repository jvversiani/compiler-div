// Rosetta Code task: Sum of a series
// Source: https://rosettacode.org/wiki/Sum_of_a_series#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 500500
// 500500
// =======================

const LOWER: i32 = 1;
const UPPER: i32 = 1000;

// Because the rule for our series is simply adding one, the number of terms are the number of
// digits between LOWER and UPPER
const NUMBER_OF_TERMS: i32 = (UPPER + 1) - LOWER;
fn main() {
    // Formulaic method
    println!("{}", (NUMBER_OF_TERMS * (LOWER + UPPER)) / 2);
    // Naive method
    println!("{}", (LOWER..UPPER + 1).fold(0, |sum, x| sum + x));
}
