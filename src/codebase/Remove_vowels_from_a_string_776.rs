// Rosetta Code task: Remove vowels from a string
// Source: https://rosettacode.org/wiki/Remove_vowels_from_a_string#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Ferris, the crab, is the unofficial mascot of the Rust Programming Language
// Frrs, th crb, s th nffcl msct f th Rst Prgrmmng Lngg
// =======================

fn remove_vowels(str: String) -> String {
    let vowels = "aeiouAEIOU";
    let mut devowelled_string = String::from("");

    for i in str.chars() {
        if vowels.contains(i) {
            continue;
        } else {
            devowelled_string.push(i);
        }
    }
    return devowelled_string;
}

fn main() {
    let intro =
        String::from("Ferris, the crab, is the unofficial mascot of the Rust Programming Language");
    println!("{}", intro);
    println!("{}", remove_vowels(intro));
}
