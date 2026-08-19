// Rosetta Code task: Harshad or Niven series
// Source: https://rosettacode.org/wiki/Harshad_or_Niven_series#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Harshad : 1
// Harshad : 2
// Harshad : 3
// Harshad : 4
// Harshad : 5
// Harshad : 6
// Harshad : 7
// Harshad : 8
// Harshad : 9
// Harshad : 10
// Harshad : 12
// Harshad : 18
// Harshad : 20
// Harshad : 21
// Harshad : 24
// Harshad : 27
// Harshad : 30
// Harshad : 36
// Harshad : 40
// Harshad : 42
// First Harshad bigger than 1_000 : 1002
// =======================

fn is_harshad (n : u32) -> bool {
    let sum_digits = n.to_string()
                      .chars()
                      .map(|c| c.to_digit(10).unwrap())
                      .fold(0, |a, b| a+b);
    n % sum_digits == 0
}

fn main() {
    for i in (1u32..).filter(|num| is_harshad(*num)).take(20) {
        println!("Harshad : {}", i);
    }
    for i in (1_001u32..).filter(|num| is_harshad(*num)).take(1) {
        println!("First Harshad bigger than 1_000 : {}", i);
    }
}
