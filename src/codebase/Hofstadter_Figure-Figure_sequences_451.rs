// Rosetta Code task: Hofstadter Figure-Figure sequences
// Source: https://rosettacode.org/wiki/Hofstadter_Figure-Figure_sequences#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// H:0 -> R: 1, S: 2
// H:1 -> R: 3, S: 4
// H:2 -> R: 7, S: 5
// H:3 -> R: 12, S: 6
// H:4 -> R: 18, S: 8
// H:5 -> R: 26, S: 9
// H:6 -> R: 35, S: 10
// H:7 -> R: 45, S: 11
// H:8 -> R: 56, S: 13
// H:9 -> R: 69, S: 14
// =======================

use std::collections::HashMap;

struct Hffs {
    sequence_r: HashMap<usize, usize>,
    sequence_s: HashMap<usize, usize>,
}

impl Hffs {
    fn new() -> Hffs {
        Hffs {
            sequence_r: HashMap::new(),
            sequence_s: HashMap::new(),
        }
    }
    fn ffr(&mut self, n: usize) -> usize {
        // first try the cache
        let new_r = if let Some(result) = self.sequence_r.get(&n) {
            *result
        } else if n == 0 {
            1
        } else {
            // call recursively
            self.ffr(n - 1) + self.ffs(n - 1)
        };

        // insert into the cache and return value
        *self.sequence_r.entry(n).or_insert(new_r)
    }

    fn ffs(&mut self, n: usize) -> usize {
        // first try the cache
        let new_s = if let Some(result) = self.sequence_s.get(&n) {
            *result
        } else if n == 0 {
            2
        } else {
            let lower = self.ffs(n - 1) + 1_usize;
            let upper = self.ffr(n) + 1_usize;
            let mut min_s: usize = 0;
            // find next available S
            for i in lower..=upper {
                if !self.sequence_r.values().any(|&val| val == i) {
                    min_s = i;
                    break;
                }
            }
            min_s
        };

        // insert into the cache and return value
        *self.sequence_s.entry(n).or_insert(new_s)
    }
}

impl Default for Hffs {
    fn default() -> Self {
        Self::new()
    }
}
fn main() {
    let mut hof = Hffs::new();

    for i in 0..10 {
        println!("H:{} -> R: {}, S: {}", i, hof.ffr(i), hof.ffs(i));
    }

    let r40 = (0..40).map(|i| hof.ffr(i)).collect::<Vec<_>>();
    let mut s960 = (0..960).map(|i| hof.ffs(i)).collect::<Vec<_>>();

    s960.extend(&r40);
    s960.sort_unstable();
    let f1000 = (1_usize..=1000).collect::<Vec<_>>();

    assert_eq!(f1000, s960, "Does NOT match");
}
