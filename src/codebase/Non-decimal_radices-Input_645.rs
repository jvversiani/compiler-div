// Rosetta Code task: Non-decimal radices/Input
// Source: https://rosettacode.org/wiki/Non-decimal_radices/Input#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Parse from plain decimal: 123
// Parse with a given radix (2-36 supported): 3735928559
// =======================

fn main() {
    println!(
        "Parse from plain decimal: {}",
        "123".parse::<u32>().unwrap()
    );

    println!(
        "Parse with a given radix (2-36 supported): {}",
        u32::from_str_radix("deadbeef", 16).unwrap()
    );
}
