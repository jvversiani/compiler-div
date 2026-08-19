// Rosetta Code task: Map range
// Source: https://rosettacode.org/wiki/Map_range#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// [-1.0, -0.9, -0.8, -0.7, -0.6, -0.5, -0.4, -0.30000000000000004, -0.19999999999999996, -0.09999999999999998, 0.0]
// =======================

use std::ops::{Add, Sub, Mul, Div};

fn map_range<T: Copy>(from_range: (T, T), to_range: (T, T), s: T) -> T 
    where T: Add<T, Output=T> +
             Sub<T, Output=T> +
             Mul<T, Output=T> +
             Div<T, Output=T>
{
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

fn main() {
    let input: Vec<f64> = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let result = input.into_iter()
        .map(|x| map_range((0.0, 10.0), (-1.0, 0.0), x))
        .collect::<Vec<f64>>();
    print!("{:?}", result);
}
