// Rosetta Code task: Compile-time calculation
// Source: https://rosettacode.org/wiki/Compile-time_calculation#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Factorial of 10 is 3628800.
// =======================

fn factorial(n: i64) -> i64 {
    let mut total = 1;
    for i in 1..n+1 {
        total *= i;
    }
    return total;
}

fn main() {
    println!("Factorial of 10 is {}.", factorial(10));
}
