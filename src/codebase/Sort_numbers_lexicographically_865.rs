// Rosetta Code task: Sort numbers lexicographically
// Source: https://rosettacode.org/wiki/Sort_numbers_lexicographically#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 0: [0, 1]
// 5: [1, 2, 3, 4, 5]
// 13: [1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9]
// 21: [1, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 2, 20, 21, 3, 4, 5, 6, 7, 8, 9]
// -22: [-1, -10, -11, -12, -13, -14, -15, -16, -17, -18, -19, -2, -20, -21, -22, -3, -4, -5, -6, -7, -8, -9, 0, 1]
// =======================

fn lex_sorted_vector(num: i32) -> Vec<i32> {
    let (min, max) = if num >= 1 { (1, num) } else { (num, 1) };
    let mut str: Vec<String> = (min..=max).map(|i| i.to_string()).collect();
    str.sort();
    str.iter().map(|s| s.parse::<i32>().unwrap()).collect()
}

fn main() {
    for n in &[0, 5, 13, 21, -22] {
        println!("{}: {:?}", n, lex_sorted_vector(*n));
    }
}
