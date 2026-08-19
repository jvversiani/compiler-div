// Rosetta Code task: Associative array/Merging
// Source: https://rosettacode.org/wiki/Associative_array/Merging#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// {
//     "color": "red",
//     "name": "Rocket Skates",
//     "price": "15.25",
//     "year": "1974",
// }
// =======================

use std::collections::BTreeMap;

fn main() {
    let mut original = BTreeMap::new();
    original.insert("name", "Rocket Skates");
    original.insert("price", "12.75");
    original.insert("color", "yellow");

    let mut update = BTreeMap::new();
    update.insert("price", "15.25");
    update.insert("color", "red");
    update.insert("year", "1974");

    original.extend(&update);

    println!("{:#?}", original);
}
