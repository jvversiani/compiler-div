// Rosetta Code task: Population count
// Source: https://rosettacode.org/wiki/Population_count#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// pop count of 3^0, 3^1 ... 3^29:
// [1, 2, 2, 4, 3, 6, 6, 5, 6, 8, 9, 13, 10, 11, 14, 15, 11, 14, 14, 17, 17, 20, 19, 22, 16, 18, 24, 30, 25, 25]
//
// First 30 even pop count:
// [3, 5, 6, 9, 10, 12, 15, 17, 18, 20, 23, 24, 27, 29, 30, 33, 34, 36, 39, 40, 43, 45, 46, 48, 51, 53, 54, 57, 58, 60]
//
// First 30 odd pop count:
// [1, 2, 4, 7, 8, 11, 13, 14, 16, 19, 21, 22, 25, 26, 28, 31, 32, 35, 37, 38, 41, 42, 44, 47, 49, 50, 52, 55, 56, 59]
// =======================

fn main() {
    let mut num = 1u64;
    let mut vec = Vec::new();
    for _ in 0..30 {
        vec.push(num.count_ones());
        num *= 3;
    }
    println!("pop count of 3^0, 3^1 ... 3^29:\n{:?}",vec);
    let mut even = Vec::new();
    let mut odd  = Vec::new();
    num = 1;
    while even.len() < 30 || odd.len() < 30 {
        match 0 == num.count_ones()%2 {
            true if even.len() < 30 => even.push(num),
            false if odd.len() < 30 => odd.push(num),
            _                       => {}
        }
        num += 1;
    }
    println!("\nFirst 30 even pop count:\n{:?}",even);
    println!("\nFirst 30 odd pop count:\n{:?}",odd);
}
