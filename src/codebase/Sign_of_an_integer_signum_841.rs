// Rosetta Code task: Sign of an integer (signum)
// Source: https://rosettacode.org/wiki/Sign_of_an_integer_(signum)#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
//   input      output
//    -10         -1
//     42         1
//     0          0
//     0          1
//     -0         -1
//    NaN        NaN
// =======================

fn main() {
    println!("{:^10} {:^10}", "input", "output");
    for i in [-10i32, 42, 0] {
        println!("{i:^10} {:^10}", i.signum());
    }
    for f in [0.0, -0.0, f64::from_bits(0xFFF8_0000_0000_0000)] {
        println!("{f:^10} {:^10}", f.signum());
    }
}
