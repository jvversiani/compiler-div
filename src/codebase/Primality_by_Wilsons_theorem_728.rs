// Rosetta Code task: Primality by Wilson's theorem
// Source: https://rosettacode.org/wiki/Primality_by_Wilson's_theorem#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
//   n | prime?
// ------------
//   2 | true
//   3 | true
//   9 | false
//  15 | false
//  29 | true
//  37 | true
//  47 | true
//  57 | false
//  67 | true
//  77 | false
//  87 | false
//  97 | true
// 237 | false
// 409 | true
// 659 | true
//
// First 120 primes by Wilson's theorem:
//   2   3   5   7  11  13  17  19  23  29  31  37  41  43  47  53  59  61  67  71
//  73  79  83  89  97 101 103 107 109 113 127 131 137 139 149 151 157 163 167 173
// 179 181 191 193 197 199 211 223 227 229 233 239 241 251 257 263 269 271 277 281
// 283 293 307 311 313 317 331 337 347 349 353 359 367 373 379 383 389 397 401 409
// 419 421 431 433 439 443 449 457 461 463 467 479 487 491 499 503 509 521 523 541
// 547 557 563 569 571 577 587 593 599 601 607 613 617 619 631 641 643 647 653 659
//
// 1000th through 1015th primes:
// 7919 7927 7933 7937 7949 7951 7963 7993 8009 8011 8017 8039 8053 8059 8069 8081
// =======================

fn factorial_mod(mut n: u32, p: u32) -> u32 {
    let mut f = 1;
    while n != 0 && f != 0 {
        f = (f * n) % p;
        n -= 1;
    }
    f
}

fn is_prime(p: u32) -> bool {
    p > 1 && factorial_mod(p - 1, p) == p - 1
}

fn main() {
    println!("  n | prime?\n------------");
    for p in vec![2, 3, 9, 15, 29, 37, 47, 57, 67, 77, 87, 97, 237, 409, 659] {
        println!("{:>3} | {}", p, is_prime(p));
    }
    println!("\nFirst 120 primes by Wilson's theorem:");
    let mut n = 0;
    let mut p = 1;
    while n < 120 {
        if is_prime(p) {
            n += 1;
            print!("{:>3}{}", p, if n % 20 == 0 { '\n' } else { ' ' });
        }
        p += 1;
    }
    println!("\n1000th through 1015th primes:");
    let mut i = 0;
    while n < 1015 {
        if is_prime(p) {
            n += 1;
            if n >= 1000 {
                i += 1;
                print!("{:>3}{}", p, if i % 16 == 0 { '\n' } else { ' ' });
            }
        }
        p += 1;
    }
}
