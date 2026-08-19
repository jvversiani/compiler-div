// Rosetta Code task: Find minimum number of coins that make a given value
// Source: https://rosettacode.org/wiki/Find_minimum_number_of_coins_that_make_a_given_value#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// The minimum number of coins needed to make a value of 988 is as follows:
//   200 x 4
//   100 x 1
//   50 x 1
//   20 x 1
//   10 x 1
//   5 x 1
//   2 x 1
//   1 x 1
//
// A total of 11 coins in all.
// =======================

fn main() {
    let denoms = vec![200, 100, 50, 20, 10, 5, 2, 1];
    let mut coins = 0;
    let amount = 988;
    let mut remaining = 988;
    println!("The minimum number of coins needed to make a value of {} is as follows:", amount);
    for denom in denoms.iter() {
        let n = remaining / denom;
        if n > 0 {
            coins += n;
            println!("  {} x {}", denom, n);
            remaining %= denom;
            if remaining == 0 {
                break;
            }
        }
    }
    println!("\nA total of {} coins in all.", coins);
}
