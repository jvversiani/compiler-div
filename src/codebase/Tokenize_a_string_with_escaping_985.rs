// Rosetta Code task: Tokenize a string with escaping
// Source: https://rosettacode.org/wiki/Tokenize_a_string_with_escaping#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// [
//     "one|uno",
//     "",
//     "three^^",
//     "four^|cuatro",
//     "",
// ]
// =======================

const SEPARATOR: char = '|';
const ESCAPE: char = '^';
const STRING: &str = "one^|uno||three^^^^|four^^^|^cuatro|";

fn tokenize(string: &str) -> Vec<String> {
    let mut token = String::new();
    let mut tokens: Vec<String> = Vec::new();
    let mut chars = string.chars();
    while let Some(ch) = chars.next() {
        match ch {
            SEPARATOR => {
                tokens.push(token);
                token = String::new();
            },
            ESCAPE => {
                if let Some(next) = chars.next() {
                    token.push(next);
                }
            },
            _ => token.push(ch),
        }
    }
    tokens.push(token);
    tokens
}

fn main() {
    println!("{:#?}", tokenize(STRING));
}
