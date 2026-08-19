// Rosetta Code task: Sum and product of an array
// Source: https://rosettacode.org/wiki/Sum_and_product_of_an_array#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// the sum is 45 and the product is 362880
// the sum is 45 and the product is 362880
// =======================

fn main() {
    let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    // using fold
    let sum = arr.iter().fold(0i32, |a, &b| a + b);
    let product = arr.iter().fold(1i32, |a, &b| a * b);
    println!("the sum is {} and the product is {}", sum, product);

    // or using sum and product
    let sum = arr.iter().sum::<i32>();
    let product = arr.iter().product::<i32>();
    println!("the sum is {} and the product is {}", sum, product);
}
