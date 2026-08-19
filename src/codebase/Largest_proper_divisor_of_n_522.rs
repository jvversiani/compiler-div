// Rosetta Code task: Largest proper divisor of n
// Source: https://rosettacode.org/wiki/Largest_proper_divisor_of_n#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// The largest proper divisors for numbers in the interval [1, 100] are:
//  1   1   1   2   1   3   1   4   3   5  
//  1   6   1   7   5   8   1   9   1  10  
//  7  11   1  12   5  13   9  14   1  15  
//  1  16  11  17   7  18   1  19  13  20  
//  1  21   1  22  15  23   1  24   7  25  
// 17  26   1  27  11  28  19  29   1  30  
//  1  31  21  32  13  33   1  34  23  35  
//  1  36   1  37  25  38  11  39   1  40  
// 27  41   1  42  17  43  29  44   1  45  
// 13  46  31  47  19  48   1  49  33  50
// =======================

fn largest_proper_divisor(n: i32) -> i32 {
    for i in 2..=(n as f64).sqrt() as i32 {
        if n % i == 0 {
            return n / i;
        }
    }
    1
}

fn main() {
    println!("The largest proper divisors for numbers in the interval [1, 100] are:");
    print!(" 1  ");
    for n in 2..=100 {
        if n % 2 == 0 {
            print!("{:2}  ", n / 2);
        } else {
            print!("{:2}  ", largest_proper_divisor(n));
        }
        if n % 10 == 0 {
            println!();
        }
    }
}
