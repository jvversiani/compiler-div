// Rosetta Code task: Averages/Root mean square
// Source: https://rosettacode.org/wiki/Averages/Root_mean_square#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// The root mean square is: 6.204837
// =======================

fn root_mean_square(vec: Vec<i32>) -> f32 {
    let sum_squares = vec.iter().fold(0, |acc, &x| acc + x.pow(2));
    return ((sum_squares as f32)/(vec.len() as f32)).sqrt();
}

fn main() {
    let vec = (1..11).collect();
    println!("The root mean square is: {}", root_mean_square(vec));
}
