// Rosetta Code task: Repeat
// Source: https://rosettacode.org/wiki/Repeat#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 1
// 2
// 3
// 4
// 5
// 6
// 7
// 8
// 9
// 10
// =======================

fn repeat(f: impl FnMut(usize), n: usize) {
    (0..n).for_each(f);
}

fn main(){
    let add_one = | x: usize | println!("{}", x + 1);
    repeat(add_one, 10);
}