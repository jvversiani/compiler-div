// Rosetta Code task: Apply a callback to an array
// Source: https://rosettacode.org/wiki/Apply_a_callback_to_an_array#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 1
// 2
// 3
// 4
// 5
// =======================

fn echo(n: &i32) {
    println!("{}", n);
}

fn main() {
    let a: [i32; 5];
    a = [1, 2, 3, 4, 5];
    let _: Vec<_> = a.iter().map(echo).collect();
}
