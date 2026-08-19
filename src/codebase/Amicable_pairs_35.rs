// Rosetta Code task: Amicable pairs
// Source: https://rosettacode.org/wiki/Amicable_pairs#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 284 220
// 1210 1184
// 2924 2620
// 5564 5020
// 6368 6232
// 10856 10744
// 14595 12285
// 18416 17296
// =======================

fn sum_of_divisors(val: u32) -> u32 {
    (1..val/2+1).filter(|n| val % n == 0)
                .fold(0, |sum, n| sum + n)
}

fn main() {
    let iter = (1..20_000).map(|i| (i, sum_of_divisors(i)))
                          .filter(|&(i, div_sum)| i > div_sum);

    for (i, sum1) in iter {
        if sum_of_divisors(sum1) == i {
           println!("{} {}", i, sum1);
        }
    }
}
