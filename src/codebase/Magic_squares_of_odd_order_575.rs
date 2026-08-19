// Rosetta Code task: Magic squares of odd order
// Source: https://rosettacode.org/wiki/Magic_squares_of_odd_order#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
//  47  58  69  80   1  12  23  34  45 
//  57  68  79   9  11  22  33  44  46 
//  67  78   8  10  21  32  43  54  56 
//  77   7  18  20  31  42  53  55  66 
//   6  17  19  30  41  52  63  65  76 
//  16  27  29  40  51  62  64  75   5 
//  26  28  39  50  61  72  74   4  15 
//  36  38  49  60  71  73   3  14  25 
//  37  48  59  70  81   2  13  24  35 
// The sum of the square is 369.
// =======================

fn main() {
    let n = 9;
    let mut square = vec![vec![0; n]; n];
    for (i, row) in square.iter_mut().enumerate() {
        for (j, e) in row.iter_mut().enumerate() {
            *e = n * (((i + 1) + (j + 1) - 1 + (n >> 1)) % n) + (((i + 1) + (2 * (j + 1)) - 2) % n) + 1;
            print!("{:3} ", e);
        }
        println!("");
    }
    let sum = n * (((n * n) + 1) / 2);
    println!("The sum of the square is {}.", sum);
}
