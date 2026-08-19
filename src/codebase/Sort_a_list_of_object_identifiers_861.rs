// Rosetta Code task: Sort a list of object identifiers
// Source: https://rosettacode.org/wiki/Sort_a_list_of_object_identifiers#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// [
//     "1.3.6.1.4.1.11.2.17.5.2.0.79",
//     "1.3.6.1.4.1.11.2.17.19.3.4.0.1",
//     "1.3.6.1.4.1.11.2.17.19.3.4.0.4",
//     "1.3.6.1.4.1.11.2.17.19.3.4.0.10",
//     "1.3.6.1.4.1.11150.3.4.0",
//     "1.3.6.1.4.1.11150.3.4.0.1",
// ]
// =======================

fn split(s: &str) -> impl Iterator<Item = u64> + '_ {
    s.split('.').map(|x| x.parse().unwrap())
}

fn main() {
    let mut oids = vec![
        "1.3.6.1.4.1.11.2.17.19.3.4.0.10",
        "1.3.6.1.4.1.11.2.17.5.2.0.79",
        "1.3.6.1.4.1.11.2.17.19.3.4.0.4",
        "1.3.6.1.4.1.11150.3.4.0.1",
        "1.3.6.1.4.1.11.2.17.19.3.4.0.1",
        "1.3.6.1.4.1.11150.3.4.0",
    ];

    oids.sort_by(|a, b| Iterator::cmp(split(a), split(b)));
    
    println!("{:#?}", oids);
}
