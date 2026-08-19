// Rosetta Code task: Euler method
// Source: https://rosettacode.org/wiki/Euler_method#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
//     Time:        0      10      20      30      40      50      60      70      80      90
// Analytic:  100.000  59.727  39.728  29.797  24.865  22.416  21.200  20.596  20.296  20.147  20.073
//  Step  2:  100.000  57.634  37.704  28.328  23.918  21.843  20.867  20.408  20.192  20.090  20.042
//  Step  5:  100.000  53.800  34.280  26.034  22.549  21.077  20.455  20.192  20.081  20.034  20.014
//  Step 10:  100.000  44.000  27.200  22.160  20.648  20.194  20.058  20.017  20.005  20.002  20.000
// =======================

fn header() {
    print!("    Time: ");
    for t in (0..100).step_by(10) {
        print!(" {:7}", t);
    }
    println!();
}

fn analytic() {
    print!("Analytic: ");
    for t in (0..=100).step_by(10) {
        print!(" {:7.3}", 20.0 + 80.0 * (-0.07 * f64::from(t)).exp());
    }
    println!();
}

fn euler<F: Fn(f64) -> f64>(f: F, mut y: f64, step: usize, end: usize) {
    print!(" Step {:2}: ", step);
    for t in (0..=end).step_by(step) {
        if t % 10 == 0 {
            print!(" {:7.3}", y);
        }
        y += step as f64 * f(y);
    }
    println!();
}

fn main() {
    header();
    analytic();
    for &i in &[2, 5, 10] {
        euler(|temp| -0.07 * (temp - 20.0), 100.0, i, 100);
    }
}
