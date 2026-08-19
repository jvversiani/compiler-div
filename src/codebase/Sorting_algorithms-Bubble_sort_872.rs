// Rosetta Code task: Sorting algorithms/Bubble sort
// Source: https://rosettacode.org/wiki/Sorting_algorithms/Bubble_sort#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Before: [8, 7, 1, 2, 9, 3, 4, 5, 0, 6]
// After: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
// Before: ["empty", "beach", "art", "car", "deal"]
// After: ["art", "beach", "car", "deal", "empty"]
// =======================

fn bubble_sort<T: Ord>(values: &mut[T]) {
    let mut n = values.len();
    let mut swapped = true;

    while swapped {
        swapped = false;

        for i in 1..n {
            if values[i - 1] > values[i] {
                values.swap(i - 1, i);
                swapped = true;
            }
        }

        n = n - 1;
    }
}

fn main() {
    // Sort numbers.
    let mut numbers = [8, 7, 1, 2, 9, 3, 4, 5, 0, 6];
    println!("Before: {:?}", numbers);

    bubble_sort(&mut numbers);
    println!("After: {:?}", numbers);

    // Sort strings.
    let mut strings = ["empty", "beach", "art", "car", "deal"];
    println!("Before: {:?}", strings);

    bubble_sort(&mut strings);
    println!("After: {:?}", strings);
}
