// Rosetta Code task: Queue/Definition
// Source: https://rosettacode.org/wiki/Queue/Definition#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// all good!
// =======================

use std::collections::VecDeque;
fn main() {
    let mut stack = VecDeque::new();
    stack.push_back("Element1");
    stack.push_back("Element2");
    stack.push_back("Element3");

    assert_eq!(Some(&"Element1"), stack.front());
    assert_eq!(Some("Element1"), stack.pop_front());
    assert_eq!(Some("Element2"), stack.pop_front());
    assert_eq!(Some("Element3"), stack.pop_front());
    assert_eq!(None, stack.pop_front());

    println!("all good!");
}
