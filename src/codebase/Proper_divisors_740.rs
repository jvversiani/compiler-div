// Rosetta Code task: Proper divisors
// Source: https://rosettacode.org/wiki/Proper_divisors#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Proper divisors of  1: []
// Proper divisors of  2: [1]
// Proper divisors of  3: [1]
// Proper divisors of  4: [1, 2]
// Proper divisors of  5: [1]
// Proper divisors of  6: [1, 2, 3]
// Proper divisors of  7: [1]
// Proper divisors of  8: [1, 2, 4]
// Proper divisors of  9: [1, 3]
// Proper divisors of 10: [1, 2, 5]
// In 1 to 20000, 15120 has the most proper divisors at 79
// =======================

trait ProperDivisors {
    fn proper_divisors(&self) -> Option<Vec<u64>>;
}

impl ProperDivisors for u64 {
    fn proper_divisors(&self) -> Option<Vec<u64>> {
        if self.le(&1) {
            return None;
        }
        let mut divisors: Vec<u64> = Vec::new();

        for i in 1..*self {
            if *self % i == 0 {
                divisors.push(i);
            }
        }
        Option::from(divisors)
    }
}

fn main() {
    for i in 1..11 {
        println!("Proper divisors of {:2}: {:?}", i,
                 i.proper_divisors().unwrap_or(vec![]));
    }

    let mut most_idx: u64 = 0;
    let mut most_divisors: Vec<u64> = Vec::new();
    for i in 1..20_001 {
        let divs = i.proper_divisors().unwrap_or(vec![]);
        if divs.len() > most_divisors.len() {
            most_divisors = divs;
            most_idx = i;
        }
    }
    println!("In 1 to 20000, {} has the most proper divisors at {}", most_idx,
             most_divisors.len());
}
