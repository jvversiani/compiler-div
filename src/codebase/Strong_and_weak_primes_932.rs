// Rosetta Code task: Strong and weak primes
// Source: https://rosettacode.org/wiki/Strong_and_weak_primes#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// First 36 strong primes: 11 17 29 37 41 59 67 71 79 97 101 107 127 137 149 163 179 191 197 223 227 239 251 269 277 281 307 311 331 347 367 379 397 419 431 439
// strong primes below 1,000,000: 37723
// strong primes below 10,000,000: 320991
// First 36 weak primes: 7 13 19 23 31 43 47 61 73 83 89 103 109 113 131 139 151 167 181 193 199 229 233 241 271 283 293 313 317 337 349 353 359 383 389 401
// weak primes below 1,000,000: 37779
// weak primes below 10,000,000: 321749
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

fn next_prime(n: i32) -> i32 {
	for i in (n+1).. {
		if is_prime(i) {
			return i;
		}
	}
	0
}

fn main() {
	let mut n = 0;
	let mut prime_q = 5;
	let mut prime_p = 3;
	let mut prime_o = 2;

	print!("First 36 strong primes: ");
	while n < 36 {
		if prime_p > (prime_o + prime_q) / 2 {
			print!("{} ",prime_p);
			n += 1;
		}
		prime_o = prime_p;
		prime_p = prime_q;
		prime_q = next_prime(prime_q);
	}
	println!("");

	while prime_p < 1000000 {
		if prime_p > (prime_o + prime_q) / 2 {
			n += 1;
		}
		prime_o = prime_p;
		prime_p = prime_q;
		prime_q = next_prime(prime_q);
	}
	println!("strong primes below 1,000,000: {}", n);

	while prime_p < 10000000 {
		if prime_p > (prime_o + prime_q) / 2 {
			n += 1;
		}
		prime_o = prime_p;
		prime_p = prime_q;
		prime_q = next_prime(prime_q);
	}
	println!("strong primes below 10,000,000: {}", n);

	n = 0;
	prime_q = 5;
	prime_p = 3;
	prime_o = 2;

	print!("First 36 weak primes: ");
	while n < 36 {
		if prime_p < (prime_o + prime_q) / 2 {
			print!("{} ",prime_p);
			n += 1;
		}
		prime_o = prime_p;
		prime_p = prime_q;
		prime_q = next_prime(prime_q);
	}
	println!("");

	while prime_p < 1000000 {
		if prime_p < (prime_o + prime_q) / 2 {
			n += 1;
		}
		prime_o = prime_p;
		prime_p = prime_q;
		prime_q = next_prime(prime_q);
	}
	println!("weak primes below 1,000,000: {}", n);

	while prime_p < 10000000 {
		if prime_p < (prime_o + prime_q) / 2 {
			n += 1;
		}
		prime_o = prime_p;
		prime_p = prime_q;
		prime_q = next_prime(prime_q);
	}
	println!("weak primes below 10,000,000: {}", n);
}
