// Rosetta Code task: Numbers divisible by their individual digits, but not by the product of their digits
// Source: https://rosettacode.org/wiki/Numbers_divisible_by_their_individual_digits,_but_not_by_the_product_of_their_digits#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
//    22   33   44   48   55   66   77   88   99  122
//   124  126  155  162  168  184  222  244  248  264
//   288  324  333  336  366  396  412  424  444  448
//   488  515  555  636  648  666  728  777  784  824
//   848  864  888  936  999
// =======================

fn to_digits( n : i32 ) -> Vec<i32> {
   let mut i : i32 = n ;
   let mut digits : Vec<i32> = Vec::new( ) ;
   while i != 0 {
      digits.push( i % 10 ) ;
      i /= 10 ;
   }
   digits
}

fn my_condition( num : i32 ) -> bool {
   let digits : Vec<i32> = to_digits( num ) ;
   if ! digits.iter( ).any( | x | *x == 0 ) {
      let prod : i32 = digits.iter( ).product( ) ;
      return digits.iter( ).all( | x | num % x == 0 ) && 
         num % prod != 0 ;
   }
   else {
      false 
   }
}

fn main() {
let mut count : i32 = 0 ;
   for n in 10 .. 1000 {
      if my_condition( n ) {
         print!("{:5}" , n) ;
         count += 1 ;
         if count % 10 == 0 {
            println!( ) ;
         }
      }
   }
   println!();
}
