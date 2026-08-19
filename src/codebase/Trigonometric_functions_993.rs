// Rosetta Code task: Trigonometric functions
// Source: https://rosettacode.org/wiki/Trigonometric_functions#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 0.7071067811865475 0.7071067811865475
// 0.7071067811865476 0.7071067811865476
// 0.9999999999999999 0.9999999999999999
// 0.7853981633974482 44.99999999999999
// 0.7853981633974483 45
// 0.7853981633974483 45
// =======================

// 20210221 Rust programming solution

use std::f64::consts::PI;

fn main() {
   let angle_radians: f64 = PI/4.0;
   let angle_degrees: f64 = 45.0;

   println!("{} {}", angle_radians.sin(), angle_degrees.to_radians().sin());
   println!("{} {}", angle_radians.cos(), angle_degrees.to_radians().cos());
   println!("{} {}", angle_radians.tan(), angle_degrees.to_radians().tan());

   let asin = angle_radians.sin().asin();
   println!("{} {}", asin, asin.to_degrees());
   let acos = angle_radians.cos().acos();
   println!("{} {}", acos, acos.to_degrees());
   let atan = angle_radians.tan().atan();
   println!("{} {}", atan, atan.to_degrees());
}
