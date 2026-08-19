// Rosetta Code task: Man or boy test
// Source: https://rosettacode.org/wiki/Man_or_boy_test#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// -67
// =======================

use std::cell::Cell;

trait Arg {
    fn run(&self) -> i32;
}

impl Arg for i32 {
    fn run(&self) -> i32 {
        *self
    }
}

struct B<'a> {
    k: &'a Cell<i32>,
    x1: &'a dyn Arg,
    x2: &'a dyn Arg,
    x3: &'a dyn Arg,
    x4: &'a dyn Arg,
}

impl<'a> Arg for B<'a> {
    fn run(&self) -> i32 {
        self.k.set(self.k.get() - 1);
        a(self.k.get(), self, self.x1, self.x2, self.x3, self.x4)
    }
}

fn a(k: i32, x1: &dyn Arg, x2: &dyn Arg, x3: &dyn Arg, x4: &dyn Arg, x5: &dyn Arg) -> i32 {
    if k <= 0 {
        x4.run() + x5.run()
    } else {
        B {
            k: &Cell::new(k),
            x1,
            x2,
            x3,
            x4,
        }
        .run()
    }
}

pub fn main() {
    println!("{}", a(10, &1, &-1, &-1, &1, &0));
}
