// Rosetta Code task: JortSort
// Source: https://rosettacode.org/wiki/JortSort#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Was the array originally sorted? no
// Was the array originally sorted? yes
// =======================

use std::cmp::{Ord, Eq};

fn jort_sort<T: Ord + Eq + Clone>(array: Vec<T>) -> bool {
    // sort the array
    let mut sorted_array = array.to_vec();
    sorted_array.sort();
    
    // compare to see if it was originally sorted
    for i in 0..array.len() {
        if array[i] != sorted_array[i] {
            return false;
        }
    }

    return true;
}

fn main() {
    let values = vec![5, 4, 3, 2, 1, 0];
    println!(
        "Was the array originally sorted? {}",
        if jort_sort(values.clone()) { "yes" } else { "no" }
    );

    let values2 = vec![0, 1, 2, 3, 4, 5];
    println!(
        "Was the array originally sorted? {}",
        if jort_sort(values2.clone()) { "yes" } else { "no" }
    );
}