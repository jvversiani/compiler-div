// Rosetta Code task: Compare a list of strings
// Source: https://rosettacode.org/wiki/Compare_a_list_of_strings#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// These strings are equal blabla and blablu ? false
// These strings are equal blabla and blablu ? true
// These strings are asc blabla and blablu ? true
// These strings are asc blabla and blablu ? false
// =======================

fn strings_are_equal(seq: &[&str]) -> bool {
    match seq {
        &[] | &[_] => true,
        &[x, y, ref tail @ ..] if x == y => strings_are_equal(&[&[y], tail].concat()),
        _ => false
    }
}

fn asc_strings(seq: &[&str]) -> bool {
    match seq {
        &[] | &[_] => true,
        &[x, y, ref tail @ ..] if x < y => asc_strings(&[&[y], tail].concat()),
        _ => false
    }
}

fn main(){
    let str1: &str = "blabla";
    let str2: &str = "blablu";
    let str3: &str = "blabla";
    println!("These strings are equal {} and {} ? {}", str1, str2, strings_are_equal(&[str1, str2]));
    println!("These strings are equal {} and {} ? {}", str1, str2, strings_are_equal(&[str1, str3]));
    println!("These strings are asc {} and {} ? {}", str1, str2, asc_strings(&[str1, str2]));
    println!("These strings are asc {} and {} ? {}", str1, str2, asc_strings(&[str2, str1]));
}