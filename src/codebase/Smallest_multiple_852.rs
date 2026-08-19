// Rosetta Code task: Smallest multiple
// Source: https://rosettacode.org/wiki/Smallest_multiple#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 232792560
// =======================

fn main() {
   let mut current : u32 = 2520 ;
   while ! (1u32..=20u32).all( | i | current % i == 0 ) {
      current += 1 ;
   }
   println!("{}" , current) ;
}
