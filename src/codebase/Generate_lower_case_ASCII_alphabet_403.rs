// Rosetta Code task: Generate lower case ASCII alphabet
// Source: https://rosettacode.org/wiki/Generate_lower_case_ASCII_alphabet#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z']
// =======================

fn main() {
    // An iterator over the lowercase alpha's
    let ascii_iter = (0..26)
        .map(|x| (x + b'a') as char);
 
    println!("{:?}", ascii_iter.collect::<Vec<char>>());
}
