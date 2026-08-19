// Rosetta Code task: Jacobi symbol
// Source: https://rosettacode.org/wiki/Jacobi_symbol#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// n\k|  0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20
// -------------------------------------------------------------------
//  1 |  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1  1
//  3 |  0  1 -1  0  1 -1  0  1 -1  0  1 -1  0  1 -1  0  1 -1  0  1 -1
//  5 |  0  1 -1 -1  1  0  1 -1 -1  1  0  1 -1 -1  1  0  1 -1 -1  1  0
//  7 |  0  1  1 -1  1 -1 -1  0  1  1 -1  1 -1 -1  0  1  1 -1  1 -1 -1
//  9 |  0  1  1  0  1  1  0  1  1  0  1  1  0  1  1  0  1  1  0  1  1
// 11 |  0  1 -1  1  1  1 -1 -1 -1  1 -1  0  1 -1  1  1  1 -1 -1 -1  1
// 13 |  0  1 -1  1  1 -1 -1 -1 -1  1  1 -1  1  0  1 -1  1  1 -1 -1 -1
// 15 |  0  1  1  0  1  0  0 -1  1  0  0 -1  0 -1 -1  0  1  1  0  1  0
// 17 |  0  1  1 -1  1 -1 -1 -1  1  1 -1 -1 -1  1 -1  1  1  0  1  1 -1
// 19 |  0  1 -1 -1  1  1  1  1 -1  1 -1  1 -1 -1 -1 -1  1  1 -1  0  1
// 21 |  0  1 -1  0  1  1  0  0 -1  0 -1 -1  0 -1  0  0  1  1  0 -1  1
// =======================

fn jacobi(mut n: i32, mut k: i32) -> i32 {
    assert!(k > 0 && k % 2 == 1);
    n %= k;
    let mut t = 1;
    while n != 0 {
        while n % 2 == 0 {
            n /= 2;
            let r = k % 8;
            if r == 3 || r == 5 {
                t = -t;
            }
        }
        std::mem::swap(&mut n, &mut k);
        if n % 4 == 3 && k % 4 == 3 {
            t = -t;
        }
        n %= k;
    }
    if k == 1 {
        t
    } else {
        0
    }
}

fn print_table(kmax: i32, nmax: i32) {
    print!("n\\k|");
    for k in 0..=kmax {
        print!(" {:2}", k);
    }
    print!("\n----");
    for _ in 0..=kmax {
        print!("---");
    }
    println!();
    for n in (1..=nmax).step_by(2) {
        print!("{:2} |", n);
        for k in 0..=kmax {
            print!(" {:2}", jacobi(k, n));
        }
        println!();
    }
}

fn main() {
    print_table(20, 21);
}
