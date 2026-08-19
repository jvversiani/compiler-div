// Rosetta Code task: Loops/Do-while
// Source: https://rosettacode.org/wiki/Loops/Do-while#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 1
// 2
// 3
// 4
// 5
// 6
// =======================

fn main(){
    let mut x = 0;

    loop {
        x += 1;
        println!("{}", x);

        if x % 6 == 0 { break; }
    }
}
