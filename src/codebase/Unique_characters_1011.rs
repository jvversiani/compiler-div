// Rosetta Code task: Unique characters
// Source: https://rosettacode.org/wiki/Unique_characters#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// ['1', '5', '6', 'b', 'g', 's', 't', 'z']
// =======================

fn uniquein2(a: Vec<&str>) -> Vec<char> {
    let mut s: Vec<char> = a.join("").chars().into_iter().collect();
    s.sort();
    let n = s.len();
    return s
        .iter()
        .enumerate()
        .filter(|(i, c)| (*i == 0_usize || **c != s[i - 1]) && (*i == n - 1 || **c != s[i + 1]))
        .map(|(_i, c)| *c)
        .collect();
}

fn main() {
    let list = ["133252abcdeeffd", "a6789798st", "yxcdfgxcyz"].to_vec();
    println!("{:?}", uniquein2(list));
}
