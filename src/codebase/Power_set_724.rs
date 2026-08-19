// Rosetta Code task: Power set
// Source: https://rosettacode.org/wiki/Power_set#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// {{}, {1}, {1, 2}, {1, 2, 3}, {1, 2, 3, 4}, {1, 2, 4}, {1, 3}, {1, 3, 4}, {1, 4}, {2}, {2, 3}, {2, 3, 4}, {2, 4}, {3}, {3, 4}, {4}}
// {{}, {"a"}, {"a", "b"}, {"a", "b", "c"}, {"a", "b", "c", "d"}, {"a", "b", "d"}, {"a", "c"}, {"a", "c", "d"}, {"a", "d"}, {"b"}, {"b", "c"}, {"b", "c", "d"}, {"b", "d"}, {"c"}, {"c", "d"}, {"d"}}
// =======================

use std::collections::BTreeSet;

fn powerset<T: Ord + Clone>(mut set: BTreeSet<T>) -> BTreeSet<BTreeSet<T>> {
    if set.is_empty() {
        let mut powerset = BTreeSet::new();
        powerset.insert(set);
        return powerset;
    }
    // Access the first value. This could be replaced with `set.pop_first().unwrap()`
    // But this is an unstable feature 
    let entry = set.iter().nth(0).unwrap().clone(); 
    set.remove(&entry);
    let mut powerset = powerset(set);
    for mut set in powerset.clone().into_iter() {
        set.insert(entry.clone());
        powerset.insert(set);
    }
    powerset
}

fn main() {
    let set = (1..5).collect();
    let set = powerset(set);
    println!("{:?}", set);

    let set = ["a", "b", "c", "d"].iter().collect();
    let set = powerset(set);
    println!("{:?}", set);
}
