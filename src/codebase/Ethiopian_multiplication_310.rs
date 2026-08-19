// Rosetta Code task: Ethiopian multiplication
// Source: https://rosettacode.org/wiki/Ethiopian_multiplication#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 17       34      Kept
// 8        68      Not Kept
// 4        136     Not Kept
// 2        272     Not Kept
// 1        544     Kept
// ---------------------------------
//          578
// =======================

fn double(a: i32) -> i32 {
    2*a
}

fn halve(a: i32) -> i32 {
    a/2
}

fn is_even(a: i32) -> bool {
    a % 2 == 0
}

fn ethiopian_multiplication(mut x: i32, mut y: i32) -> i32 {
    let mut sum = 0;

    while x >= 1 {
        print!("{} \t {}", x, y);
        match is_even(x) {
            true  => println!("\t Not Kept"),
            false => {
                println!("\t Kept");
                sum += y;
            }
        }
        x = halve(x);
        y = double(y);
    }
    sum
}

fn main() {
    let output = ethiopian_multiplication(17, 34);
    println!("---------------------------------");
    println!("\t {}", output);
}
