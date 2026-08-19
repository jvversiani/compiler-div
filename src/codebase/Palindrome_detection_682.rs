// Rosetta Code task: Palindrome detection
// Source: https://rosettacode.org/wiki/Palindrome_detection#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// '': true
// 'a': true
// 'ada': true
// 'adad': false
// 'ingirumimusnocteetconsumimurigni': true
// '人人為我,我為人人': true
// 'Я иду с мечем, судия': false
// '아들딸들아': true
// 'The quick brown fox': false
// =======================

fn is_palindrome(string: &str) -> bool {
    let half_len = string.len() / 2;
    string
        .chars()
        .take(half_len)
        .eq(string.chars().rev().take(half_len))
}

macro_rules! test {
    ( $( $x:tt ),* ) => { $( println!("'{}': {}", $x, is_palindrome($x)); )* };
}

fn main() {
    test!(
        "",
        "a",
        "ada",
        "adad",
        "ingirumimusnocteetconsumimurigni",
        "人人為我,我為人人",
        "Я иду с мечем, судия",
        "아들딸들아",
        "The quick brown fox"
    );
}
