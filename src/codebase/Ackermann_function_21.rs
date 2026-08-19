// Rosetta Code task: Ackermann function
// Source: https://rosettacode.org/wiki/Ackermann_function#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 125
// =======================

fn ack(m: isize, n: isize) -> isize {
    if m == 0 {
        n + 1
    } else if n == 0 {
        ack(m - 1, 1)
    } else {
        ack(m - 1, ack(m, n - 1))
    }
}

fn main() {
    let a = ack(3, 4);
    println!("{}", a); // 125
}
