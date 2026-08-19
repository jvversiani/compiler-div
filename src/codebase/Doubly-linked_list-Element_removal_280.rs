// Rosetta Code task: Doubly-linked list/Element removal
// Source: https://rosettacode.org/wiki/Doubly-linked_list/Element_removal#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Before removals: ["dog", "cat", "bear"]
// After removal 1: ["dog", "bear"]
// After removal 2: ["bear"]
// =======================

use std::collections::LinkedList;

fn main() {    
    let mut dll = LinkedList::from(["dog", "cat", "bear"]);
    println!("Before removals: {dll:?}");
    {
        let mut right = dll.split_off(2);
        dll.pop_back();
        dll.append(&mut right);
    }
    println!("After removal 1: {dll:?}");
    dll.pop_front();
    println!("After removal 2: {dll:?}");
}
