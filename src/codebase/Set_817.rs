// Rosetta Code task: Set
// Source: https://rosettacode.org/wiki/Set#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Set A: [1, 3, 4]
// Set B: [3, 5, 6]
// Does A contain 4? true
// Union: [1, 3, 4, 5, 6]
// Intersection: [3]
// Difference: [1, 4]
// Is A a subset of B? false
// Is A equal to B? false
// =======================

use std::collections::BTreeSet;

fn main() {
  let a = vec![1, 3, 4].into_iter().collect::<BTreeSet<i32>>();
  let b = vec![3, 5, 6].into_iter().collect::<BTreeSet<i32>>();

  println!("Set A: {:?}", a.iter().collect::<Vec<_>>());
  println!("Set B: {:?}", b.iter().collect::<Vec<_>>());
  println!("Does A contain 4? {}", a.contains(&4));
  println!("Union: {:?}", a.union(&b).collect::<Vec<_>>());
  println!("Intersection: {:?}", a.intersection(&b).collect::<Vec<_>>());
  println!("Difference: {:?}", a.difference(&b).collect::<Vec<_>>());
  println!("Is A a subset of B? {}", a.is_subset(&b));
  println!("Is A equal to B? {}", a == b);
}
