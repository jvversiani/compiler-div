// Rosetta Code task: Sum of the digits of n is substring of n
// Source: https://rosettacode.org/wiki/Sum_of_the_digits_of_n_is_substring_of_n#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 109, 119, 129, 139, 149, 159, 169, 179, 189, 199, 200, 300, 400, 500, 600, 700, 800, 900, 910, 911, 912, 913, 914, 915, 916, 917, 918, 919]
// =======================

fn sum_digits( mut num : u32 ) -> u32 {
   let mut sum : u32 = 0 ;
   while num != 0 {
      sum += num % 10 ;
      num /= 10 ;
   }
   sum 
}

fn main() {
    let solution : Vec<u32> = (0..1000).filter( | &d | {
          let digit_sum : u32 = sum_digits( d ) ;
          let sumstring = digit_sum.to_string( ) ;
          let sumstr : &str = sumstring.as_str( ) ;
          let numstring : String = d.to_string( ) ;
          let numstr : &str = numstring.as_str( ) ;
          numstr.contains( &sumstr )
          }).collect( ) ;
    println!("{:?}" , solution ) ;
}
