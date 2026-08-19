// Rosetta Code task: Strip a set of characters from a string
// Source: https://rosettacode.org/wiki/Strip_a_set_of_characters_from_a_string#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// The string 'aaaaaaaaaabbaaaaaaaaaaccaaaa' without 'bc' becomes aaaaaaaaaaaaaaaaaaaaaaaa
// =======================

fn strip_characters(original: &str, to_strip: &str) -> String {
    let mut result = String::new();
    for c in original.chars() {
        if !to_strip.contains(c) {
            result.push(c);
        }
    }
    result
}

fn main(){
    println!("The string 'aaaaaaaaaabbaaaaaaaaaaccaaaa' without 'bc' becomes {} ", strip_characters("aaaaaaaaaabbaaaaaaaaaaccaaaa", "bc"));
}