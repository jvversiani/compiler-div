// Rosetta Code task: Sorting algorithms/Selection sort
// Source: https://rosettacode.org/wiki/Sorting_algorithms/Selection_sort#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// The initial array is [9, 4, 8, 3, -5, 2, 1, 6]
//  The sorted array is [-5, 1, 2, 3, 4, 6, 8, 9]
// =======================

fn selection_sort(array: &mut [i32]) {

    let mut min;

    for i in 0..array.len() {

        min = i;

        for j in (i+1)..array.len() {

            if array[j] < array[min] {
                min = j;
            }
        }

        let tmp = array[i];
        array[i] = array[min];
        array[min] = tmp;
    }
}

fn main() {

    let mut array = [ 9, 4, 8, 3, -5, 2, 1, 6 ];
    println!("The initial array is {:?}", array);

    selection_sort(&mut array);
    println!(" The sorted array is {:?}", array);
}
