// Rosetta Code task: Special divisors
// Source: https://rosettacode.org/wiki/Special_divisors#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// [1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 13, 17, 19, 22, 23, 26, 27, 29, 31, 33, 37, 39, 41, 43, 44, 46, 47, 53, 55, 59, 61, 62, 66, 67, 69, 71, 73, 77, 79, 82, 83, 86, 88, 89, 93, 97, 99, 101, 103, 107, 109, 113, 121, 127, 131, 137, 139, 143, 149, 151, 157, 163, 167, 169, 173, 179, 181, 187, 191, 193, 197, 199]
// =======================

fn condition( num : u16 ) -> bool {
   let divis : Vec<u16> = divisors( num ) ;
   let reversed : u16 = my_reverse( num ) ;
   divis.iter( ).all( | d | {
         let revi = my_reverse( *d ) ;
         reversed % revi == 0 } ) 
}

fn my_reverse( num : u16 ) -> u16 {
   let numstring : String = num.to_string( ) ;
   let nstr : &str = numstring.as_str( ) ;
   let mut reversed_str : String = String::new( ) ;
   for c in  nstr.chars( ).rev( ) {
      reversed_str.push( c ) ;
   }
   let reversi : &str = reversed_str.as_str( ) ;
   reversi.parse::<u16>( ).unwrap( )
}

fn divisors( n : u16 ) -> Vec<u16> {
   (1..=n).filter( | &d | n % d == 0 ).collect( ) 
}

fn main() {
    println!("{:?}" , (1u16..200u16).filter( | &d | condition( d ) ).collect
          ::<Vec<u16>>( ) )  ;
}
