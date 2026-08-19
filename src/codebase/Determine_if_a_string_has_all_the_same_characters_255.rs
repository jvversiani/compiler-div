// Rosetta Code task: Determine if a string has all the same characters
// Source: https://rosettacode.org/wiki/Determine_if_a_string_has_all_the_same_characters#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Checking string "" of length 0:
//         All characters in the string are the same
// Checking string "   " of length 3:
//         All characters in the string are the same
// Checking string "2" of length 1:
//         All characters in the string are the same
// Checking string "333" of length 3:
//         All characters in the string are the same
// Checking string ".55" of length 3:
//         Not all characters are the same.
//         '5' (0x35) at position 2 differs.
// Checking string "tttTTT" of length 6:
//         Not all characters are the same.
//         'T' (0x54) at position 4 differs.
// Checking string "4444 444k" of length 9:
//         Not all characters are the same.
//         ' ' (0x20) at position 5 differs.
// Checking string "pépé" of length 4:
//         Not all characters are the same.
//         'é' (0xE9) at position 2 differs.
// Checking string "🐶🐶🐺🐶" of length 4:
//         Not all characters are the same.
//         '🐺' (0x1F43A) at position 3 differs.
// Checking string "🎄🎄🎄🎄" of length 4:
//         All characters in the string are the same
// =======================

fn test_string(input: &str) {
    println!("Checking string {:?} of length {}:", input, input.chars().count());

    let mut chars = input.chars();

    match chars.next() {
        Some(first) => {
            if let Some((character, pos)) = chars.zip(2..).filter(|(c, _)| *c != first).next() {
                println!("\tNot all characters are the same.");
                println!("\t{:?} (0x{:X}) at position {} differs.", character, character as u32, pos);

                return;
            }
        },
        None => {}
    }

    println!("\tAll characters in the string are the same");
}

fn main() {
    let tests = ["", "   ", "2", "333", ".55", "tttTTT", "4444 444k", "pépé", "🐶🐶🐺🐶", "🎄🎄🎄🎄"];

    for string in &tests {
        test_string(string);
    }
}
