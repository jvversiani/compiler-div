// Rosetta Code task: AKS test for primes
// Source: https://rosettacode.org/wiki/AKS_test_for_primes#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 0: [1]
// 1: [1, -1]
// 2: [1, -2, 1]
// 3: [1, -3, 3, -1]
// 4: [1, -4, 6, -4, 1]
// 5: [1, -5, 10, -10, 5, -1]
// 6: [1, -6, 15, -20, 15, -6, 1]
// 7: [1, -7, 21, -35, 35, -21, 7, -1]
// 2 3 5 7 11 13 17 19 23 29 31 37 41 43 47
// =======================

fn aks_coefficients(k: usize) -> Vec<i64> {
    let mut coefficients = vec![0i64; k + 1];
    coefficients[0] = 1;
    for i in 1..(k + 1) {
        coefficients[i] = -(1..i).fold(coefficients[0], |prev, j|{
            let old = coefficients[j];
            coefficients[j] = old - prev;
            old
        });
    }
    coefficients
}

fn is_prime(p: usize) -> bool {
    if p < 2 {
        false
    } else {
        let c = aks_coefficients(p);
        (1..p / 2 + 1).all(|i| c[i] % p as i64 == 0)
    }
}

fn main() {
    for i in 0..8 {
        println!("{}: {:?}", i, aks_coefficients(i));
    }
    for i in (1..=50).filter(|&i| is_prime(i)) {
        print!("{} ", i);
    }
}
