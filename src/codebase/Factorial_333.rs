// Rosetta Code task: Factorial
// Source: https://rosettacode.org/wiki/Factorial#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 1
// 2
// 6
// 24
// 120
// 720
// 5040
// 40320
// 362880
// 1
// 2
// 6
// 24
// 120
// 720
// 5040
// 40320
// 362880
// =======================

fn factorial_recursive (n: u64) -> u64 {
    match n {
        0 => 1,
        _ => n * factorial_recursive(n-1)
    }
}

fn factorial_iterative(n: u64) -> u64 {
    (1..=n).product()
}

fn main () {
    for i in 1..10 {
        println!("{}", factorial_recursive(i))
    }
    for i in 1..10 {
        println!("{}", factorial_iterative(i))
    }
}
