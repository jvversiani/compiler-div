// Rosetta Code task: Metaprogramming
// Source: https://rosettacode.org/wiki/Metaprogramming#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// add_assign: [3, 4, 5, 6]
// mul_assign: [2, 4, 6, 8]
// sub_assign: [3, 3, 3, 3]
// =======================

// dry.rs
use std::ops::{Add, Mul, Sub};

macro_rules! assert_equal_len {
    ($a:ident, $b:ident, $func:ident, $op:tt) => (
        assert!($a.len() == $b.len(),
                "{:?}: dimension mismatch: {:?} {:?} {:?}",
                stringify!($func),
                ($a.len(),),
                stringify!($op),
                ($b.len(),));
    )
}

macro_rules! op {
    ($func:ident, $bound:ident, $op:tt, $method:ident) => (
        fn $func<T: $bound<T, Output=T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
            assert_equal_len!(xs, ys, $func, $op);

            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                *x = $bound::$method(*x, *y);
            }
        }
    )
}

// Generate add_assign, mul_assign, and sub_assign.
op!(add_assign, Add, +=, add);
op!(mul_assign, Mul, *=, mul);
op!(sub_assign, Sub, -=, sub);

fn main() {
    let mut a = vec![1u32, 2, 3, 4];
    let b = vec![2u32, 2, 2, 2];
    add_assign(&mut a, &b);
    println!("add_assign: {:?}", a); // [3, 4, 5, 6]

    let mut c = vec![1u32, 2, 3, 4];
    mul_assign(&mut c, &b);
    println!("mul_assign: {:?}", c); // [2, 4, 6, 8]

    let mut d = vec![5u32, 5, 5, 5];
    sub_assign(&mut d, &b);
    println!("sub_assign: {:?}", d); // [3, 3, 3, 3]
}
