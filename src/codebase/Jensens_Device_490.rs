// Rosetta Code task: Jensen's Device
// Source: https://rosettacode.org/wiki/Jensen's_Device#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 5.187378
// =======================

use std::f32;

fn harmonic_sum<F>(lo: usize, hi: usize, term: F) -> f32
where
    F: Fn(f32) -> f32,
{
    (lo..hi + 1).fold(0.0, |acc, item| acc + term(item as f32))
}

fn main() {
    println!("{}", harmonic_sum(1, 100, |i| 1.0 / i));
}
