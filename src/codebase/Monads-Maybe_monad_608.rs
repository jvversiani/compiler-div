// Rosetta Code task: Monads/Maybe monad
// Source: https://rosettacode.org/wiki/Monads/Maybe_monad#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// i = 00, upside down square root = Some("0")
// i = 01, upside down square root = Some("1")
// i = 02, upside down square root = None
// i = 03, upside down square root = None
// i = 04, upside down square root = None
// i = 05, upside down square root = None
// i = 06, upside down square root = None
// i = 07, upside down square root = None
// i = 08, upside down square root = None
// i = 09, upside down square root = None
// i = 10, upside down square root = None
// i = 11, upside down square root = None
// i = 12, upside down square root = None
// i = 13, upside down square root = None
// i = 14, upside down square root = None
// i = 15, upside down square root = None
// i = 16, upside down square root = None
// i = 17, upside down square root = None
// i = 18, upside down square root = None
// i = 19, upside down square root = None
// i = 20, upside down square root = None
// i = 21, upside down square root = None
// i = 22, upside down square root = None
// i = 23, upside down square root = None
// i = 24, upside down square root = None
// i = 25, upside down square root = None
// i = 26, upside down square root = None
// i = 27, upside down square root = None
// i = 28, upside down square root = None
// i = 29, upside down square root = None
// i = 30, upside down square root = None
// i = 31, upside down square root = None
// i = 32, upside down square root = None
// i = 33, upside down square root = None
// i = 34, upside down square root = None
// i = 35, upside down square root = None
// i = 36, upside down square root = Some("9")
// =======================

use std::collections::HashMap;

/// Returns the arithmetic square root of x, if it exists
fn arithmetic_square_root(x: u8) -> Option<u8> {
    // the number of perfect squares for u8 is so low you can just fit the entire list in memory
    let perfect_squares: HashMap<u8, u8> = HashMap::from([
        (0, 0),
        (1, 1),
        (4, 2),
        (9, 3),
        (16, 4),
        (25, 5),
        (36, 6),
        (49, 7),
        (64, 8),
        (81, 9),
        (100, 10),
        (121, 11),
        (144, 12),
        (169, 13),
        (196, 14),
        (225, 15),
    ]);

    // `HashMap::<K, V>::get(&self, &Q)` also returns a `Option<&V>`, we then turn it to `Option<V>`
    perfect_squares.get(&x).copied()
}


/// If x in base 10 is also a valid number when looking upside down, return a string slice for that
/// number upside down
fn upside_down_num(x: u8) -> Option<&'static str> {
    match x {
        0 => Some("0"),
        1 => Some("1"),
        6 => Some("9"),
        8 => Some("8"),
        9 => Some("6"),
        10 => Some("01"),
        11 => Some("11"),
        16 => Some("91"),
        _ => None
    }
}

fn main() {
    // if the number from 0 to 36 inclusive, is a perfect square and its square root is also a
    // valid number when looking upside down, then we will get a Some containing the string slice,
    // otherwise we get a None, indicating it's not a perfect square or the square root is not a
    // valid number while looking upside down
    (0..=36)
        .map(|x| arithmetic_square_root(x).and_then(upside_down_num))
        .enumerate()
        .for_each(|(i, upside_down_square_root)|
            println!("i = {i:02}, upside down square root = {upside_down_square_root:?}"));
}
