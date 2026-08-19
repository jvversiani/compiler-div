// Rosetta Code task: Numerical integration
// Source: https://rosettacode.org/wiki/Numerical_integration#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 0.2500250000000004
// 4.605986057514688
// 12500000.000728702
// 18000000.001390498
// =======================

fn integral<F>(f: F, range: std::ops::Range<f64>, n_steps: u32) -> f64
    where F: Fn(f64) -> f64
{
    let step_size = (range.end - range.start)/n_steps as f64;

    let mut integral = (f(range.start) + f(range.end))/2.;
    let mut pos = range.start + step_size;
    while pos < range.end {
        integral += f(pos);
        pos += step_size;
    }
    integral * step_size
}

fn main() {
    println!("{}", integral(|x| x.powi(3), 0.0..1.0, 100));
    println!("{}", integral(|x| 1.0/x, 1.0..100.0, 1000));
    println!("{}", integral(|x| x, 0.0..5000.0, 5_000_000));
    println!("{}", integral(|x| x, 0.0..6000.0, 6_000_000));
}
