// Rosetta Code task: Greatest prime dividing the n-th cubefree number
// Source: https://rosettacode.org/wiki/Greatest_prime_dividing_the_n-th_cubefree_number#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// The first 100 terms of a370833 are:
//   1   2   3   2   5   3   7   3   5  11
//   3  13   7   5  17   3  19   5   7  11
//  23   5  13   7  29   5  31  11  17   7
//   3  37  19  13  41   7  43  11   5  23
//  47   7   5  17  13  53  11  19  29  59
//   5  61  31   7  13  11  67  17  23   7
//  71  73  37   5  19  11  13  79  41  83
//   7  17  43  29  89   5  13  23  31  47
//  19  97   7  11   5 101  17 103   7  53
// 107 109  11  37 113  19  23  29  13  59
//
// The 1000th term of a370833 is 109
// The 10000th term of a370833 is 101
// =======================

fn prime_factors(mut n: u32) -> Vec<u32> {
    let mut factors = Vec::new();
    
    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }
    
    let mut i = 3;
    while i <= (n as f64).sqrt() as u32 {
        while n % i == 0 {
            factors.push(i);
            n /= i;
        }
        i += 2;
    }
    
    if n > 2 {
        factors.push(n);
    }
    
    factors
}

fn main() {
    const MAXIMUM: u32 = 10_000;
    let mut count = 1;
    let mut i = 2;
    const LOWER_LIMIT: u32 = 100;
    let mut upper_limit = 1000;
    let mut first_hundred = vec![1];

    while count < MAXIMUM {
        let factors = prime_factors(i);
        let mut cube_free = true;

        if factors.len() >= 3 {
            for j in 2..factors.len() {
                if factors[j - 2] == factors[j - 1] && factors[j - 1] == factors[j] {
                    cube_free = false;
                    break;
                }
            }
        }

        if cube_free {
            if count < LOWER_LIMIT {
                if let Some(&last_factor) = factors.last() {
                    first_hundred.push(last_factor);
                }
            }
            count += 1;
            
            if count == LOWER_LIMIT {
                println!("The first {} terms of a370833 are:", LOWER_LIMIT);
                for (idx, &term) in first_hundred.iter().enumerate() {
                    print!("{:3}", term);
                    if idx % 10 == 9 {
                        println!();
                    } else {
                        print!(" ");
                    }
                }
                println!();
            } else if count == upper_limit {
                if let Some(&last_factor) = factors.last() {
                    println!("The {}th term of a370833 is {}", count, last_factor);
                }
                upper_limit *= 10;
            }
        }

        i += 1;
    }
}
