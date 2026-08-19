// Rosetta Code task: Abundant, deficient and perfect number classifications
// Source: https://rosettacode.org/wiki/Abundant,_deficient_and_perfect_number_classifications#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// deficient:      15043
// perfect:            4
// abundant:        4953
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
    // deficient starts at 1 because 1 is deficient but proper_divisors returns
    // and empty Vec
    let (mut abundant, mut deficient, mut perfect) = (0u32, 1u32, 0u32);
    for i in 1..20_001 {
        if let Some(divisors) = i.proper_divisors() {
            let sum: u64 = divisors.iter().sum();
            if sum < i {
                deficient += 1
            } else if sum > i {
                abundant += 1
            } else {
                perfect += 1
            }
        }
    }
    println!("deficient:\t{:5}\nperfect:\t{:5}\nabundant:\t{:5}",
             deficient, perfect, abundant);
}
