// Rosetta Code task: Prime triangle
// Source: https://rosettacode.org/wiki/Prime_triangle#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
//  1  2
//  1  2  3
//  1  2  3  4
//  1  4  3  2  5
//  1  4  3  2  5  6
//  1  4  3  2  5  6  7
//  1  2  3  4  7  6  5  8
//  1  2  3  4  7  6  5  8  9
//  1  2  3  4  7  6  5  8  9 10
//  1  2  3  4  7 10  9  8  5  6 11
//  1  2  3  4  7 10  9  8  5  6 11 12
//  1  2  3  4  7  6  5 12 11  8  9 10 13
//  1  2  3  4  7  6 13 10  9  8 11 12  5 14
//  1  2  3  4  7  6 13 10  9  8 11 12  5 14 15
//  1  2  3  4  7  6  5 12 11  8 15 14  9 10 13 16
//  1  2  3  4  7  6  5 12 11  8  9 10 13 16 15 14 17
//  1  2  3  4  7  6  5  8  9 10 13 16 15 14 17 12 11 18
//  1  2  3  4  7  6  5  8  9 10 13 16 15 14 17 12 11 18 19
//  1  2  3  4  7  6  5  8  9 10 13 16 15 14 17 12 19 18 11 20
//
// 1 1 1 1 1 2 4 7 24 80 216 648 1304 3392 13808 59448 155464 480728 1588162
// =======================

fn is_prime(n: u32) -> bool {
    assert!(n < 64);
    ((1u64 << n) & 0x28208a20a08a28ac) != 0
}

fn prime_triangle_row(a: &mut [u32]) -> bool {
    if a.len() == 2 {
        return is_prime(a[0] + a[1]);
    }
    for i in (1..a.len() - 1).step_by(2) {
        if is_prime(a[0] + a[i]) {
            a.swap(i, 1);
            if prime_triangle_row(&mut a[1..]) {
                return true;
            }
            a.swap(i, 1);
        }
    }
    false
}

fn prime_triangle_count(a: &mut [u32]) -> u32 {
    let mut count = 0;
    if a.len() == 2 {
        if is_prime(a[0] + a[1]) {
            count += 1;
        }
    } else {
        for i in (1..a.len() - 1).step_by(2) {
            if is_prime(a[0] + a[i]) {
                a.swap(i, 1);
                count += prime_triangle_count(&mut a[1..]);
                a.swap(i, 1);
            }
        }
    }
    count
}

fn print(a: &[u32]) {
    if a.is_empty() {
        return;
    }
    print!("{:2}", a[0]);
    for x in &a[1..] {
        print!(" {:2}", x);
    }
    println!();
}

fn main() {
    for n in 2..21 {
        let mut a: Vec<u32> = (1..=n).collect();
        if prime_triangle_row(&mut a) {
            print(&a);
        }
    }
    println!();
    for n in 2..21 {
        let mut a: Vec<u32> = (1..=n).collect();
        if n > 2 {
            print!(" ");
        }
        print!("{}", prime_triangle_count(&mut a));
    }
    println!();
}
