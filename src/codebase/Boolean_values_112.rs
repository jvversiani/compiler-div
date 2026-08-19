// Rosetta Code task: Boolean values
// Source: https://rosettacode.org/wiki/Boolean_values#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// foo is true.
// bar is false.
// =======================

fn main() {
    // Rust contains a single boolean type: `bool`, represented by the keywords `true` and `false`.
    // Expressions inside `if` and `while` statements must result in type `bool`. There is no
    // automatic conversion to the boolean type.

    let true_value = true;
    if true_value {
        println!("foo is {}.", true_value);
    }

    let false_value = false;
    if !false_value {
        println!("bar is {}.", false_value);
    }
}
