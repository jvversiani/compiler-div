// Rosetta Code task: Sorting algorithms/Shell sort
// Source: https://rosettacode.org/wiki/Sorting_algorithms/Shell_sort#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Before: [4, 65, 2, -31, 0, 99, 2, 83, 782, 1]
// After: [-31, 0, 1, 2, 2, 4, 65, 83, 99, 782]
// =======================

fn shell_sort<T: Ord + Copy>(v: &mut [T]) {
    let mut gap = v.len() / 2;
    let len = v.len();
    while gap > 0 {
        for i in gap..len {
            let temp = v[i];
            let mut j = i;
            while j >= gap && v[j - gap] > temp {
                v[j] = v[j - gap];
                j -= gap;
            }
            v[j] = temp;
        }
        gap /= 2;
    }
}

fn main() {
    let mut numbers = [4i32, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    println!("Before: {:?}", numbers);
    shell_sort(&mut numbers);
    println!("After: {:?}", numbers);
}
