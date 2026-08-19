// Rosetta Code task: Casting out nines
// Source: https://rosettacode.org/wiki/Casting_out_nines#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 1 9 10 18 19 27 28 36 37 45 46 54 55 63 64 72 73 81 82 90 91 99 
// Trying 22 numbers instead of 99 saves 77.78%
// 1 6 10 15 16 21 25 30 31 36 40 45 46 51 55 60 61 66 70 75 76 81 85 90 91 96 100 105 106 111 115 120 121 126 130 135 136 141 145 150 151 156 160 165 166 171 175 180 181 186 190 195 196 201 205 210 211 216 220 225 226 231 235 240 241 246 250 255 
// Trying 68 numbers instead of 255 saves 73.33%
// =======================

fn compare_co9_efficiency(base: u64, upto: u64) {
    let naive_candidates: Vec<u64> = (1u64..upto).collect();
    let co9_candidates: Vec<u64> = naive_candidates.iter().cloned()
        .filter(|&x| x % (base - 1) == (x * x) % (base - 1))
        .collect();
    for candidate in &co9_candidates {
        print!("{} ", candidate);
    }
    println!();
    println!(
        "Trying {} numbers instead of {} saves {:.2}%",
        co9_candidates.len(),
        naive_candidates.len(),
        100.0 - 100.0 * (co9_candidates.len() as f64 / naive_candidates.len() as f64)
    );
}

fn main() {
    compare_co9_efficiency(10, 100);
    compare_co9_efficiency(16, 256);
}
