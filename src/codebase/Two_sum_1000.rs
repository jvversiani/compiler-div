// Rosetta Code task: Two sum
// Source: https://rosettacode.org/wiki/Two_sum#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Some((1, 3))
// =======================

use std::cmp::Ordering;
use std::ops::Add;

fn two_sum<T>(arr: &[T], sum: T) -> Option<(usize, usize)>
where
    T: Add<Output = T> + Ord + Copy,
{
    if arr.len() == 0 {
        return None;
    }

    let mut i = 0;
    let mut j = arr.len() - 1;

    while i < j {
        match (arr[i] + arr[j]).cmp(&sum) {
            Ordering::Equal => return Some((i, j)),
            Ordering::Less => i += 1,
            Ordering::Greater => j -= 1,
        }
    }

    None
}

fn main() {
    let arr = [0, 2, 11, 19, 90];
    let sum = 21;

    println!("{:?}", two_sum(&arr, sum));
}
