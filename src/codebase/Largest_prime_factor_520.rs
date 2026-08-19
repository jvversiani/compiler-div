// Rosetta Code task: Largest prime factor
// Source: https://rosettacode.org/wiki/Largest_prime_factor#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 6857
// =======================

fn main( ) {
   let mut current : i64 = 600851475143 ;
   let mut latest_divisor : i64 = 2 ;
   while current != 1 {
      latest_divisor = 2 ;
      while current % latest_divisor != 0 {
         latest_divisor += 1 ;
      }
      current /= latest_divisor ;
   }
   println!("{}" , latest_divisor ) ;
}
