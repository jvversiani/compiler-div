// Rosetta Code task: Character codes
// Source: https://rosettacode.org/wiki/Character_codes#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 97
// a
// 960
// π
// =======================

use std::char::from_u32;

fn main() {
    //ascii char
    println!("{}", 'a' as u8);
    println!("{}", 97 as char);

    //unicode char
    println!("{}", 'π' as u32);
    println!("{}", from_u32(960).unwrap());
}
