// Rosetta Code task: Square but not cube
// Source: https://rosettacode.org/wiki/Square_but_not_cube#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 1 is a square and a cube.
// 4
// 9
// 16
// 25
// 36
// 49
// 64 is a square and a cube.
// 81
// 100
// 121
// 144
// 169
// 196
// 225
// 256
// 289
// 324
// 361
// 400
// 441
// 484
// 529
// 576
// 625
// 676
// 729 is a square and a cube.
// 784
// 841
// 900
// 961
// 1024
// 1089
// =======================

fn main() {
    let mut s = 1;
    let mut c = 1;
    let mut cube = 1;
    let mut n = 0;
    while n < 30 {
        let square = s * s;
        while cube < square {
            c += 1;
            cube = c * c * c;
        }
        if cube == square {
            println!("{} is a square and a cube.", square);
        } else {
            println!("{}", square);
            n += 1;
        }
        s += 1;
    }
}
