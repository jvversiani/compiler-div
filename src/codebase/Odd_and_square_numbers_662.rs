// Rosetta Code task: Odd and square numbers
// Source: https://rosettacode.org/wiki/Odd_and_square_numbers#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 121
// 169
// 225
// 289
// 361
// 441
// 529
// 625
// 729
// 841
// 961
// =======================

fn main() {
    let odd_squares =
        core::iter::successors(Some([1, 8]), |[acc, x]| Some([acc + x, x + 8])).map(|[x, _]| x);
    
    for i in odd_squares
        .skip_while(|&x| x < 100)
        .take_while(|&x| x < 1000)
    {
        println!("{i}");
    }
}
