// Rosetta Code task: Compound data type
// Source: https://rosettacode.org/wiki/Compound_data_type#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 1, 2.5
// =======================

 // Defines a generic struct where x and y can be of any type T
struct Point<T> {
    x: T,
    y: T,
}
fn main() {
    let p = Point { x: 1.0, y: 2.5 }; // p is of type Point<f64>
    println!("{}, {}", p.x, p.y);
}
