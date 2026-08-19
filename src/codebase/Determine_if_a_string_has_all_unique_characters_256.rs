// Rosetta Code task: Determine if a string has all unique characters
// Source: https://rosettacode.org/wiki/Determine_if_a_string_has_all_unique_characters#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// "" (length 0) is unique
// "." (length 1) is unique
// "abcABC" (length 6) is unique
// "XYZ ZYX" (length 7) is not unique
// 	first duplicate: "X" (U+0058) at indices 0 and 6
// "1234567890ABCDEFGHIJKLMN0PQRSTUVWXYZ" (length 36) is not unique
// 	first duplicate: "0" (U+0030) at indices 9 and 24
// "01234567890ABCDEFGHIJKLMN0PQRSTUVWXYZ0X" (length 39) is not unique
// 	first duplicate: "0" (U+0030) at indices 0 and 10
// "hétérogénéité" (length 13) is not unique
// 	first duplicate: "é" (U+00E9) at indices 1 and 3
// "🎆🎃🎇🎈" (length 4) is unique
// "😍😀🙌💃😍🙌" (length 6) is not unique
// 	first duplicate: "😍" (U+1F60D) at indices 0 and 4
// "🐠🐟🐡🦈🐬🐳🐋🐡" (length 8) is not unique
// 	first duplicate: "🐡" (U+1F421) at indices 2 and 7
// =======================

fn unique(s: &str) -> Option<(usize, usize, char)> {
    s.chars().enumerate().find_map(|(i, c)| {
        s.chars()
            .enumerate()
            .skip(i + 1)
            .find(|(_, other)| c == *other)
            .map(|(j, _)| (i, j, c))
    })
}

fn main() {
    let strings = [
        "",
        ".",
        "abcABC",
        "XYZ ZYX",
        "1234567890ABCDEFGHIJKLMN0PQRSTUVWXYZ",
        "01234567890ABCDEFGHIJKLMN0PQRSTUVWXYZ0X",
        "hétérogénéité",
        "🎆🎃🎇🎈",
        "😍😀🙌💃😍🙌",
        "🐠🐟🐡🦈🐬🐳🐋🐡",
    ];

    for string in &strings {
        print!("\"{}\" (length {})", string, string.chars().count());
        match unique(string) {
            None => println!(" is unique"),
            Some((i, j, c)) => println!(
                " is not unique\n\tfirst duplicate: \"{}\" (U+{:0>4X}) at indices {} and {}",
                c, c as usize, i, j
            ),
        }
    }
}
