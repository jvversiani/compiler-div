// Rosetta Code task: Case-sensitivity of identifiers
// Source: https://rosettacode.org/wiki/Case-sensitivity_of_identifiers#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// The three dogs are named Benjamin, Samba and Bernie.
// =======================

fn main() {
    let dog = "Benjamin";
    let Dog = "Samba";
    let DOG = "Bernie";
    println!("The three dogs are named {}, {} and {}.", dog, Dog, DOG);
}
