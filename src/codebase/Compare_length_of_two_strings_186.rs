// Rosetta Code task: Compare length of two strings
// Source: https://rosettacode.org/wiki/Compare_length_of_two_strings#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// "a" and "b" are of equal length, 1
//
// "e" has length 1 and is the longest
// "cd" has length 2 and is the shortest
//
// "gh" has length 2 and is the longest
// "f" has length 1 and is the shortest
// =======================

fn compare_and_report<T: ToString>(string1: T, string2: T) -> String {
    let strings = [string1.to_string(), string2.to_string()];
    let difference = strings[0].len() as i32 - strings[1].len() as i32;
    if difference == 0 { // equal
        format!("\"{}\" and \"{}\" are of equal length, {}", strings[0], strings[1], strings[0].len())
    } else if difference > 1 { // string1 > string2
        format!("\"{}\" has length {} and is the longest\n\"{}\" has length {} and is the shortest", strings[0], strings[0].len(), strings[1], strings[1].len())
    } else { // string2 > string1
        format!("\"{}\" has length {} and is the longest\n\"{}\" has length {} and is the shortest", strings[1], strings[1].len(), strings[0], strings[0].len())
    }
}

fn main() {
    println!("{}", compare_and_report("a", "b"));
    println!("\n{}", compare_and_report("cd", "e"));
    println!("\n{}", compare_and_report("f", "gh"));
}
