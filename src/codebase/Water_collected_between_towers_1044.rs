// Rosetta Code task: Water collected between towers
// Source: https://rosettacode.org/wiki/Water_collected_between_towers#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// pattern: [1, 5, 3, 7, 2], fill: 2
// pattern: [5, 3, 7, 2, 6, 4, 5, 9, 1, 2], fill: 14
// pattern: [2, 6, 3, 5, 2, 8, 1, 4, 2, 2, 5, 3, 5, 7, 4, 1], fill: 35
// pattern: [5, 5, 5, 5], fill: 0
// pattern: [5, 6, 7, 8], fill: 0
// pattern: [8, 7, 7, 6], fill: 0
// pattern: [6, 7, 10, 7, 6], fill: 0
// =======================

use std::cmp::min;

fn getfill(pattern: &[usize]) -> usize {
    let mut total = 0;
    for (idx, val) in pattern.iter().enumerate() {
        let l_peak = pattern[..idx].iter().max();
        let r_peak = pattern[idx + 1..].iter().max();
        if l_peak.is_some() && r_peak.is_some() {
            let peak = min(l_peak.unwrap(), r_peak.unwrap());
            if peak > val {
                total += peak - val;
            }
        }
    }
    total
}

fn main() {
    let patterns = vec![
        vec![1, 5, 3, 7, 2],
        vec![5, 3, 7, 2, 6, 4, 5, 9, 1, 2],
        vec![2, 6, 3, 5, 2, 8, 1, 4, 2, 2, 5, 3, 5, 7, 4, 1],
        vec![5, 5, 5, 5],
        vec![5, 6, 7, 8],
        vec![8, 7, 7, 6],
        vec![6, 7, 10, 7, 6],
    ];

    for pattern in patterns {
        println!("pattern: {:?}, fill: {}", &pattern, getfill(&pattern));
    }
}
