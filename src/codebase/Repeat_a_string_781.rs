// Rosetta Code task: Repeat a string
// Source: https://rosettacode.org/wiki/Repeat_a_string#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// hahahahaha
// =======================

fn main(){
    println!("{}", std::iter::repeat("ha").take(5).collect::<String>());
}
