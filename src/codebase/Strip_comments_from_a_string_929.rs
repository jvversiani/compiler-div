// Rosetta Code task: Strip comments from a string
// Source: https://rosettacode.org/wiki/Strip_comments_from_a_string#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// "apples, pears"
// "apples, pears"
// "apples, pears and bananas"
// =======================

fn strip_comment<'a>(input: &'a str, markers: &[char]) -> &'a str {
    input
        .find(markers)
        .map(|idx| &input[..idx])
        .unwrap_or(input)
        .trim()
}

fn main() {
    println!("{:?}", strip_comment("apples, pears # and bananas", &['#', ';']));
    println!("{:?}", strip_comment("apples, pears ; and bananas", &['#', ';']));
    println!("{:?}", strip_comment("apples, pears and bananas ", &['#', ';']));
}
