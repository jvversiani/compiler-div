// Rosetta Code task: Van der Corput sequence
// Source: https://rosettacode.org/wiki/Van_der_Corput_sequence#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Base 2:  0.500000  0.250000  0.750000  0.125000  0.625000  0.375000  0.875000  0.062500  0.562500  0.312500
// Base 3:  0.333333  0.666667  0.111111  0.444444  0.777778  0.222222  0.555556  0.888889  0.037037  0.370370
// Base 4:  0.250000  0.500000  0.750000  0.062500  0.312500  0.562500  0.812500  0.125000  0.375000  0.625000
// Base 5:  0.200000  0.400000  0.600000  0.800000  0.040000  0.240000  0.440000  0.640000  0.840000  0.080000
// =======================

/// Van der Corput sequence for any base, based on C languange example from Wikipedia.
pub fn corput(nth: usize, base: usize) -> f64 {
    let mut n = nth;
    let mut q: f64 = 0.0;
    let mut bk: f64 = 1.0 / (base as f64);

    while n > 0_usize {
      q += ((n % base) as f64)*bk;
      n /= base;
      bk /= base as f64;
    }
    q
}

fn main() {
  for base in 2_usize..=5_usize {
    print!("Base {}:", base);
    for i in 1_usize..=10_usize {
      let c = corput(i, base);
      print!("  {:.6}", c)
    }
    println!("");
  }
}
