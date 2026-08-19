// Rosetta Code task: Short-circuit evaluation
// Source: https://rosettacode.org/wiki/Short-circuit_evaluation#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// a
// b
// true and true == true
// a
// true or true == true
//
// a
// b
// true and false == false
// a
// true or false == true
//
// a
// false and true == false
// a
// b
// false or true == true
//
// a
// false and false == false
// a
// b
// false or false == false
// =======================

fn a(foo: bool) -> bool {
    println!("a");
    foo
}

fn b(foo: bool) -> bool {
    println!("b");
    foo
}

fn main() {
    for i in vec![true, false] {
        for j in vec![true, false] {
            println!("{} and {} == {}", i, j, a(i) && b(j));
            println!("{} or {} == {}", i, j, a(i) || b(j));
            println!();
        }
    }
}
