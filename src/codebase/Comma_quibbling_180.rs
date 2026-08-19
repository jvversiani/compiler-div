// Rosetta Code task: Comma quibbling
// Source: https://rosettacode.org/wiki/Comma_quibbling#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// {}
// {ABC}
// {ABC and DEF}
// {ABC, DEF, G and H}
// =======================

fn quibble(seq: &[&str]) -> String {
    match seq.len() {
        0 => "{}".to_string(),
        1 => format!("{{{}}}", seq[0]),
        _ => {
            format!("{{{} and {}}}",
                    seq[..seq.len() - 1].join(", "),
                    seq.last().unwrap())
        }
    }
}

fn main() {
    println!("{}", quibble(&[]));
    println!("{}", quibble(&["ABC"]));
    println!("{}", quibble(&["ABC", "DEF"]));
    println!("{}", quibble(&["ABC", "DEF", "G", "H"]));
}
