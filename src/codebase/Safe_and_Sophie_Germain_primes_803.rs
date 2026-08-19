// Rosetta Code task: Safe and Sophie Germain primes
// Source: https://rosettacode.org/wiki/Safe_and_Sophie_Germain_primes#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// The first 50 Sophie - Germain primes:
//     2    3    5   11   23   29   41   53   83   89
//   113  131  173  179  191  233  239  251  281  293
//   359  419  431  443  491  509  593  641  653  659
//   683  719  743  761  809  911  953 1013 1019 1031
//  1049 1103 1223 1229 1289 1409 1439 1451 1481 1499
// =======================

fn is_prime( num : u32 ) -> bool {
   let limit : u32 = (num as f32).sqrt( ).floor( ) as u32 ;
   (2..=limit).all( | x | num % x != 0 )
}

fn main() {
   let mut sophie_primes : Vec<u32> = Vec::new( ) ;
   let mut count : u8 = 0 ;
   let mut current : u32 = 2 ;
   while count < 50 {
      if is_prime( current ) && is_prime( 2 * current + 1 ) {
         sophie_primes.push( current ) ;
         count += 1 ;
      }
      current += 1 ;
   }
   println!("The first 50 Sophie - Germain primes:" ) ;
   let mut ct : u8 = 0 ;
   for num in sophie_primes {
      print!("{:>5}" , num ) ;
      ct += 1 ;
      if  ct % 10 == 0  {
        println!("") ;
        ct = 0 ;
      }
   }
}
