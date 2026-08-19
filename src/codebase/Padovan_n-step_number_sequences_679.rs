// Rosetta Code task: Padovan n-step number sequences
// Source: https://rosettacode.org/wiki/Padovan_n-step_number_sequences#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
//
// N=2: 1,1,1,2,2,3,4,5,7,9,12,16,21,28,37,
// N=3: 1,1,1,2,3,4,6,9,13,19,28,41,60,88,129,
// N=4: 1,1,1,2,3,5,7,11,17,26,40,61,94,144,221,
// N=5: 1,1,1,2,3,5,8,12,19,30,47,74,116,182,286,
// N=6: 1,1,1,2,3,5,8,13,20,32,51,81,129,205,326,
// N=7: 1,1,1,2,3,5,8,13,21,33,53,85,136,218,349,
// N=8: 1,1,1,2,3,5,8,13,21,34,54,87,140,225,362,
// =======================

fn padovan(n: u64, x: u64) -> u64 {
    if n < 2 {
        return 0;
    }

    match n {
        2 if x <= n + 1 => 1,
        2 => padovan(n, x - 2) + padovan(n, x - 3),
        _ if x <= n + 1 => padovan(n - 1, x),
        _ => ((x - n - 1)..(x - 1)).fold(0, |acc, value| acc + padovan(n, value)),
    }
}
fn main() {
    (2..=8).for_each(|n| {
        print!("\nN={}: ", n);
        (1..=15).for_each(|x| print!("{},", padovan(n, x)))
    });
}
