// Rosetta Code task: Josephus problem
// Source: https://rosettacode.org/wiki/Josephus_problem#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// JOSEPHUS n=41, k=3, m=3
// Executed: [2, 5, 8, 11, 14, 17, 20, 23, 26, 29, 32, 35, 38, 0, 4, 9, 13, 18, 22, 27, 31, 36, 40, 6, 12, 19, 25, 33, 39, 7, 16, 28, 37, 10, 24, 1, 21, 3]
// Executed position number 5: 14
// Survivors: [15, 30, 34]
// =======================

const N: usize = 41;
const K: usize = 3;
const M: usize = 3;
const POSITION: usize = 5;

fn main() {
    let mut prisoners: Vec<usize> = Vec::new();
    let mut executed: Vec<usize> = Vec::new();
    for pos in 0..N {
        prisoners.push(pos);
    }

    let mut to_kill: usize = 0;
    let mut len: usize = prisoners.len();

    while len > M {
        to_kill = (to_kill + K - 1) % len;
        executed.push(prisoners.remove(to_kill));
        len -= 1;
    }

    println!("JOSEPHUS n={}, k={}, m={}", N, K, M);
    println!("Executed: {:?}", executed);
    println!("Executed position number {}: {}", POSITION, executed[POSITION - 1]);
    println!("Survivors: {:?}", prisoners);
}
