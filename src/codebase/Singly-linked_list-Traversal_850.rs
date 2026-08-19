// Rosetta Code task: Singly-linked list/Traversal
// Source: https://rosettacode.org/wiki/Singly-linked_list/Traversal#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// iter (by &T):    3 2 1 
// after iter_mut:  6 4 2 
// into_iter (by T): 6 4 2
// =======================

// 
//
// Iteration by value (simply empties the list as the caller now owns all values)
//
//
pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.head.take().map(|node| { 
            let node = *node;
            self.0.head = node.next;
            node.elem
        })
    }
}

//
//
// Iteration by immutable reference
//
//

pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

//
//
// Iteration by mutable reference
//
//

pub struct IterMut<'a, T: 'a> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
}

type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    // Push onto the front of the list.
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter<'a>(&'a self) -> Iter<'a,T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_mut().map(|node| &mut **node) }
    }
}

fn main() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3); // list is now 3 -> 2 -> 1 (push is front-insert)

    // Iterate by immutable reference.
    print!("iter (by &T):    ");
    for x in list.iter() {
        print!("{} ", x);
    }
    println!();

    // Iterate by mutable reference, doubling each element in place.
    for x in list.iter_mut() {
        *x *= 2;
    }

    print!("after iter_mut:  ");
    for x in list.iter() {
        print!("{} ", x);
    }
    println!();

    // Iterate by value, consuming the list.
    print!("into_iter (by T):");
    for x in list.into_iter() {
        print!(" {}", x);
    }
    println!();
    // `list` is now moved/empty; can't be used after into_iter.
}
