// Rosetta Code task: Distinct power numbers
// Source: https://rosettacode.org/wiki/Distinct_power_numbers#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// [4, 8, 9, 16, 25, 27, 32, 64, 81, 125, 243, 256, 625, 1024, 3125]
// =======================

fn main() {
    let mut v = (2u32..=5)
        .flat_map(|a| (2u32..=5).map(move |b| a.pow(b)))
        .collect::<Vec<_>>();
    v.sort();
    v.dedup();
    println!("{v:?}");
}
