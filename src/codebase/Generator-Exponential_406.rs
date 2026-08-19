// Rosetta Code task: Generator/Exponential
// Source: https://rosettacode.org/wiki/Generator/Exponential#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 529 576 625 676 784 841 900 961 1024 1089
// =======================

use std::cmp::Ordering;
use std::iter::Peekable;

fn powers(m: u32) -> impl Iterator<Item = u64> {
    (0u64..).map(move |x| x.pow(m))
}

fn noncubic_squares() -> impl Iterator<Item = u64> {
    NoncubicSquares {
        squares: powers(2).peekable(),
        cubes: powers(3).peekable(),
    }
}

struct NoncubicSquares<T: Iterator<Item = u64>, U: Iterator<Item = u64>> {
    squares: Peekable<T>,
    cubes: Peekable<U>,
}

impl<T: Iterator<Item = u64>, U: Iterator<Item = u64>> Iterator for NoncubicSquares<T, U> {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        loop {
            match self.squares.peek()?.cmp(self.cubes.peek()?) {
                Ordering::Equal => self.squares.next(),
                Ordering::Greater => self.cubes.next(),
                Ordering::Less => return self.squares.next(),
            };
        }
    }
}

fn main() {
    noncubic_squares()
        .skip(20)
        .take(10)
        .for_each(|x| print!("{} ", x));
    println!();
}
