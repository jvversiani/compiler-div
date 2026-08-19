// Rosetta Code task: Closures/Value capture
// Source: https://rosettacode.org/wiki/Closures/Value_capture#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 7th val: 49
// =======================

fn main() {
    let fs: Vec<_> = (0..10).map(|i| {move || i*i} ).collect();
    println!("7th val: {}", fs[7]());
}
