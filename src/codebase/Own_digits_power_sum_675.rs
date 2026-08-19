// Rosetta Code task: Own digits power sum
// Source: https://rosettacode.org/wiki/Own_digits_power_sum#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// [153, 370, 371, 407, 1634, 8208, 9474, 54748, 92727, 93084]
// =======================

fn is_own_digits_power_sum(n: u32) -> bool {
    let n_str = n.to_string();
    n_str.chars()
        .map(|c| {
            let digit = c.to_digit(10).unwrap();
            digit.pow(n_str.len() as u32)
        })
        .sum::<u32>()
        == n
}

fn main() {
    let result: Vec<u32> = (10u32.pow(2)..10u32.pow(5))
        .filter(|&n| is_own_digits_power_sum(n))
        .collect();

    println!("{:?}", result);
}
