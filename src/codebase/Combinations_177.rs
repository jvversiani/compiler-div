// Rosetta Code task: Combinations
// Source: https://rosettacode.org/wiki/Combinations#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 1 2 3 
// 1 2 4 
// 1 2 5 
// 1 3 4 
// 1 3 5 
// 1 4 5 
// 2 3 4 
// 2 3 5 
// 2 4 5 
// 3 4 5 
// A B C 
// A B D 
// A B E 
// A C D 
// A C E 
// A D E 
// B C D 
// B C E 
// B D E 
// C D E
// =======================

use std::fmt::Display;

fn comb<T: Display>(arr: &[T], n: usize) {
    let mut incl_arr: Vec<bool> = vec![false; arr.len()];
    comb_intern(arr, n, &mut incl_arr, 0);
}

fn comb_intern<T: Display>(arr: &[T], n: usize, incl_arr: &mut [bool], index: usize) {
    if arr.len() < n + index {
        return;
    }
    if n == 0 {
        let it = arr.iter().zip(incl_arr.iter()).filter_map(|(val, incl)| {
            if *incl { Some(val) } else { None }
        });
        for val in it {
            print!("{} ", *val);
        }
        println!();
        return;
    }

    incl_arr[index] = true;
    comb_intern(arr, n - 1, incl_arr, index + 1);
    incl_arr[index] = false;

    comb_intern(arr, n, incl_arr, index + 1);
}

fn main() {
    let arr1 = [1, 2, 3, 4, 5];
    comb(&arr1, 3);

    let arr2 = ["A", "B", "C", "D", "E"];
    comb(&arr2, 3);
}