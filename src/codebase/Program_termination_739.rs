// Rosetta Code task: Program termination
// Source: https://rosettacode.org/wiki/Program_termination#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// The program is running
// =======================

fn main() {
    println!("The program is running");
    return;
    println!("This line won't be printed");
}
