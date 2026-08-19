// Rosetta Code task: Collect and sort square numbers in ascending order from three lists
// Source: https://rosettacode.org/wiki/Collect_and_sort_square_numbers_in_ascending_order_from_three_lists#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// [9, 16, 25, 36, 36, 49, 81, 121, 144, 169]
// =======================

fn main() {
    let list1 = [3, 3, 34, 25, 9, 12, 36, 56, 36];
    let list2 = [2, 8, 81, 169, 34, 55, 76, 49, 7];
    let list3 = [75, 121, 75, 144, 35, 16, 46, 35];

    let mut found: Vec<_> = list1
        .iter()
        .chain(list2.iter())
        .chain(list3.iter())
        .filter(|&x: &&i32| *x == x.isqrt().pow(2))
        .collect();

    found.sort();
    // found.dedup(); // Some solutions only keep unique numbers

    println!("{found:?}");
}
