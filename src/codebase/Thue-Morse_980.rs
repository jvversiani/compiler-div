// Rosetta Code task: Thue-Morse
// Source: https://rosettacode.org/wiki/Thue-Morse#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 1: 0
// 2: 01
// 3: 0110
// 4: 01101001
// 5: 0110100110010110
// 6: 01101001100101101001011001101001
// 7: 0110100110010110100101100110100110010110011010010110100110010110
// 8: 01101001100101101001011001101001100101100110100101101001100101101001011001101001011010011001011001101001100101101001011001101001
// =======================

const ITERATIONS: usize = 8;

fn neg(sequence: &String) -> String {
    sequence.chars()
        .map(|ch| {
            (1 - ch.to_digit(2).unwrap()).to_string()
        })
        .collect::<String>()
}

fn main() {
    let mut sequence: String = String::from("0");
    for i in 0..ITERATIONS {
        println!("{}: {}", i + 1, sequence);
        sequence = format!("{}{}", sequence, neg(&sequence));
    }
}
