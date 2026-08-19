// Rosetta Code task: String interpolation (included)
// Source: https://rosettacode.org/wiki/String_interpolation_(included)#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Mary had a little lamb
// Mary had a little lamb
// Mary had a little lamb
// =======================

fn main() {
    println!("Mary had a {} lamb", "little");
    // You can specify order
    println!("{1} had a {0} lamb", "little", "Mary");
    // Or named arguments if you prefer
    println!("{name} had a {adj} lamb", adj="little", name="Mary");
}
