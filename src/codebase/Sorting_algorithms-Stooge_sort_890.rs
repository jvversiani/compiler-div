// Rosetta Code task: Sorting algorithms/Stooge sort
// Source: https://rosettacode.org/wiki/Sorting_algorithms/Stooge_sort#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Before: [1, 9, 4, 7, 6, 5, 3, 2, 8]
// After: [1, 2, 3, 4, 5, 6, 7, 8, 9]
// =======================

fn stoogesort<E>(a: &mut [E])
    where E: PartialOrd
{
    let len = a.len();

    if a.first().unwrap() > a.last().unwrap() {
        a.swap(0, len - 1);
    }
    if len - 1 > 1 {
        let t = len / 3;
        stoogesort(&mut a[..len - 1]);
        stoogesort(&mut a[t..]);
        stoogesort(&mut a[..len - 1]);
    }
}

fn main() {
    let mut numbers = vec![1_i32, 9, 4, 7, 6, 5, 3, 2, 8];
    println!("Before: {:?}", &numbers);
    stoogesort(&mut numbers);
    println!("After: {:?}", &numbers);
}
