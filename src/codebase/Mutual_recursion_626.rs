// Rosetta Code task: Mutual recursion
// Source: https://rosettacode.org/wiki/Mutual_recursion#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 1 1 2 2 3 3 4 5 5 6 6 7 8 8 9 9 10 11 11 12
// 0 0 1 2 2 3 4 4 5 6 6 7 7 8 9 9 10 11 11 12
// =======================

fn f(n: u32) -> u32 {
    match n {
        0 => 1,
        _ => n - m(f(n - 1))
    }
}

fn m(n: u32) -> u32 {
    match n {
        0 => 0,
        _ => n - f(m(n - 1))
    }
}

fn main() {
    for i in (0..20).map(f) {
        print!("{} ", i);
    }
    println!("");

    for i in (0..20).map(m) {
        print!("{} ", i);
    }
    println!("")
}
