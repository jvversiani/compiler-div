// Rosetta Code task: Sieve of Eratosthenes
// Source: https://rosettacode.org/wiki/Sieve_of_Eratosthenes#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97]
// =======================

fn primes(n: usize) -> impl Iterator<Item = usize> {
    const START: usize = 2;
    if n < START {
        Vec::new()
    } else {
        let mut is_prime = vec![true; n + 1 - START];
        let limit = (n as f64).sqrt() as usize;
        for i in START..limit + 1 {
            let mut it = is_prime[i - START..].iter_mut().step_by(i);
            if let Some(true) = it.next() {
                it.for_each(|x| *x = false);
            }
        }
        is_prime
    }
    .into_iter()
    .enumerate()
    .filter_map(|(e, b)| if b { Some(e + START) } else { None })
}

fn main(){
    println!("{:?}", primes(100).collect::<Vec<usize>>());
}
