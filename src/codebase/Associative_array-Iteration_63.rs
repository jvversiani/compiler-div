// Rosetta Code task: Associative array/Iteration
// Source: https://rosettacode.org/wiki/Associative_array/Iteration#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Germany has had 252 gold medals, 260 silver medals, and 270 bronze medals
// Great Britain has had 246 gold medals, 276 silver medals, and 284 bronze medals
// Soviet Union has had 473 gold medals, 376 silver medals, and 355 bronze medals
// United States has had 1072 gold medals, 859 silver medals, and 749 bronze medals
// =======================

use std::collections::BTreeMap;
fn main() {
    let mut olympic_medals = BTreeMap::new();
    olympic_medals.insert("United States", (1072, 859, 749));
    olympic_medals.insert("Soviet Union", (473, 376, 355));
    olympic_medals.insert("Great Britain", (246, 276, 284));
    olympic_medals.insert("Germany", (252, 260, 270));
    for (country, medals) in olympic_medals {
        println!("{} has had {} gold medals, {} silver medals, and {} bronze medals", 
               country, medals.0, medals.1, medals.2);
            
    }
}
