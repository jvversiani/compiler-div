// Rosetta Code task: Sequence of non-squares
// Source: https://rosettacode.org/wiki/Sequence_of_non-squares#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// [2, 3, 5, 6, 7, 8, 10, 11, 12, 13, 14, 15, 17, 18, 19, 20, 21, 22, 23, 24, 26, 27]
// 0 unexpected squares found
// =======================

fn f(n: i64) -> i64 {
    n + (0.5 + (n as f64).sqrt()) as i64
}

fn is_sqr(n: i64) -> bool {
    let a = (n as f64).sqrt() as i64;
    n == a * a || n == (a+1) * (a+1) || n == (a-1) * (a-1) 
}

fn main() {
    println!( "{:?}", (1..23).map(|n| f(n)).collect::<Vec<i64>>() );
    let count = (1..1_000_000).map(|n| f(n)).filter(|&n| is_sqr(n)).count();
    println!("{} unexpected squares found", count);
}
