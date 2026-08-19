// Rosetta Code task: Unique characters in each string
// Source: https://rosettacode.org/wiki/Unique_characters_in_each_string#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// ['1', '2', '3', 'a', 'b', 'c']
// =======================

fn main() {
    let list = ["1a3c52debeffd", "2b6178c97a938stf", "3ycxdb1fgxa2yz"];
    let mut charset: Vec<char> = list.join("").chars().collect();
    charset.sort();
    charset.dedup();
    println!(
        "{:?}",
        charset
            .iter()
            .filter(|c| {
                list.iter()
                    .filter(|s| s.chars().filter(|x| &x == c).count() == 1)
                    .count()
                    == list.len()
            })
            .collect::<Vec<&char>>()
    );
}
