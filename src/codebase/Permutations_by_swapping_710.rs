// Rosetta Code task: Permutations by swapping
// Source: https://rosettacode.org/wiki/Permutations_by_swapping#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Permutations and signs for three items:
// [0, 1, 2] 1
// [1, 0, 2] -1
// [2, 0, 1] 1
// [0, 2, 1] -1
// [1, 2, 0] 1
// [2, 1, 0] -1
//
// Permutations and signs for four items:
// [0, 1, 2, 3] 1
// [1, 0, 2, 3] -1
// [2, 0, 1, 3] 1
// [0, 2, 1, 3] -1
// [1, 2, 0, 3] 1
// [2, 1, 0, 3] -1
// [3, 1, 0, 2] 1
// [1, 3, 0, 2] -1
// [0, 3, 1, 2] 1
// [3, 0, 1, 2] -1
// [1, 0, 3, 2] 1
// [0, 1, 3, 2] -1
// [0, 2, 3, 1] 1
// [2, 0, 3, 1] -1
// [3, 0, 2, 1] 1
// [0, 3, 2, 1] -1
// [2, 3, 0, 1] 1
// [3, 2, 0, 1] -1
// [3, 2, 1, 0] 1
// [2, 3, 1, 0] -1
// [1, 3, 2, 0] 1
// [3, 1, 2, 0] -1
// [2, 1, 3, 0] 1
// [1, 2, 3, 0] -1
// =======================

// Implementation of Heap's algorithm.
// See https://en.wikipedia.org/wiki/Heap%27s_algorithm#Details_of_the_algorithm
fn generate<T, F>(a: &mut [T], output: F)
where
    F: Fn(&[T], isize),
{
    let n = a.len();
    let mut c = vec![0; n];
    let mut i = 1;
    let mut sign = 1;
    output(a, sign);
    while i < n {
        if c[i] < i {
            if (i & 1) == 0 {
                a.swap(0, i);
            } else {
                a.swap(c[i], i);
            }
            sign = -sign;
            output(a, sign);
            c[i] += 1;
            i = 1;
        } else {
            c[i] = 0;
            i += 1;
        }
    }
}

fn print_permutation<T: std::fmt::Debug>(a: &[T], sign: isize) {
    println!("{:?} {}", a, sign);
}

fn main() {
    println!("Permutations and signs for three items:");
    let mut a = vec![0, 1, 2];
    generate(&mut a, print_permutation);

    println!("\nPermutations and signs for four items:");
    let mut b = vec![0, 1, 2, 3];
    generate(&mut b, print_permutation);
}
