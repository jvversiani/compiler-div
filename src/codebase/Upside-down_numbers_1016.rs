// Rosetta Code task: Upside-down numbers
// Source: https://rosettacode.org/wiki/Upside-down_numbers#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// The first 50 upside-down numbers:
// [5, 19, 28, 37, 46, 55, 64, 73, 82, 91, 159, 258, 357, 456, 555, 654, 753, 852, 951, 1199, 1289, 1379, 1469, 1559, 1649, 1739, 1829, 1919, 2198, 2288, 2378, 2468, 2558, 2648, 2738, 2828, 2918, 3197, 3287, 3377, 3467, 3557, 3647, 3737, 3827, 3917, 4196, 4286, 4376, 4466]
// The five hundredth such number : 494616
// The five thousandth such number : 56546545
// =======================

fn is_upside_down( num : u32 ) -> bool {
   let numberstring : String = num.to_string( ) ;
   let len = numberstring.len( ) ;
   let numberstr = numberstring.as_str( ) ;
   if numberstr.contains( "0" ) {
      return false ;
   }
   if len % 2 == 1 && numberstr.chars( ).nth( len / 2 ).unwrap( ) != '5' {
      return false ;
   }
   let mut forward : usize = 0 ;
   let mut backward : usize = len - 1 ;
   while forward <= backward {
      let first = numberstr.chars( ).nth( forward ).expect("No digit!").
         to_digit( 10 ).unwrap( ) ;
      let second = numberstr.chars( ).nth( backward ).expect("No digit!").
         to_digit( 10 ).unwrap( ) ;
      if first + second != 10 {
         return false ;
      }
      forward += 1 ;
      if backward != 0 {
         backward -= 1 ;
       }
   }
   true 
}

fn main() {
    let mut solution : Vec<u32> = Vec::new( ) ;
    let mut sum : u32 = 0 ;
    let mut current : u32 = 0 ;
    while sum < 50 {
       current += 1 ;
       if is_upside_down( current ) {
          solution.push( current ) ;
          sum += 1 ;
       }
    }
    let five_hundr : u32 ;
    while sum != 500 {
       current += 1 ;
       if is_upside_down( current ) {
          sum += 1 ;
       }
    }
    five_hundr = current ;
    let five_thous : u32 ;
    while sum != 5000 {
       current += 1 ;
       if is_upside_down( current ) {
          sum += 1 ;
       }
    }
    five_thous = current ;
    println!("The first 50 upside-down numbers:") ;
    println!("{:?}" , solution ) ;
    println!("The five hundredth such number : {}" , five_hundr) ;
    println!("The five thousandth such number : {}" , five_thous ) ;
}
