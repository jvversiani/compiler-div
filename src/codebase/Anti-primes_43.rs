// Rosetta Code task: Anti-primes
// Source: https://rosettacode.org/wiki/Anti-primes#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// The first 20 anti-primes are:
// 1 2 4 6 12 24 36 48 60 120 180 240 360 720 840 1260 1680 2520 5040 7560
// =======================

fn count_divisors(n: u64) -> usize {
    if n < 2 {
        return 1;
    }
    2 + (2..=(n / 2)).filter(|i| n % i == 0).count()
}

fn main() {
    println!("The first 20 anti-primes are:");
    (1..)
        .scan(0, |max, n| {
            let d = count_divisors(n);
            Some(if d > *max {
                *max = d;
                Some(n)
            } else {
                None
            })
        })
        .flatten()
        .take(20)
        .for_each(|n| print!("{} ", n));
    println!();
}
