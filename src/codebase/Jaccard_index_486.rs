// Rosetta Code task: Jaccard index
// Source: https://rosettacode.org/wiki/Jaccard_index#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
//      Set A              Set B         J(A, B)
// ---------------------------------------------
// []                  []                  1.00000
// []                  [1, 2, 3, 4, 5]     0.00000
// []                  [1, 3, 5, 7, 9]     0.00000
// []                  [2, 4, 6, 8, 10]    0.00000
// []                  [2, 3, 5, 7]        0.00000
// []                  [8]                 0.00000
// [1, 2, 3, 4, 5]     []                  0.00000
// [1, 2, 3, 4, 5]     [1, 2, 3, 4, 5]     1.00000
// [1, 2, 3, 4, 5]     [1, 3, 5, 7, 9]     0.42857
// [1, 2, 3, 4, 5]     [2, 4, 6, 8, 10]    0.25000
// [1, 2, 3, 4, 5]     [2, 3, 5, 7]        0.50000
// [1, 2, 3, 4, 5]     [8]                 0.00000
// [1, 3, 5, 7, 9]     []                  0.00000
// [1, 3, 5, 7, 9]     [1, 2, 3, 4, 5]     0.42857
// [1, 3, 5, 7, 9]     [1, 3, 5, 7, 9]     1.00000
// [1, 3, 5, 7, 9]     [2, 4, 6, 8, 10]    0.00000
// [1, 3, 5, 7, 9]     [2, 3, 5, 7]        0.50000
// [1, 3, 5, 7, 9]     [8]                 0.00000
// [2, 4, 6, 8, 10]    []                  0.00000
// [2, 4, 6, 8, 10]    [1, 2, 3, 4, 5]     0.25000
// [2, 4, 6, 8, 10]    [1, 3, 5, 7, 9]     0.00000
// [2, 4, 6, 8, 10]    [2, 4, 6, 8, 10]    1.00000
// [2, 4, 6, 8, 10]    [2, 3, 5, 7]        0.12500
// [2, 4, 6, 8, 10]    [8]                 0.20000
// [2, 3, 5, 7]        []                  0.00000
// [2, 3, 5, 7]        [1, 2, 3, 4, 5]     0.50000
// [2, 3, 5, 7]        [1, 3, 5, 7, 9]     0.50000
// [2, 3, 5, 7]        [2, 4, 6, 8, 10]    0.12500
// [2, 3, 5, 7]        [2, 3, 5, 7]        1.00000
// [2, 3, 5, 7]        [8]                 0.00000
// [8]                 []                  0.00000
// [8]                 [1, 2, 3, 4, 5]     0.00000
// [8]                 [1, 3, 5, 7, 9]     0.00000
// [8]                 [2, 4, 6, 8, 10]    0.20000
// [8]                 [2, 3, 5, 7]        0.00000
// [8]                 [8]                 1.00000
// =======================

use std::collections::HashSet;
use std::fmt::Display;

fn vector_to_string<T: Display>(vec: &[T]) -> String {
    let mut result = String::from("[");
    for (i, item) in vec.iter().enumerate() {
        result.push_str(&item.to_string());
        if i < vec.len() - 1 {
            result.push_str(", ");
        }
    }
    result.push(']');
    result
}

fn jaccard_index(a: &[i32], b: &[i32]) -> f64 {
    let set_a: HashSet<i32> = a.iter().cloned().collect();
    
    let intersection_count = b.iter()
        .filter(|&element| set_a.contains(element))
        .count();
    
    let mut union_set = set_a.clone();
    union_set.extend(b.iter().cloned());
    let union_count = union_set.len();
    
    if union_count == 0 {
        1.0
    } else if intersection_count == 0 {
        0.0
    } else {
        intersection_count as f64 / union_count as f64
    }
}

fn main() {
    let tests: Vec<Vec<i32>> = vec![
        vec![],
        vec![1, 2, 3, 4, 5],
        vec![1, 3, 5, 7, 9],
        vec![2, 4, 6, 8, 10],
        vec![2, 3, 5, 7],
        vec![8],
    ];

    println!("     Set A              Set B         J(A, B)");
    println!("---------------------------------------------");
    
    for a in &tests {
        for b in &tests {
            println!("{:<19} {:<19} {:.5}", 
                vector_to_string(a), 
                vector_to_string(b), 
                jaccard_index(a, b)
            );
        }
    }
}
