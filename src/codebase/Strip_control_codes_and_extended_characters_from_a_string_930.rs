// Rosetta Code task: Strip control codes and extended characters from a string
// Source: https://rosettacode.org/wiki/Strip_control_codes_and_extended_characters_from_a_string#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// abcd
// =======================

fn stripped(tostrip: &str) -> String {
    return tostrip
        .chars()
        .filter(|c| !c.is_ascii_control() && c.is_ascii())
        .collect();
}

fn main() {
    println!("{}", stripped("\x08a\x00b\n\rc\x0cd\u{00c3}"));
}
