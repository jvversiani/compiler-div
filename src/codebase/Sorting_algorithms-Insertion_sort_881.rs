// Rosetta Code task: Sorting algorithms/Insertion sort
// Source: https://rosettacode.org/wiki/Sorting_algorithms/Insertion_sort#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Before: [4, 3, 2, 1, 0]
// After: [0, 1, 2, 3, 4]
// =======================

fn insertion_sort<T: std::cmp::Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j-1] {
            arr.swap(j, j-1);
            j = j-1;
        }
    }
}

fn main(){
    let mut valeurs = vec![4, 3, 2, 1, 0];
    println!("Before: {:?}", valeurs);
    insertion_sort(&mut valeurs);
    println!("After: {:?}", valeurs);
}