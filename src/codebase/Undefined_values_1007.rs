// Rosetta Code task: Undefined values
// Source: https://rosettacode.org/wiki/Undefined_values#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// All good!
// =======================

use std::ptr;

fn main(){
    let p: *const i32 = ptr::null();
    assert!(p.is_null());
    println!("All good!");
}
