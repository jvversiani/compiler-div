// Rosetta Code task: Zig-zag matrix
// Source: https://rosettacode.org/wiki/Zig-zag_matrix#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// [[0, 1, 5, 6, 14], [2, 4, 7, 13, 15], [3, 8, 12, 16, 21], [9, 11, 17, 20, 22], [10, 18, 19, 23, 24]]
// =======================

use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::iter::repeat;

#[derive(Debug, PartialEq, Eq)]
struct SortIndex {
    x: usize,
    y: usize,
}

impl SortIndex {
    fn new(x: usize, y: usize) -> SortIndex {
        SortIndex { x, y }
    }
}

impl PartialOrd for SortIndex {
    fn partial_cmp(&self, other: &SortIndex) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SortIndex {
    fn cmp(&self, other: &SortIndex) -> Ordering {
        let lower = if self.x + self.y == other.x + other.y {
            if (self.x + self.y) % 2 == 0 {
                self.x < other.x
            } else {
                self.y < other.y
            }
        } else {
            (self.x + self.y) < (other.x + other.y)
        };

        if lower {
            Less
        } else if self == other {
            Equal
        } else {
            Greater
        }
    }
}

fn zigzag(n: usize) -> Vec<Vec<usize>> {
    let mut l: Vec<SortIndex> = (0..n * n).map(|i| SortIndex::new(i % n, i / n)).collect();
    l.sort();

    let init_vec = vec![0; n];
    let mut result: Vec<Vec<usize>> = repeat(init_vec).take(n).collect();
    for (i, &SortIndex { x, y }) in l.iter().enumerate() {
        result[y][x] = i
    }
    result
}

fn main() {
    println!("{:?}", zigzag(5));
}
