// Rosetta Code task: Safe primes and unsafe primes
// Source: https://rosettacode.org/wiki/Safe_primes_and_unsafe_primes#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// first 35 safe primes: 5 7 11 23 47 59 83 107 167 179 227 263 347 359 383 467 479 503 563 587 719 839 863 887 983 1019 1187 1283 1307 1319 1367 1439 1487 1523 1619
// first 35 unsafe primes: 2 3 13 17 19 29 31 37 41 43 53 61 67 71 73 79 89 97 101 103 109 113 127 131 137 139 149 151 157 163 173 181 191 193 197
// safe primes below 1,000,000: 4324
// unsafe primes below 1,000,000: 74174
// safe primes below 10,000,000: 30657
// unsafe primes below 10,000,000: 633922
// =======================

fn is_prime(n: i32) -> bool {
	for i in 2..n {
		if i * i > n {
			return true;
		}
		if n % i == 0 {
			return false;
		}
	}
	n > 1
}

fn is_safe_prime(n: i32) -> bool {
	is_prime(n) && is_prime((n - 1) / 2)
}

fn is_unsafe_prime(n: i32) -> bool {
	is_prime(n) && !is_prime((n - 1) / 2)
}

fn next_prime(n: i32) -> i32 {
	for i in (n+1).. {
		if is_prime(i) {
			return i;
		}
	}
	0
}

fn main() {
	let mut safe = 0;
	let mut unsf = 0;
	let mut p = 2;

	print!("first 35 safe primes: ");
	while safe < 35 {
		if is_safe_prime(p) {
			safe += 1;
			print!("{} ", p);
		}
		p = next_prime(p);
	}
	println!("");

	p = 2;

	print!("first 35 unsafe primes: ");
	while unsf < 35 {
		if is_unsafe_prime(p) {
			unsf += 1;
			print!("{} ", p);
		}
		p = next_prime(p);
	}
	println!("");

	p = 2;
	safe = 0;
	unsf = 0;

	while p < 1000000 {
		if is_safe_prime(p) {
			safe += 1;
		} else {
			unsf += 1;
		}
		p = next_prime(p);
	}
	println!("safe primes below 1,000,000: {}", safe);
	println!("unsafe primes below 1,000,000: {}", unsf);

	while p < 10000000 {
		if is_safe_prime(p) {
			safe += 1;
		} else {
			unsf += 1;
		}
		p = next_prime(p);
	}
	println!("safe primes below 10,000,000: {}", safe);
	println!("unsafe primes below 10,000,000: {}", unsf);
}
