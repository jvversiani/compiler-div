// Rosetta Code task: Perfect numbers
// Source: https://rosettacode.org/wiki/Perfect_numbers#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 6 is a perfect number.
// 28 is a perfect number.
// 496 is a perfect number.
// 8128 is a perfect number.
// =======================

fn main ( ) {
	fn factor_sum(n: i32) -> i32 {
	    let mut v = Vec::new(); //create new empty array
	    for  x in 1..n-1 {      //test vaules 1 to n-1
	    	if n%x == 0 {   //if current x is a factor of n
	    		v.push(x);      //add x to the array
	    	}
	    }
    let mut sum = v.iter().sum(); //iterate over array and sum it up 
    return sum;
    }
    
    fn perfect_nums(n: i32) {
    	for x in 2..n {       //test numbers from 1-n
    		if factor_sum(x) == x {//call factor_sum on each value of x, if return value is = x
    			println!("{} is a perfect number.", x); //print value of x 
    		}
    	}
    }
    perfect_nums(10000);
}
