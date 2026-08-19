// Rosetta Code task: Munchausen numbers
// Source: https://rosettacode.org/wiki/Munchausen_numbers#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Munchausen numbers below 5_000 : [1, 3435]
// =======================

fn main() {
    let mut solutions = Vec::new();

    for num in 1..5_000 {
        let power_sum = num.to_string()
            .chars()
            .map(|c| {
                let digit = c.to_digit(10).unwrap();
                (digit as f64).powi(digit as i32) as usize
            })
            .sum::<usize>();

        if power_sum == num {
            solutions.push(num);
        }
    }

    println!("Munchausen numbers below 5_000 : {:?}", solutions);
}
