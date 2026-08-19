// Rosetta Code task: Humble numbers
// Source: https://rosettacode.org/wiki/Humble_numbers#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 14, 15, 16, 18, 20, 21, 24, 25, 27, 28, 30, 32, 35, 36, 40, 42, 45, 48, 49, 50, 54, 56, 60, 63, 64, 70, 72, 75, 80, 81, 84, 90, 96, 98, 100, 105, 108, 112, 120]
// length 0 with 0 humble numbers!
// length 1 with 9 humble numbers!
// length 2 with 36 humble numbers!
// length 3 with 95 humble numbers!
// =======================

fn num_length( number : u32 ) -> usize {
   let numberstring : String = number.to_string( ) ;
   let numstr = numberstring.as_str( ) ;
   numstr.chars( ).count( ) 
}

fn is_humble( number : u32 ) -> bool {
   let result : bool = match number {
      num if num <= 1 => true , 
      num if num % 2 == 0 => is_humble( number / 2 ) ,
      num if num % 3 == 0 => is_humble( number / 3 ) ,
      num if num % 4 == 0 => is_humble( number / 4 ) , 
      num if num % 5 == 0 => is_humble( number / 5 ) ,
      num if num % 6 == 0 => is_humble( number / 6 ) ,
      num if num % 7 == 0 => is_humble( number / 7 ) ,
      _                   => false
   } ;
   result
}

fn main() {
   let mut first_fifty : Vec<u32> = Vec::new( ) ;
   let mut pairs : Vec<(usize , usize)> = Vec::new( ) ;
   let mut oldlength : usize = 0 ;
   let mut currentnum : u32 = 1 ;
   let mut length : usize = 1 ;
   let mut lengthsum : usize = 0 ;
   while length < 4 {
      if is_humble( currentnum ) {
         if first_fifty.len( ) < 50 {
            first_fifty.push( currentnum ) ;
         }
         if length == oldlength {
            lengthsum += 1 ;
         }
         else {
            let temp_len : usize = length - 1 ;
            pairs.push( (temp_len , lengthsum ) ) ;
            lengthsum = 1 ;
            oldlength = length ;
         }
      }
      currentnum += 1 ;
      length = num_length( currentnum ) ;
   }
   pairs.push((length - 1 , lengthsum )) ;
   println!("{:?}" , first_fifty ) ;
   for p in pairs {
      println!("length {} with {} humble numbers!" , p.0 , p.1 ) ;
   }
}
