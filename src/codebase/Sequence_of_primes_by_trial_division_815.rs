// Rosetta Code task: Sequence of primes by trial division
// Source: https://rosettacode.org/wiki/Sequence_of_primes_by_trial_division#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Primes below 100:
// [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97]
// =======================

fn is_prime(number: u32) -> bool {
    #[allow(clippy::cast_precision_loss)]
    let limit = (number as f32).sqrt() as u32 + 1;

    // We test if the number is divisible by any number up to the limit
    !(number < 2 || (2..limit).any(|x| number % x == 0))
}

fn main() {
    println!(
        "Primes below 100:\n{:?}",
        (0_u32..100).fold(vec![], |mut acc, number| {
            if is_prime(number) {
                acc.push(number)
            };
            acc
        })
    );
}
