// Rosetta Code task: Variadic function
// Source: https://rosettacode.org/wiki/Variadic_function#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Rosetta
// Code
// Is
// Awesome!
// =======================

// 20220106 Rust programming solution

macro_rules! print_all {
   ($($args:expr),*) => { $( println!("{}", $args); )* } 
}

fn main() {
   print_all!("Rosetta", "Code", "Is", "Awesome!");
}
