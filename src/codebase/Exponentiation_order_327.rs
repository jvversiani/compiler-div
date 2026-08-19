// Rosetta Code task: Exponentiation order
// Source: https://rosettacode.org/wiki/Exponentiation_order#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 5**3**2   =   15625
// (5**3)**2 =   15625
// 5**(3**2) = 1953125
// =======================

fn main() {
    println!("5**3**2   = {:7}", 5u32.pow(3).pow(2));
    println!("(5**3)**2 = {:7}", (5u32.pow(3)).pow(2));
    println!("5**(3**2) = {:7}", 5u32.pow(3u32.pow(2)));
}
