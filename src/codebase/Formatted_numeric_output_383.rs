// Rosetta Code task: Formatted numeric output
// Source: https://rosettacode.org/wiki/Formatted_numeric_output#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
//     7.125
// 00007.125
//    -7.125
// -0007.125
// =======================

fn main() {
    let x = 7.125;

    println!("{:9}", x);
    println!("{:09}", x);
    println!("{:9}", -x);
    println!("{:09}", -x);
}
