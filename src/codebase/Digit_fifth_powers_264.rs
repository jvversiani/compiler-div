// Rosetta Code task: Digit fifth powers
// Source: https://rosettacode.org/wiki/Digit_fifth_powers#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 443839
// =======================

// Helper function to extract digits of a number
fn digits(mut n: i32) -> Vec<i32> {
    let mut result = Vec::new();
    while n > 0 {
        result.push(n % 10);
        n /= 10;
    }
    result
}


fn main() {
    let upper_limit = 6 * i32::pow(9, 5);
    let result: i32 = (2..upper_limit)
        .filter(|&n| {
            let sum: i32 = digits(n).iter().map(|&d| i32::pow(d, 5)).sum();
            sum == n
        })
        .sum();
    println!("{}", result);
}
