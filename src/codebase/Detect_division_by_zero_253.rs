// Rosetta Code task: Detect division by zero
// Source: https://rosettacode.org/wiki/Detect_division_by_zero#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 5 / 4 = 1
// 4 / 0 results in a division by zero
// =======================

fn test_division(numerator: u32, denominator: u32) {
    match numerator.checked_div(denominator) {
        Some(result) => println!("{} / {} = {}", numerator, denominator, result),
        None => println!("{} / {} results in a division by zero", numerator, denominator)
    }
}

fn main() {
    test_division(5, 4);
    test_division(4, 0);
}
