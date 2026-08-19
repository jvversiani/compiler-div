// Rosetta Code task: Accumulator factory
// Source: https://rosettacode.org/wiki/Accumulator_factory#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 8.3
// =======================

// rustc 1.26.0 or later

use std::ops::Add;

fn foo<Num>(n: Num) -> impl FnMut(Num) -> Num
        where Num: Add<Output=Num> + Copy + 'static {
    let mut acc = n;
    move |i: Num| {
        acc = acc + i;
        acc
    }
}

fn main() {
    let mut x = foo(1.);
    x(5.);
    foo(3.);
    println!("{}", x(2.3));
}
