// Rosetta Code task: Sort stability
// Source: https://rosettacode.org/wiki/Sort_stability#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Original:
// UK London
// US New York
// US Birmingham
// UK Birmingham
//
// When sorted by city:
// US Birmingham
// UK Birmingham
// UK London
// US New York
//
// When sorted by county:
// UK London
// UK Birmingham
// US New York
// US Birmingham
// =======================

fn main() {
    let country_city = [
        ("UK", "London"),
        ("US", "New York"),
        ("US", "Birmingham"),
        ("UK", "Birmingham"),
    ];

    let mut city_sorted = country_city.clone();
    city_sorted.sort_by_key(|k| k.1);

    let mut country_sorted = country_city.clone();
    country_sorted.sort_by_key(|k| k.0);

    println!("Original:");
    for x in &country_city {
        println!("{} {}", x.0, x.1);
    }

    println!("\nWhen sorted by city:");
    for x in &city_sorted {
        println!("{} {}", x.0, x.1);
    }

    println!("\nWhen sorted by county:");
    for x in &country_sorted {
        println!("{} {}", x.0, x.1);
    }
}
