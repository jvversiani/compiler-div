// Rosetta Code task: Remove duplicate elements
// Source: https://rosettacode.org/wiki/Remove_duplicate_elements#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Before removal of duplicates : [0, 0, 1, 1, 2, 3, 2]
// After removal of duplicates : [0, 1, 2, 3]
// Before removal of duplicates : ["ab", "cde", "ab", "xyz", "d", "d", "ab"]
// After removal of duplicates : ["ab", "cde", "d", "xyz"]
// =======================

use std::collections::BTreeSet;

fn remove_duplicate_elements_hashing<T: Ord>(elements: &mut Vec<T>) {
    let set: BTreeSet<_> = elements.drain(..).collect();
    elements.extend(set.into_iter());
}

fn remove_duplicate_elements_sorting<T: Ord>(elements: &mut Vec<T>) {
    elements.sort_unstable(); // order does not matter
    elements.dedup();
}

fn main() {
    let mut sample_elements = vec![0, 0, 1, 1, 2, 3, 2];
    println!("Before removal of duplicates : {:?}", sample_elements);
    remove_duplicate_elements_sorting(&mut sample_elements);
    println!("After removal of duplicates : {:?}", sample_elements);

    let mut sample_elements2 = vec!["ab", "cde", "ab", "xyz", "d", "d", "ab"];
    println!("Before removal of duplicates : {:?}", sample_elements2);
    remove_duplicate_elements_hashing(&mut sample_elements2);
    println!("After removal of duplicates : {:?}", sample_elements2);
}
