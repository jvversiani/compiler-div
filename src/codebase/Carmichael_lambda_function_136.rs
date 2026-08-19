// Rosetta Code task: Carmichael lambda function
// Source: https://rosettacode.org/wiki/Carmichael_lambda_function#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
//  n   carmichael(n) iterations(n)
// --------------------------------
//  1         1             0
//  2         1             1
//  3         2             2
//  4         2             2
//  5         4             3
//  6         2             2
//  7         6             3
//  8         2             2
//  9         6             3
// 10         4             3
// 11        10             4
// 12         2             2
// 13        12             3
// 14         6             3
// 15         4             3
// 16         4             3
// 17        16             4
// 18         6             3
// 19        18             4
// 20         4             3
// 21         6             3
// 22        10             4
// 23        22             5
// 24         2             2
// 25        20             4
//
// Iterations to 1     n     lambda(n)
// -----------------------------------
//  0                  1            1
//  1                  2            1
//  2                  3            2
//  3                  5            4
//  4                 11           10
//  5                 23           22
//  6                 47           46
//  7                283          282
//  8                719          718
//  9               1439         1438
// 10               2879         2878
// 11              34549        34548
// 12             138197       138196
// 13             531441       354294
// 14            1594323      1062882
// 15            4782969      3188646
// =======================

use std::cmp;

#[derive(Debug, Clone)]
struct PrimePower {
    prime: u32,
    power: u32,
}

impl PrimePower {
    fn new(prime: u32, power: u32) -> Self {
        PrimePower { prime, power }
    }
}

fn prime_powers(number: u32) -> Vec<PrimePower> {
    let mut powers = Vec::new();
    let mut n = number;
    
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            powers.push(PrimePower::new(i, 0));
            while n % i == 0 {
                powers.last_mut().unwrap().power += 1;
                n /= i;
            }
        }
        i += 1;
    }
    
    if n > 1 {
        powers.push(PrimePower::new(n, 1));
    }
    
    powers
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u32, b: u32) -> u32 {
    (a / gcd(a, b)) * b
}

fn carmichael_lambda(number: u32) -> u32 {
    if number == 1 {
        return 1;
    }
    
    let powers = prime_powers(number);
    let mut result = 1;
    
    for prime_power in powers {
        let mut car = (prime_power.prime - 1) * prime_power.prime.pow(prime_power.power - 1);
        if prime_power.prime == 2 && prime_power.power >= 3 {
            car /= 2;
        }
        result = lcm(result, car);
    }
    
    result
}

fn count_iterations_to_one(n: u32) -> u32 {
    if n <= 1 {
        0
    } else {
        count_iterations_to_one(carmichael_lambda(n)) + 1
    }
}

fn main() {
    println!(" n   carmichael(n) iterations(n)");
    println!("--------------------------------");
    
    for i in 1..=25 {
        println!("{:2}{:10}{:14}", 
                 i, 
                 carmichael_lambda(i), 
                 count_iterations_to_one(i));
    }
    
    println!();
    println!("Iterations to 1     n     lambda(n)");
    println!("-----------------------------------");
    
    let mut n = 1;
    for i in 0..=15 {
        while count_iterations_to_one(n) != i {
            n += 1;
        }
        println!("{:2}{:19}{:13}", i, n, carmichael_lambda(n));
    }
}
