// Rosetta Code task: Show the (decimal) value of a number of 1s appended with a 3, then squared
// Source: https://rosettacode.org/wiki/Show_the_(decimal)_value_of_a_number_of_1s_appended_with_a_3,_then_squared#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 3 ^ 2 =                      9
// 13 ^ 2 =                   169
// 113 ^ 2 =                12769
// 1113 ^ 2 =             1238769
// 11113 ^ 2 =          123498769
// 111113 ^ 2 =       12346098769
// 1111113 ^ 2 =    1234572098769
// 11111113 ^ 2 = 123456832098769
// =======================

fn main() {
   let mut big_squares : Vec<u64> = Vec::new( ) ;
   let mut numberstrings : Vec<String> = Vec::new( ) ;
   for n in 0..8 {
      let mut numberstring : String = String::new( ) ;
      for i in 0..=n {
         if  i != 0  {
            numberstring.push( '1' ) ;
         }
      }
      numberstring.push('3') ;
      let number : u64 = numberstring.parse::<u64>().unwrap( ) ;
      numberstrings.push( numberstring ) ;
      big_squares.push( number.pow( 2 )) ;
   }
   for i in 0..numberstrings.len( ) {
      print!("{} ^ 2 =" , numberstrings[ i ] ) ;
      let width = 30 - (7 + i )  ;
      println!("{:>width$}" , big_squares[ i ] ) ;
   }
}
