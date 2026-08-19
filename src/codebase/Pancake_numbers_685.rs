// Rosetta Code task: Pancake numbers
// Source: https://rosettacode.org/wiki/Pancake_numbers#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// p( 1) =  0  p( 2) =  1  p( 3) =  3  p( 4) =  4  p( 5) =  5  
// p( 6) =  7  p( 7) =  8  p( 8) =  9  p( 9) = 10  p(10) = 11  
// p(11) = 13  p(12) = 14  p(13) = 15  p(14) = 16  p(15) = 17  
// p(16) = 18  p(17) = 19  p(18) = 20  p(19) = 21  p(20) = 23
// =======================

fn pancake(n: i32) -> i32 {
    let mut gap = 2;
    let mut sum = 2;
    let mut adj = -1;

    while sum < n {
        adj += 1;
        gap = gap * 2 - 1;
        sum += gap;
    }

    n + adj
}

fn main() {
    for i in 0..4 {
        for j in 1..6 {
            let n = i * 5 + j;
            print!("p({:2}) = {:2}  ", n, pancake(n));
        }
        println!();
    }
}
