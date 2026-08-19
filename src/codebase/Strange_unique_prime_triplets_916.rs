// Rosetta Code task: Strange unique prime triplets
// Source: https://rosettacode.org/wiki/Strange_unique_prime_triplets#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Strange unique prime triplets < 30:
//  3 +  5 + 11 = 19
//  3 +  5 + 23 = 31
//  3 +  5 + 29 = 37
//  3 +  7 + 13 = 23
//  3 +  7 + 19 = 29
//  3 + 11 + 17 = 31
//  3 + 11 + 23 = 37
//  3 + 11 + 29 = 43
//  3 + 17 + 23 = 43
//  5 +  7 + 11 = 23
//  5 +  7 + 17 = 29
//  5 +  7 + 19 = 31
//  5 +  7 + 29 = 41
//  5 + 11 + 13 = 29
//  5 + 13 + 19 = 37
//  5 + 13 + 23 = 41
//  5 + 13 + 29 = 47
//  5 + 17 + 19 = 41
//  5 + 19 + 23 = 47
//  5 + 19 + 29 = 53
//  7 + 11 + 13 = 31
//  7 + 11 + 19 = 37
//  7 + 11 + 23 = 41
//  7 + 11 + 29 = 47
//  7 + 13 + 17 = 37
//  7 + 13 + 23 = 43
//  7 + 17 + 19 = 43
//  7 + 17 + 23 = 47
//  7 + 17 + 29 = 53
//  7 + 23 + 29 = 59
// 11 + 13 + 17 = 41
// 11 + 13 + 19 = 43
// 11 + 13 + 23 = 47
// 11 + 13 + 29 = 53
// 11 + 17 + 19 = 47
// 11 + 19 + 23 = 53
// 11 + 19 + 29 = 59
// 13 + 17 + 23 = 53
// 13 + 17 + 29 = 59
// 13 + 19 + 29 = 61
// 17 + 19 + 23 = 59
// 19 + 23 + 29 = 71
// Count of strange unique prime triplets < 30 is 42.
// Count of strange unique prime triplets < 1000 is 241580.
// =======================

fn prime_sieve(limit: usize) -> Vec<bool> {
    let mut sieve = vec![true; limit];
    if limit > 0 {
        sieve[0] = false;
    }
    if limit > 1 {
        sieve[1] = false;
    }
    for i in (4..limit).step_by(2) {
        sieve[i] = false;
    }
    let mut p = 3;
    loop {
        let mut q = p * p;
        if q >= limit {
            break;
        }
        if sieve[p] {
            let inc = 2 * p;
            while q < limit {
                sieve[q] = false;
                q += inc;
            }
        }
        p += 2;
    }
    sieve
}

fn strange_unique_prime_triplets(limit: usize, verbose: bool) {
    if limit < 6 {
        return;
    }
    let mut primes = Vec::new();
    let sieve = prime_sieve(limit * 3);
    for p in (3..limit).step_by(2) {
        if sieve[p] {
            primes.push(p);
        }
    }
    if verbose {
        println!("Strange unique prime triplets < {}:", limit);
    }
    let mut count = 0;
    let n = primes.len();
    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            for k in j + 1..n {
                let sum = primes[i] + primes[j] + primes[k];
                if sieve[sum] {
                    count += 1;
                    if verbose {
                        println!(
                            "{:2} + {:2} + {:2} = {:2}",
                            primes[i], primes[j], primes[k], sum
                        );
                    }
                }
            }
        }
    }
    println!(
        "Count of strange unique prime triplets < {} is {}.",
        limit, count
    );
}

fn main() {
    strange_unique_prime_triplets(30, true);
    strange_unique_prime_triplets(1000, false);
}
