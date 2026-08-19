// Rosetta Code task: Greatest subsequential sum
// Source: https://rosettacode.org/wiki/Greatest_subsequential_sum#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Max subsequence sum: 96 for [1, 2, 39, 34, 20]
// =======================

fn main() {
    let nums = [1,2,39,34,20, -20, -16, 35, 0];

    let mut max = 0;
    let mut boundaries = 0..0;

    for length in 0..nums.len() {
        for start in 0..nums.len()-length {
            let sum = (&nums[start..start+length]).iter()
                .fold(0, |sum, elem| sum+elem);
            if sum > max {
                max = sum;
                boundaries = start..start+length;
            }
        }
    }

    println!("Max subsequence sum: {} for {:?}", max, &nums[boundaries]);;
}
