// Rosetta Code task: List comprehensions
// Source: https://rosettacode.org/wiki/List_comprehensions#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// [[3, 4, 5], [5, 12, 13], [6, 8, 10], [8, 15, 17], [9, 12, 15], [12, 16, 20]]
// =======================

fn pyth(n: u32) -> impl Iterator<Item = [u32; 3]> {
    (1..=n).flat_map(move |x| {
        (x..=n).flat_map(move |y| {
            (y..=n).filter_map(move |z| {
                if x.pow(2) + y.pow(2) == z.pow(2) {
                    Some([x, y, z])
                } else {
                    None
                }
            })
        })
    })
}

fn main() {
    let triples: Vec<[u32; 3]> = pyth(20).collect();
    println!("{:?}", triples);
}