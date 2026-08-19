// Rosetta Code task: N-smooth numbers
// Source: https://rosettacode.org/wiki/N-smooth_numbers#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// First 25 n-smooth numbers for n = 2 -> 29:
// 2: 1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536, 131072, 262144, 524288, 1048576, 2097152, 4194304, 8388608, 16777216
// 3: 1, 2, 3, 4, 6, 8, 9, 12, 16, 18, 24, 27, 32, 36, 48, 54, 64, 72, 81, 96, 108, 128, 144, 162, 192
// 5: 1, 2, 3, 4, 5, 6, 8, 9, 10, 12, 15, 16, 18, 20, 24, 25, 27, 30, 32, 36, 40, 45, 48, 50, 54
// 7: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 14, 15, 16, 18, 20, 21, 24, 25, 27, 28, 30, 32, 35, 36
// 11: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 14, 15, 16, 18, 20, 21, 22, 24, 25, 27, 28, 30, 32
// 13: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 18, 20, 21, 22, 24, 25, 26, 27, 28
// 17: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 20, 21, 22, 24, 25, 26, 27
// 19: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 24, 25, 26
// 23: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25
// 29: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25
//
// 3 n-smooth numbers starting from 3000th for n = 3 -> 29:
// 3: 91580367978306252441724649472, 92829823186414819915547541504, 94096325042746502515294076928
// 5: 278942752080, 279936000000, 281250000000
// 7: 50176000, 50331648, 50388480
// 11: 2112880, 2116800, 2117016
// 13: 390000, 390390, 390625
// 17: 145800, 145860, 146016
// 19: 74256, 74358, 74360
// 23: 46552, 46575, 46585
// 29: 33516, 33524, 33534
//
// 20 n-smooth numbers starting from 30,000th for n = 503 -> 521:
// 503: 62913, 62914, 62916, 62918, 62920, 62923, 62926, 62928, 62930, 62933, 62935, 62937, 62944, 62946, 62951, 62952, 62953, 62957, 62959, 62964
// 509: 62601, 62602, 62604, 62607, 62608, 62609, 62611, 62618, 62620, 62622, 62624, 62625, 62626, 62628, 62629, 62634, 62640, 62643, 62645, 62646
// 521: 62287, 62288, 62291, 62292, 62300, 62304, 62307, 62308, 62310, 62315, 62320, 62321, 62322, 62325, 62328, 62329, 62330, 62331, 62335, 62336
// =======================

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n % 2 == 0 {
        return n == 2;
    }
    if n % 3 == 0 {
        return n == 3;
    }
    let mut p = 5;
    while p * p <= n {
        if n % p == 0 {
            return false;
        }
        p += 2;
        if n % p == 0 {
            return false;
        }
        p += 4;
    }
    true
}

fn find_primes(from: u32, to: u32) -> Vec<u32> {
    let mut primes: Vec<u32> = Vec::new();
    for p in from..=to {
        if is_prime(p) {
            primes.push(p);
        }
    }
    primes
}

fn find_nsmooth_numbers(n: u32, count: usize) -> Vec<u128> {
    let primes = find_primes(2, n);
    let num_primes = primes.len();
    let mut result = Vec::with_capacity(count);
    let mut queue = Vec::with_capacity(num_primes);
    let mut index = Vec::with_capacity(num_primes);
    for i in 0..num_primes {
        index.push(0);
        queue.push(primes[i] as u128);
    }
    result.push(1);
    for i in 1..count {
        for p in 0..num_primes {
            if queue[p] == result[i - 1] {
                index[p] += 1;
                queue[p] = result[index[p]] * primes[p] as u128;
            }
        }
        let mut min_index: usize = 0;
        for p in 1..num_primes {
            if queue[min_index] > queue[p] {
                min_index = p;
            }
        }
        result.push(queue[min_index]);
    }
    result
}

fn print_nsmooth_numbers(n: u32, begin: usize, count: usize) {
    let numbers = find_nsmooth_numbers(n, begin + count);
    print!("{}: {}", n, &numbers[begin]);
    for i in 1..count {
        print!(", {}", &numbers[begin + i]);
    }
    println!();
}

fn main() {
    println!("First 25 n-smooth numbers for n = 2 -> 29:");
    for n in 2..=29 {
        if is_prime(n) {
            print_nsmooth_numbers(n, 0, 25);
        }
    }
    println!();
    println!("3 n-smooth numbers starting from 3000th for n = 3 -> 29:");
    for n in 3..=29 {
        if is_prime(n) {
            print_nsmooth_numbers(n, 2999, 3);
        }
    }
    println!();
    println!("20 n-smooth numbers starting from 30,000th for n = 503 -> 521:");
    for n in 503..=521 {
        if is_prime(n) {
            print_nsmooth_numbers(n, 29999, 20);
        }
    }
}
