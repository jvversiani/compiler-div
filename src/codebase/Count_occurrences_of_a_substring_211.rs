// Rosetta Code task: Count occurrences of a substring
// Source: https://rosettacode.org/wiki/Count_occurrences_of_a_substring#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 3
// 2
// =======================

fn main() {
    println!("{}","the three truths".matches("th").count());
    println!("{}","ababababab".matches("abab").count());
}
