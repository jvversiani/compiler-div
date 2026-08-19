// Rosetta Code task: Generic swap
// Source: https://rosettacode.org/wiki/Generic_swap#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// x = 12, y = 20
// x = 20, y = 12
// =======================

fn generic_swap<'a, T>(var1: &'a mut T, var2: &'a mut T) {
    std::mem::swap(var1, var2)
}

fn main(){
    let mut x: u64 = 12;
    let mut y: u64 = 20;

    println!("x = {}, y = {}", x, y);

    generic_swap(&mut x, &mut y);
    
    println!("x = {}, y = {}", x, y);
}