// Rosetta Code task: Nth root
// Source: https://rosettacode.org/wiki/Nth_root#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 2
// =======================

// 20210212 Rust programming solution

fn nthRoot(n: f64, A: f64) -> f64 {

   let      p  =  1e-9_f64 ;
   let mut x0  =     A / n ;

   loop {
      let mut x1 = ( (n-1.0) * x0 + A / f64::powf(x0, n-1.0) ) / n;
      if (x1-x0).abs() < (x0*p).abs() { return x1 };
      x0 = x1
   }
}

fn main() {
   println!("{}", nthRoot(3. , 8. ));
}
