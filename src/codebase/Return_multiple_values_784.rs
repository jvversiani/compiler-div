// Rosetta Code task: Return multiple values
// Source: https://rosettacode.org/wiki/Return_multiple_values#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Hello,42
// =======================

fn multi_hello() -> (&'static str, i32) {
    ("Hello",42)
}

fn main() {
    let (str,num)=multi_hello();
    println!("{},{}",str,num);
}
