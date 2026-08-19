// Rosetta Code task: Shoelace formula for polygonal area
// Source: https://rosettacode.org/wiki/Shoelace_formula_for_polygonal_area#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 30
// =======================

fn shoelace(points: &[(f64, f64)]) -> f64 {
    let mut left_sum = 0.0;
    let mut right_sum = 0.0;

    for i in 0..points.len() {
        let j = (i + 1) % points.len();
        left_sum += points[i].0 * points[j].1;
        right_sum += points[j].0 * points[i].1;
    }

    0.5 * (left_sum - right_sum).abs()
}

fn main() {
    let points: Vec<(f64, f64)> = vec![
        (3.0, 4.0),
        (5.0, 11.0),
        (12.0, 8.0),
        (9.0, 5.0),
        (5.0, 6.0),
    ];

    let ans = shoelace(&points);
    println!("{}", ans);
}
