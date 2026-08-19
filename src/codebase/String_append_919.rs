// Rosetta Code task: String append
// Source: https://rosettacode.org/wiki/String_append#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Hello world!!!!
// =======================

use std::ops::Add;

fn main(){
    let hello = String::from("Hello world");
    println!("{}", hello.add("!!!!"));
}
