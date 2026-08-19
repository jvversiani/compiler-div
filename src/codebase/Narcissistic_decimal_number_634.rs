// Rosetta Code task: Narcissistic decimal number
// Source: https://rosettacode.org/wiki/Narcissistic_decimal_number#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 0
// 1
// 2
// 3
// 4
// 5
// 6
// 7
// 8
// 9
// 153
// 370
// 371
// 407
// 1634
// 8208
// 9474
// 54748
// 92727
// 93084
// 548834
// 1741725
// 4210818
// 9800817
// 9926315
// =======================

fn is_narcissistic(x: u32) -> bool {
    let digits: Vec<u32> = x
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    digits
        .iter()
        .map(|d| d.pow(digits.len() as u32))
        .sum::<u32>()
        == x
}

fn main() {
    let mut counter = 0;
    let mut i = 0;
    while counter < 25 {
        if is_narcissistic(i) {
            println!("{}", i);
            counter += 1;
        }
        i += 1;
    }
}
