// Rosetta Code task: Primality by trial division
// Source: https://rosettacode.org/wiki/Primality_by_trial_division#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 2 
// 3 
// 5 
// 7 
// 11 
// 13 
// 17 
// 19 
// 23 
// 29 
// =======================

fn is_prime(n: u64) -> bool {
    match n {
        0 | 1 => false,
        2 => true,
        _even if n % 2 == 0 => false,
        _ => {
            let sqrt_limit = (n as f64).sqrt() as u64;
            (3..=sqrt_limit).step_by(2).find(|i| n % i == 0).is_none()
        }
    }
}

fn main() {
    for i in (1..30).filter(|i| is_prime(*i)) {
        println!("{} ", i);
    }
}
