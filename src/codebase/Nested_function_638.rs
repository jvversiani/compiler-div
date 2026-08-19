// Rosetta Code task: Nested function
// Source: https://rosettacode.org/wiki/Nested_function#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 1. First
// 2. Second
// 3. Third
// =======================

fn make_list(sep: &str) -> String {
    let mut counter = 0;
    let mut make_item = |label| {
        counter += 1;
        format!("{}{}{}", counter, sep, label)
    };
    format!(
        "{}\n{}\n{}",
        make_item("First"),
        make_item("Second"),
        make_item("Third")
    )
}

fn main() {
    println!("{}", make_list(". "))
}
