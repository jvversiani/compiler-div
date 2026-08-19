// Rosetta Code task: Chowla numbers
// Source: https://rosettacode.org/wiki/Chowla_numbers#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// chowla(1) = 0
// chowla(2) = 0
// chowla(3) = 0
// chowla(4) = 2
// chowla(5) = 0
// chowla(6) = 5
// chowla(7) = 0
// chowla(8) = 6
// chowla(9) = 3
// chowla(10) = 7
// chowla(11) = 0
// chowla(12) = 15
// chowla(13) = 0
// chowla(14) = 9
// chowla(15) = 8
// chowla(16) = 14
// chowla(17) = 0
// chowla(18) = 20
// chowla(19) = 0
// chowla(20) = 21
// chowla(21) = 10
// chowla(22) = 13
// chowla(23) = 0
// chowla(24) = 35
// chowla(25) = 5
// chowla(26) = 15
// chowla(27) = 12
// chowla(28) = 27
// chowla(29) = 0
// chowla(30) = 41
// chowla(31) = 0
// chowla(32) = 30
// chowla(33) = 14
// chowla(34) = 19
// chowla(35) = 12
// chowla(36) = 54
// chowla(37) = 0
// Count of primes up to 100 = 25
// Count of primes up to 1000 = 168
// Count of primes up to 10000 = 1229
// Count of primes up to 100000 = 9592
// Count of primes up to 1000000 = 78498
// Count of primes up to 10000000 = 664579
// 6 is a number that is perfect
// 28 is a number that is perfect
// 496 is a number that is perfect
// 8128 is a number that is perfect
// 33550336 is a number that is perfect
// There are 5 perfect numbers <= 35,000,000
// =======================

fn chowla(n: usize) -> usize {
    let mut sum = 0;
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            sum += i;
            let j = n / i;
            if i != j {
                sum += j;
            }
        }
        i += 1;
    }
    sum
}

fn sieve(limit: usize) -> Vec<bool> {
    let mut c = vec![false; limit];
    let mut i = 3;
    while i * i < limit {
        if !c[i] && chowla(i) == 0 {
            let mut j = 3 * i;
            while j < limit {
                c[j] = true;
                j += 2 * i;
            }
        }
        i += 2;
    }
    c
}

fn main() {
    for i in 1..=37 {
        println!("chowla({}) = {}", i, chowla(i));
    }

    let mut count = 1;
    let limit = 1e7 as usize;
    let mut power = 100;
    let c = sieve(limit);
    for i in (3..limit).step_by(2) {
        if !c[i] {
            count += 1;
        }
        if i == power - 1 {
            println!("Count of primes up to {} = {}", power, count);
            power *= 10;
        }
    }

    count = 0;
    let limit = 35000000;
    let mut k = 2;
    let mut kk = 3;
    loop {
        let p = k * kk;
        if p > limit {
            break;
        }
        if chowla(p) == p - 1 {
            println!("{} is a number that is perfect", p);
            count += 1;
        }
        k = kk + 1;
        kk += k;
    }
    println!("There are {} perfect numbers <= 35,000,000", count);
}
