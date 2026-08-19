// Rosetta Code task: Binary search
// Source: https://rosettacode.org/wiki/Binary_search#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// The index of 4 in [1, 2, 3, 4, 5] is Some(3)
// The index of 7 in [1, 2, 3, 4, 5] is None
// =======================

fn binary_search<T:PartialOrd>(searchvalue: T, v: &[T] ) -> Option<usize> {
    let mut lower = 0 as usize;
    let mut upper = v.len();
    while upper > lower {
        let mid = lower + (upper - lower) / 2;
        if v[mid] == searchvalue {
            return Some(mid);
        } else if searchvalue < v[mid] {
            upper = mid;
        } else {
            lower = mid + 1;
        }
    }
    None
}

fn main() {
    let values: Vec<u32> = vec![1, 2, 3, 4, 5];
    println!("The index of 4 in {:?} is {:?}", values, binary_search(4, &values));
    println!("The index of 7 in {:?} is {:?}", values, binary_search(7, &values));
}