// Rosetta Code task: Tau function
// Source: https://rosettacode.org/wiki/Tau_function#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 1 2 2 3 2 4 2 4 3 4 2 6 2 4 4 5 2 6 2 6 4 4 2 8 3 4 4 6 2 8 2 6 4 4 4 9 2 4 4 8 2 8 2 6 6 4 2 10 3 6 4 6 2 8 4 8 4 4 2 12 2 4 6 7 4 8 2 6 4 8 2 12 2 4 6 6 4 8 2 10 5 4 2 12 4 4 4 8 2 12 4 6 4 4 4 12 2 6 6 9
// =======================

// returns the highest power of i that is a factor of n,
// and n divided by that power of i
fn factor_exponent(n: i32, i: i32) -> (i32, i32) {
	if n % i == 0 {
		let (a, b) = factor_exponent(n / i, i);
		(a + 1, b)
	} else {
		(0, n)
	}
}

fn tau(n: i32) -> i32 {
	for i in 2..(n+1) {
		if n % i == 0 {
			let (count, next) = factor_exponent(n, i);
			return (count + 1) * tau(next);
		}
	}
	return 1;
}

fn main() {
	for i in 1..101 {
		print!("{} ", tau(i));
	}
}
