// Rosetta Code task: Sorting algorithms/Merge sort
// Source: https://rosettacode.org/wiki/Sorting_algorithms/Merge_sort#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Before: [4, 3, 2, 1, 0]
// After: [0, 1, 2, 3, 4]
// =======================

pub fn merge_sort1<T: Copy + Ord>(v: &mut [T]) {
    sort(v, &mut Vec::new());

    fn sort<T: Copy + Ord>(v: &mut [T], t: &mut Vec<T>) {
        match v.len() {
            0 | 1 => (),
            // n if n <= 20 => insertion_sort(v),
            n => {
                if t.is_empty() {
                    t.reserve_exact(n);
                    t.resize(n, v[0]);
                }
                let m = n / 2;
                sort(&mut v[..m], t);
                sort(&mut v[m..], t);
                if v[m - 1] <= v[m] {
                    return;
                }
                copy(v, t);
                merge(&t[..m], &t[m..n], v);
            }
        }
    }

    // merge a + b -> c
    #[inline(always)]
    fn merge<T: Copy + Ord>(a: &[T], b: &[T], c: &mut [T]) {
        let (mut i, mut j) = (0, 0);
        for k in 0..c.len() {
            if i < a.len() && (j >= b.len() || a[i] <= b[j]) {
                c[k] = a[i];
                i += 1;
            } else {
                c[k] = b[j];
                j += 1;
            }
        }
    }

    #[inline(always)]
    fn copy<T: Copy>(src: &[T], dst: &mut [T]) {
        for i in 0..src.len() {
            dst[i] = src[i];
        }
    }
}

fn main(){
    let mut valeurs = vec![4, 3, 2, 1, 0];
    println!("Before: {:?}", valeurs);
    merge_sort1(&mut valeurs);
    println!("After: {:?}", valeurs);
}
