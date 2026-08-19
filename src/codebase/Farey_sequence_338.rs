// Rosetta Code task: Farey sequence
// Source: https://rosettacode.org/wiki/Farey_sequence#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 1: 0/1 1/1
// 2: 0/1 1/2 1/1
// 3: 0/1 1/3 1/2 2/3 1/1
// 4: 0/1 1/4 1/3 1/2 2/3 3/4 1/1
// 5: 0/1 1/5 1/4 1/3 2/5 1/2 3/5 2/3 3/4 4/5 1/1
// 6: 0/1 1/6 1/5 1/4 1/3 2/5 1/2 3/5 2/3 3/4 4/5 5/6 1/1
// 7: 0/1 1/7 1/6 1/5 1/4 2/7 1/3 2/5 3/7 1/2 4/7 3/5 2/3 5/7 3/4 4/5 5/6 6/7 1/1
// 8: 0/1 1/8 1/7 1/6 1/5 1/4 2/7 1/3 3/8 2/5 3/7 1/2 4/7 3/5 5/8 2/3 5/7 3/4 4/5 5/6 6/7 7/8 1/1
// 9: 0/1 1/9 1/8 1/7 1/6 1/5 2/9 1/4 2/7 1/3 3/8 2/5 3/7 4/9 1/2 5/9 4/7 3/5 5/8 2/3 5/7 3/4 7/9 4/5 5/6 6/7 7/8 8/9 1/1
// 10: 0/1 1/10 1/9 1/8 1/7 1/6 1/5 2/9 1/4 2/7 3/10 1/3 3/8 2/5 3/7 4/9 1/2 5/9 4/7 3/5 5/8 2/3 7/10 5/7 3/4 7/9 4/5 5/6 6/7 7/8 8/9 9/10 1/1
// 11: 0/1 1/11 1/10 1/9 1/8 1/7 1/6 2/11 1/5 2/9 1/4 3/11 2/7 3/10 1/3 4/11 3/8 2/5 3/7 4/9 5/11 1/2 6/11 5/9 4/7 3/5 5/8 7/11 2/3 7/10 5/7 8/11 3/4 7/9 4/5 9/11 5/6 6/7 7/8 8/9 9/10 10/11 1/1
// 100: 3045
// 200: 12233
// 300: 27399
// 400: 48679
// 500: 76117
// 600: 109501
// 700: 149019
// 800: 194751
// 900: 246327
// 1000: 304193
// =======================

#[derive(Copy, Clone)]
struct Fraction {
    numerator: u32,
    denominator: u32,
}

use std::fmt;

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

impl Fraction {
    fn new(n: u32, d: u32) -> Fraction {
        Fraction {
            numerator: n,
            denominator: d,
        }
    }
}

fn farey_sequence(n: u32) -> impl std::iter::Iterator<Item = Fraction> {
    let mut a = 0;
    let mut b = 1;
    let mut c = 1;
    let mut d = n;
    std::iter::from_fn(move || {
        if a > n {
            return None;
        }
        let result = Fraction::new(a, b);
        let k = (n + b) / d;
        let next_c = k * c - a;
        let next_d = k * d - b;
        a = c;
        b = d;
        c = next_c;
        d = next_d;
        Some(result)
    })
}

fn main() {
    for n in 1..=11 {
        print!("{}:", n);
        for f in farey_sequence(n) {
            print!(" {}", f);
        }
        println!();
    }
    for n in (100..=1000).step_by(100) {
        println!("{}: {}", n, farey_sequence(n).count());
    }
}
