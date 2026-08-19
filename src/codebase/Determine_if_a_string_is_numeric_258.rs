// Rosetta Code task: Determine if a string is numeric
// Source: https://rosettacode.org/wiki/Determine_if_a_string_is_numeric#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// '142857' can be parsed as i32
// '142857' can be parsed as i64
// '142857' can be parsed as i128
// '142857' can be parsed as f64
// '3.14' _cannot_ be parsed as i32
// '3.14' _cannot_ be parsed as i64
// '3.14' _cannot_ be parsed as i128
// '3.14' can be parsed as f64
// 'not of this earth!' _cannot_ be parsed as i32
// 'not of this earth!' _cannot_ be parsed as i64
// 'not of this earth!' _cannot_ be parsed as i128
// 'not of this earth!' _cannot_ be parsed as f64
// =======================

use std::str::FromStr;

fn parsable<T: FromStr>(s: &str) -> bool {
    s.parse::<T>().is_ok()
}

fn main() {
    let test_cases = [
        "142857",
        "3.14",
        "not of this earth!"
    ];

    let types: &[(&str, fn(&str) -> bool)] = &[
        ("i32", parsable::<i32> as fn(&str) -> bool),
        ("i64", parsable::<i32> as fn(&str) -> bool),
        ("i128", parsable::<i32> as fn(&str) -> bool),
        
        ("f64", parsable::<f64> as fn(&str) -> bool),
    ];

    for &case in &test_cases {
        for &(type_name, parse_fn) in types {
            println!(
                "'{}' {} be parsed as {}",
                case,
                if parse_fn(case) { "can" } else { "_cannot_" },
                type_name
            );
        }
    }
}