// Rosetta Code task: Binomial transform
// Source: https://rosettacode.org/wiki/Binomial_transform#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Catalan number sequence:
// 1 1 2 5 14 42 132 429 1430 4862 16796 58786 208012 742900 2674440 9694845 
// Forward binomial transform:
// 1 2 5 15 51 188 731 2950 12235 51822 223191 974427 4302645 19181100 86211885 390248055 
// Inverse binomial transform:
// 1 0 1 1 3 6 15 36 91 232 603 1585 4213 11298 30537 83097 
// Round trip:
// 1 1 2 5 14 42 132 429 1430 4862 16796 58786 208012 742900 2674440 9694845 
// Self-inverting:
// 1 0 1 -1 3 -6 15 -36 91 -232 603 -1585 4213 -11298 30537 -83097 
// Round trip self-inverting:
// 1 1 2 5 14 42 132 429 1430 4862 16796 58786 208012 742900 2674440 9694845 
//
//
// Prime flip-flop sequence:
// 0 1 1 0 1 0 1 0 0 0 1 0 1 0 0 0 1 0 1 0 
// Forward binomial transform:
// 0 1 3 6 11 20 37 70 134 255 476 869 1564 2821 5201 9948 19793 40562 84271 174952 
// Inverse binomial transform:
// 0 1 -1 0 3 -10 25 -56 118 -237 456 -847 1540 -2795 5173 -9918 19761 -40528 84235 -174914 
// Round trip:
// 0 1 1 0 1 0 1 0 0 0 1 0 1 0 0 0 1 0 1 0 
// Self-inverting:
// 0 -1 -1 0 3 10 25 56 118 237 456 847 1540 2795 5173 9918 19761 40528 84235 174914 
// Round trip self-inverting:
// 0 1 1 0 1 0 1 0 0 0 1 0 1 0 0 0 1 0 1 0 
//
//
// Fibonacci number sequence:
// 0 1 1 2 3 5 8 13 21 34 55 89 144 233 377 610 987 1597 2584 4181 
// Forward binomial transform:
// 0 1 3 8 21 55 144 377 987 2584 6765 17711 46368 121393 317811 832040 2178309 5702887 14930352 39088169 
// Inverse binomial transform:
// 0 1 -1 2 -3 5 -8 13 -21 34 -55 89 -144 233 -377 610 -987 1597 -2584 4181 
// Round trip:
// 0 1 1 2 3 5 8 13 21 34 55 89 144 233 377 610 987 1597 2584 4181 
// Self-inverting:
// 0 -1 -1 -2 -3 -5 -8 -13 -21 -34 -55 -89 -144 -233 -377 -610 -987 -1597 -2584 -4181 
// Round trip self-inverting:
// 0 1 1 2 3 5 8 13 21 34 55 89 144 233 377 610 987 1597 2584 4181 
//
//
// Padovan number sequence:
// 1 0 0 1 0 1 1 1 2 2 3 4 5 7 9 12 16 21 28 37 
// Forward binomial transform:
// 1 1 1 2 5 12 28 65 151 351 816 1897 4410 10252 23833 55405 128801 299426 696081 1618192 
// Inverse binomial transform:
// 1 -1 1 0 -3 10 -24 49 -89 145 -208 245 -174 -176 1121 -3185 7137 -13920 24301 -37926 
// Round trip:
// 1 0 0 1 0 1 1 1 2 2 3 4 5 7 9 12 16 21 28 37 
// Self-inverting:
// 1 1 1 0 -3 -10 -24 -49 -89 -145 -208 -245 -174 176 1121 3185 7137 13920 24301 37926 
// Round trip self-inverting:
// 1 0 0 1 0 1 1 1 2 2 3 4 5 7 9 12 16 21 28 37
// =======================

use std::convert::TryInto;
use std::fmt;

fn print_vector<T: fmt::Display>(vec: &Vec<T>) {
    for element in vec {
        print!("{} ", element);
    }
}

fn factorial(number: u32) -> Result<u64, String> {
    if number > 20 {
        return Err(format!("Too large for 64 bit number: {}", number));
    }
    if number < 2 {
        return Ok(1);
    }

    let mut factorial: u64 = 1;
    for i in 2..=number {
        factorial *= u64::from(i);
    }
    Ok(factorial)
}

fn binomial(n: u32, k: u32) -> Result<u64, String> {
    let n_fact = factorial(n)?;
    let n_minus_k_fact = factorial(n - k)?;
    let k_fact = factorial(k)?;

    Ok(n_fact / n_minus_k_fact / k_fact)
}

fn forward(vec: &Vec<i64>) -> Result<Vec<i64>, String> {
    let size = vec.len();
    let mut transform: Vec<i64> = vec![0; size];

    for n in 0..size {
        for k in 0..=n {
            let binomial_coeff = binomial(n.try_into().unwrap(), k.try_into().unwrap())?;
            transform[n] += (binomial_coeff as i64) * vec[k];
        }
    }

    Ok(transform)
}

fn inverse(vec: &Vec<i64>) -> Result<Vec<i64>, String> {
    let size = vec.len();
    let mut transform: Vec<i64> = vec![0; size];

    for n in 0..size {
        for k in 0..=n {
            let binomial_coeff = binomial(n.try_into().unwrap(), k.try_into().unwrap())?;
            let sign: i32 = if (n - k) % 2 == 1 { -1 } else { 1 };
            transform[n] += (binomial_coeff as i64) * vec[k] * (sign as i64);
        }
    }

    Ok(transform)
}

fn self_inverting(vec: &Vec<i64>) -> Result<Vec<i64>, String> {
    let size = vec.len();
    let mut transform: Vec<i64> = vec![0; size];

    for n in 0..size {
        for k in 0..=n {
            let binomial_coeff = binomial(n.try_into().unwrap(), k.try_into().unwrap())?;
            let sign: i32 = if k % 2 == 1 { -1 } else { 1 };
            transform[n] += (binomial_coeff as i64) * vec[k] * (sign as i64);
        }
    }

    Ok(transform)
}

fn main() -> Result<(), String> {
    let sequences: Vec<Vec<i64>> = vec![
        vec![
            1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862, 16796, 58786, 208012, 742900, 2674440,
            9694845,
        ],
        vec![0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0],
        vec![
            0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181,
        ],
        vec![
            1, 0, 0, 1, 0, 1, 1, 1, 2, 2, 3, 4, 5, 7, 9, 12, 16, 21, 28, 37,
        ],
    ];

    let names: Vec<String> = vec![
        "Catalan number sequence:".to_string(),
        "Prime flip-flop sequence:".to_string(),
        "Fibonacci number sequence:".to_string(),
        "Padovan number sequence:".to_string(),
    ];

    for i in 0..sequences.len() {
        println!("{}", names[i]);
        print_vector(&sequences[i]);
        println!("\nForward binomial transform:");
        print_vector(&forward(&sequences[i])?);
        println!("\nInverse binomial transform:");
        print_vector(&inverse(&sequences[i])?);
        println!("\nRound trip:");
        print_vector(&inverse(&forward(&sequences[i])?)?);
        println!("\nSelf-inverting:");
        print_vector(&self_inverting(&sequences[i])?);
        println!("\nRound trip self-inverting:");
        print_vector(&self_inverting(&self_inverting(&sequences[i])?)?);
        println!("\n\n");
    }

    Ok(())
}
