// Rosetta Code task: Function definition
// Source: https://rosettacode.org/wiki/Function_definition#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 1123 x 4324 = 4855852
// =======================

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn main(){
    println!("1123 x 4324 = {}", multiply(1123, 4324));
}