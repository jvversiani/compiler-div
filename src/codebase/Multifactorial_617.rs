// Rosetta Code task: Multifactorial
// Source: https://rosettacode.org/wiki/Multifactorial#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 1 2 6 24 120 720 5040 40320 362880 3628800
// 1 2 3 8 15 48 105 384 945 3840
// 1 2 3 4 10 18 28 80 162 280
// 1 2 3 4 5 12 21 32 45 120
// 1 2 3 4 5 6 14 24 36 50
// =======================

fn multifactorial(n: i32, deg: i32) -> i32 {
	if n < 1 {
		1
	} else {
		n * multifactorial(n - deg, deg)
	}
}

fn main() {
	for i in 1..6 {
		for j in 1..11 {
			print!("{} ", multifactorial(j, i));
		}
	println!("");
	}
}
