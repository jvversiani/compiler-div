// Rosetta Code task: Rice coding
// Source: https://rosettacode.org/wiki/Rice_coding#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Base Rice Coding:
// 0 -> 000 -> 0
// 1 -> 001 -> 1
// 2 -> 010 -> 2
// 3 -> 011 -> 3
// 4 -> 1000 -> 4
// 5 -> 1001 -> 5
// 6 -> 1010 -> 6
// 7 -> 1011 -> 7
// 8 -> 11000 -> 8
// 9 -> 11001 -> 9
// 10 -> 11010 -> 10
// Extended Rice Coding:
// -10 -> 1111011 -> -10
// -9 -> 1111001 -> -9
// -8 -> 111011 -> -8
// -7 -> 111001 -> -7
// -6 -> 11011 -> -6
// -5 -> 11001 -> -5
// -4 -> 1011 -> -4
// -3 -> 1001 -> -3
// -2 -> 011 -> -2
// -1 -> 001 -> -1
// 0 -> 000 -> 0
// 1 -> 010 -> 1
// 2 -> 1000 -> 2
// 3 -> 1010 -> 3
// 4 -> 11000 -> 4
// 5 -> 11010 -> 5
// 6 -> 111000 -> 6
// 7 -> 111010 -> 7
// 8 -> 1111000 -> 8
// 9 -> 1111010 -> 9
// 10 -> 11111000 -> 10
// =======================

fn rice_encode(n: i32, k: u32, extended: bool) -> String {
    let mut value = n;
    if extended {
        value = if n < 0 { -2 * n - 1 } else { 2 * n };
    }
    
    assert!(value >= 0, "n must be non-negative");
    
    let m = 2_i32.pow(k);
    let q = value / m;
    let r = value % m;
    
    // Create q ones
    let ones = "1".repeat(q as usize);
    
    // Format r as binary with k+1 bits
    let r_binary = format!("{:0width$b}", r, width = (k + 1) as usize);
    
    ones + &r_binary
}

fn rice_decode(a: &str, k: u32, extended: bool) -> i32 {
    let m = 2_i32.pow(k);
    
    // Find the first '0'
    let q = a.find('0').unwrap_or(0);
    
    // Parse the remainder
    let r = i32::from_str_radix(&a[q..], 2).unwrap();
    
    let mut i = (q as i32) * m + r;
    
    if extended {
        i = if i % 2 != 0 { -((i + 1) / 2) } else { i / 2 };
    }
    
    i
}

fn main() {
    println!("Base Rice Coding:");
    for n in 0..=10 {
        let s = rice_encode(n, 2, false);
        println!("{} -> {} -> {}", n, s, rice_decode(&s, 2, false));
    }
    
    println!("Extended Rice Coding:");
    for n in -10..=10 {
        let s = rice_encode(n, 2, true);
        println!("{} -> {} -> {}", n, s, rice_decode(&s, 2, true));
    }
}
