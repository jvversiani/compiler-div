// Rosetta Code task: Loops/For
// Source: https://rosettacode.org/wiki/Loops/For#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// *
// **
// ***
// ****
// *****
// =======================

fn main() {
    for i in 0..5 {
        for _ in 0..=i {
            print!("*");
        }

        println!();
    }
}
