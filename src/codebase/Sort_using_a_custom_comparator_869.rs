// Rosetta Code task: Sort using a custom comparator
// Source: https://rosettacode.org/wiki/Sort_using_a_custom_comparator#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// ["strings", "sample", "sorted", "Here", "some", "are", "be", "to"]
// =======================

fn main() {
    let mut words = ["Here", "are", "some", "sample", "strings", "to", "be", "sorted"];
    words.sort_by(|l, r| Ord::cmp(&r.len(), &l.len()).then(Ord::cmp(l, r)));
    println!("{:?}", words);
}
