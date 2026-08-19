// Rosetta Code task: Magnanimous numbers
// Source: https://rosettacode.org/wiki/Magnanimous_numbers#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// First 45 magnanimous numbers:
//   0   1   2   3   4   5   6   7   8   9  11  12  14  16  20 
//  21  23  25  29  30  32  34  38  41  43  47  49  50  52  56 
//  58  61  65  67  70  74  76  83  85  89  92  94  98 101 110 
//
// 241st through 250th magnanimous numbers:
// 17992 19972 20209 20261 20861 22061 22201 22801 22885 24407 
//
// 391st through 400th magnanimous numbers:
// 486685 488489 515116 533176 551558 559952 595592 595598 600881 602081
// =======================

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n % 2 == 0 {
        return n == 2;
    }
    if n % 3 == 0 {
        return n == 3;
    }
    let mut p = 5;
    while p * p <= n {
        if n % p == 0 {
            return false;
        }
        p += 2;
        if n % p == 0 {
            return false;
        }
        p += 4;
    }
    true
}

fn is_magnanimous(n: u32) -> bool {
    let mut p: u32 = 10;
    while n >= p {
        if !is_prime(n % p + n / p) {
            return false;
        }
        p *= 10;
    }
    true
}

fn main() {
    let mut m = (0..).filter(|x| is_magnanimous(*x)).take(400);
    println!("First 45 magnanimous numbers:");
    for (i, n) in m.by_ref().take(45).enumerate() {
        if i > 0 && i % 15 == 0 {
            println!();
        }
        print!("{:3} ", n);
    }
    println!("\n\n241st through 250th magnanimous numbers:");
    for n in m.by_ref().skip(195).take(10) {
        print!("{} ", n);
    }
    println!("\n\n391st through 400th magnanimous numbers:");
    for n in m.by_ref().skip(140) {
        print!("{} ", n);
    }
    println!();
}
