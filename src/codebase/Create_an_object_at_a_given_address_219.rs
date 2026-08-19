// Rosetta Code task: Create an object at a given address
// Source: https://rosettacode.org/wiki/Create_an_object_at_a_given_address#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 5
// 6
// =======================

use std::{mem,ptr};

fn main() {
    let mut data: i32;

    // Rust does not allow us to use uninitialized memory but the STL provides an `unsafe`
    // function to override this protection.
    unsafe {data = mem::uninitialized()}

    // Construct a raw pointer (perfectly safe)
    let address = &mut data as *mut _;

    // Note: the object's address is printed via {:p} on Rosetta Code, but a
    // real address is non-reproducible (ASLR), so we print only the value to
    // keep the integration-test output deterministic.
    unsafe {ptr::write(address, 5)}
    println!("{}", data);

    unsafe {ptr::write(address, 6)}
    println!("{}", data);

}
