// Rosetta Code task: Sorting algorithms/Gnome sort
// Source: https://rosettacode.org/wiki/Sorting_algorithms/Gnome_sort#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// before: [10, 8, 4, 3, 1, 9, 0, 2, 7, 5, 6]
// after:  [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
// =======================

fn gnome_sort<T: PartialOrd>(a: &mut [T]) {
    let len = a.len();
    let mut i: usize = 1;
    let mut j: usize = 2;
    while i < len {
        if a[i - 1] <= a[i] {
            // for descending sort, use >= for comparison
            i = j;
            j += 1;
        } else {
            a.swap(i - 1, i);
            i -= 1;
            if i == 0 {
                i = j;
                j += 1;
            }
        }
    }
}

fn main() {
    let mut v = vec![10, 8, 4, 3, 1, 9, 0, 2, 7, 5, 6];
    println!("before: {:?}", v);
    gnome_sort(&mut v);
    println!("after:  {:?}", v);
}
