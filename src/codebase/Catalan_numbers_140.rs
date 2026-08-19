// Rosetta Code task: Catalan numbers
// Source: https://rosettacode.org/wiki/Catalan_numbers#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// c_n(1) = 1
// c_n(2) = 2
// c_n(3) = 5
// c_n(4) = 14
// c_n(5) = 42
// c_n(6) = 132
// c_n(7) = 429
// c_n(8) = 1430
// c_n(9) = 4862
// c_n(10) = 16796
// c_n(11) = 58786
// c_n(12) = 208012
// c_n(13) = 742900
// c_n(14) = 2674440
// c_n(15) = 9694845
// =======================

fn c_n(n: u64) -> u64 {
    match n {
        0 => 1,
        _ => c_n(n - 1) * 2 * (2 * n - 1) / (n + 1)
    }
}

fn main() {
    for i in 1..16 {
        println!("c_n({}) = {}", i, c_n(i));
    }
}
