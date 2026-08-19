// Rosetta Code task: Sorting algorithms/Cocktail sort
// Source: https://rosettacode.org/wiki/Sorting_algorithms/Cocktail_sort#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// before: [10, 8, 4, 3, 1, 9, 0, 2, 7, 5, 6]
// after:  [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
// =======================

fn cocktail_sort<T: PartialOrd>(a: &mut [T]) {
    let len = a.len();
    loop {
        let mut swapped = false;
        let mut i = 0;
        while i + 1 < len {
            if a[i] > a[i + 1] {
                a.swap(i, i + 1);
                swapped = true;
            }
            i += 1;
        }
        if swapped {
            swapped = false;
            i = len - 1;
            while i > 0 {
                if a[i - 1] > a[i] {
                    a.swap(i - 1, i);
                    swapped = true;
                }
                i -= 1;
            }
        }
        if !swapped {
            break;
        }
    }
}

fn main() {
    let mut v = vec![10, 8, 4, 3, 1, 9, 0, 2, 7, 5, 6];
    println!("before: {:?}", v);
    cocktail_sort(&mut v);
    println!("after:  {:?}", v);
}
