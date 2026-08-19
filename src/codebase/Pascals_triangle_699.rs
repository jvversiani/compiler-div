// Rosetta Code task: Pascal's triangle
// Source: https://rosettacode.org/wiki/Pascal's_triangle#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
//                    1 
//                  1  1 
//                1  2  1 
//              1  3  3  1 
//            1  4  6  4  1 
//          1  5 10 10  5  1 
//        1  6 15 20 15  6  1 
//      1  7 21 35 35 21  7  1 
//    1  8 28 56 70 56 28  8  1 
//  1  9 36 84 126 126 84 36  9  1
// =======================

fn pascal_triangle(n: u64)
{

  for i in 0..n {
    let mut c = 1;
    for _j in 1..2*(n-1-i)+1 {
      print!(" ");
    }
    for k in 0..i+1 {
      print!("{:2} ", c);
      c = c * (i-k)/(k+1);
    }
    println!();
  }
}

fn main(){
  pascal_triangle(10);
}
