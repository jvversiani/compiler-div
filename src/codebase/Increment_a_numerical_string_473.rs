// Rosetta Code task: Increment a numerical string
// Source: https://rosettacode.org/wiki/Increment_a_numerical_string#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// "0"
// =======================

fn next_string(input: &str) -> String {
    (input.parse::<i64>().unwrap() + 1).to_string()
}

fn main() {
    let s = "-1";
    let s2 = next_string(s);
    println!("{:?}", s2);
}
