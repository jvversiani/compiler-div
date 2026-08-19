// Rosetta Code task: Associative array/Creation
// Source: https://rosettacode.org/wiki/Associative_array/Creation#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// {"Germany": (252, 260, 270), "Great Britain": (246, 276, 284), "Soviet Union": (473, 376, 355), "United States": (1072, 859, 749)}
// =======================

use std::collections::BTreeMap;
fn main() {
    let mut olympic_medals = BTreeMap::new();
    olympic_medals.insert("United States", (1072, 859, 749));
    olympic_medals.insert("Soviet Union", (473, 376, 355));
    olympic_medals.insert("Great Britain", (246, 276, 284));
    olympic_medals.insert("Germany", (252, 260, 270));
    println!("{:?}", olympic_medals);
}
