// Rosetta Code task: Doubly-linked list/Traversal
// Source: https://rosettacode.org/wiki/Doubly-linked_list/Traversal#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 1 5 7 0 3 2
// 2 3 0 7 5 1
// =======================

use std::collections::LinkedList;

fn main() {    
    let numbers = LinkedList::from([1, 5, 7, 0, 3, 2]);
    
    for i in &numbers {
        print!("{i} ");
    }
    println!();

    for i in numbers.iter().rev() {
        print!("{i} ");
    }
    println!();
}
