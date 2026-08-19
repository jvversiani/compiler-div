// Rosetta Code task: Vile and Dopey numbers
// Source: https://rosettacode.org/wiki/Vile_and_Dopey_numbers#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// First 25 vile numbers:
// [1, 3, 4, 5, 7, 9, 11, 12, 13, 15, 16, 17, 19, 20, 21, 23, 25, 27, 28, 29, 31, 33, 35, 36, 37]
//
// First 25 dopey numbers:
// [2, 6, 8, 10, 14, 18, 22, 24, 26, 30, 32, 34, 38, 40, 42, 46, 50, 54, 56, 58, 62, 66, 70, 72, 74]
//
//    N   Viles Dopeys
// __________________
//    2      1      1
//    4      3      1
//    8      5      3
//   16     11      5
//   32     21     11
//   64     43     21
//  128     85     43
//  256    171     85
//  512    341    171
// 1024    683    341
// =======================

fn is_vile(n: u64) -> bool {
    n.trailing_zeros() % 2 == 0
}

fn is_dopey(n: u64) -> bool {
    n.trailing_zeros() % 2 != 0
}

fn main() {
    let vile_numbers: Vec<u64> = (1..=100)
        .filter(|&n| is_vile(n))
        .take(25)
        .collect();
    println!("First 25 vile numbers:");
    println!("{:?}", vile_numbers);

    let dopey_numbers: Vec<u64> = (1..=100)
        .filter(|&n| is_dopey(n))
        .take(25)
        .collect();
    println!("\nFirst 25 dopey numbers:");
    println!("{:?}", dopey_numbers);

    println!("\n   N   Viles Dopeys");
    println!("{}", "_".repeat(18));
    for i in 0..10 {
        let n = 2u64 << i;
        let viles_count = (1..=n).filter(|&x| is_vile(x)).count();
        let dopey_count = n as usize - viles_count;
        println!("{:>4} {:>6} {:>6}", n, viles_count, dopey_count);
    }
}
