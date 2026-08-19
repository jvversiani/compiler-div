// Rosetta Code task: Currying
// Source: https://rosettacode.org/wiki/Currying#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// The answer to life is 42.
// =======================

fn add_n(n : i32) -> impl Fn(i32) -> i32 {
    move |x| n + x
}

fn main() {
    let adder = add_n(40);
    println!("The answer to life is {}.", adder(2));
}
