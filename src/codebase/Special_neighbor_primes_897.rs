// Rosetta Code task: Special neighbor primes
// Source: https://rosettacode.org/wiki/Special_neighbor_primes#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// (3 , 5)
// (5 , 7)
// (7 , 11)
// (11 , 13)
// (13 , 17)
// (19 , 23)
// (29 , 31)
// (31 , 37)
// (41 , 43)
// (43 , 47)
// (61 , 67)
// (67 , 71)
// (73 , 79)
// =======================

fn is_prime( number : u16 ) -> bool {
   let limit : u16 = (number as f32).sqrt( ).floor( ) as u16 ;
   (2..=limit).all( | i | number % i != 0 ) 
}

fn main() {
   let primes : Vec<u16> = (2..100).filter( | &d | is_prime( d ) ).collect( ) ;
   let prime_slice = &primes[..] ;
   let mut iter = prime_slice.windows( 2 ) ;
   while let Some( p ) = iter.next( ) {
      if is_prime( p[0] + p[1] - 1 ) {
         println!("({} , {})" , p[0] , p[1] );
      }
   }
}
