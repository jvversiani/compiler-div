// Rosetta Code task: Cyclops numbers
// Source: https://rosettacode.org/wiki/Cyclops_numbers#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// First 50 cyclops numbers:
//        0        101        102        103        104        105        106        107        108        109   
//      201        202        203        204        205        206        207        208        209        301   
//      302        303        304        305        306        307        308        309        401        402   
//      403        404        405        406        407        408        409        501        502        503   
//      504        505        506        507        508        509        601        602        603        604   
//
// First 50 prime cyclops numbers:
//      101        103        107        109        307        401        409        503        509        601   
//      607        701        709        809        907      11027      11047      11057      11059      11069   
//    11071      11083      11087      11093      12011      12037      12041      12043      12049      12071   
//    12073      12097      13033      13037      13043      13049      13063      13093      13099      14011   
//    14029      14033      14051      14057      14071      14081      14083      14087      15013      15017   
//
// First 50 blind prime cyclops numbers:
//      101        103        107        109        307        401        503        509        601        607   
//      701        709        809        907      11071      11087      11093      12037      12049      12097   
//    13099      14029      14033      14051      14071      14081      14083      14087      15031      15053   
//    15083      16057      16063      16067      16069      16097      17021      17033      17041      17047   
//    17053      17077      18047      18061      18077      18089      19013      19031      19051      19073   
//
// First 50 palindromic prime cyclops numbers:
//      101      16061      31013      35053      38083      73037      74047      91019      94049    1120211   
//  1150511    1160611    1180811    1190911    1250521    1280821    1360631    1390931    1490941    1520251   
//  1550551    1580851    1630361    1640461    1660661    1670761    1730371    1820281    1880881    1930391   
//  1970791    3140413    3160613    3260623    3310133    3380833    3460643    3470743    3590953    3670763   
//  3680863    3970793    7190917    7250527    7310137    7540457    7630367    7690967    7750577    7820287
// =======================

fn main() {
    let first50 = first_cyclops(50);
    println!("First 50 cyclops numbers:");
    print_vector(&first50, 10);
    
    let prime50 = first_cyclops_primes(50);
    println!("\nFirst 50 prime cyclops numbers:");
    print_vector(&prime50, 10);
    
    let blind50 = first_blind_cyclops_primes(50);
    println!("\nFirst 50 blind prime cyclops numbers:");
    print_vector(&blind50, 10);
    
    let palindrome50 = first_palindrome_cyclops_primes(50);
    println!("\nFirst 50 palindromic prime cyclops numbers:");
    print_vector(&palindrome50, 10);
}

fn print_vector(v: &[i32], nc: usize) {
    let mut col = 0;
    for e in v {
        print!("{:8}   ", e);
        col += 1;
        if col == nc {
            println!();
            col = 0;
        }
    }
}

fn is_cyclops_number(n: i32) -> bool {
    if n == 0 {
        return true;
    }
    
    let mut num = n;
    let mut m = num % 10;
    let mut count = 0;
    
    // Count digits before the zero
    while m != 0 {
        count += 1;
        num /= 10;
        m = num % 10;
    }
    
    // Skip the zero
    num /= 10;
    m = num % 10;
    
    // Count digits after the zero
    while m != 0 {
        count -= 1;
        num /= 10;
        m = num % 10;
    }
    
    num == 0 && count == 0
}

fn first_cyclops(n: usize) -> Vec<i32> {
    let mut result = Vec::new();
    let mut i = 0;
    
    while result.len() < n {
        if is_cyclops_number(i) {
            result.push(i);
        }
        i += 1;
    }
    
    result
}

fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    
    let sqrt_n = (n as f64).sqrt() as i32;
    for i in 2..=sqrt_n {
        if n % i == 0 {
            return false;
        }
    }
    
    true
}

fn first_cyclops_primes(n: usize) -> Vec<i32> {
    let mut result = Vec::new();
    let mut i = 0;
    
    while result.len() < n {
        if is_cyclops_number(i) && is_prime(i) {
            result.push(i);
        }
        i += 1;
    }
    
    result
}

fn blind_cyclops(n: i32) -> i32 {
    let mut num = n;
    let mut m = num % 10;
    let mut k = 0;
    
    // Extract digits before the zero
    while m != 0 {
        k = 10 * k + m;
        num /= 10;
        m = num % 10;
    }
    
    // Skip the zero
    num /= 10;
    
    // Reconstruct the number by reversing the first part
    while k != 0 {
        m = k % 10;
        num = 10 * num + m;
        k /= 10;
    }
    
    num
}

fn first_blind_cyclops_primes(n: usize) -> Vec<i32> {
    let mut result = Vec::new();
    let mut i = 0;
    
    while result.len() < n {
        if is_cyclops_number(i) && is_prime(i) {
            let j = blind_cyclops(i);
            if is_prime(j) {
                result.push(i);
            }
        }
        i += 1;
    }
    
    result
}

fn is_palindrome(n: i32) -> bool {
    let mut k = 0;
    let mut l = n;
    
    while l != 0 {
        let m = l % 10;
        k = 10 * k + m;
        l /= 10;
    }
    
    n == k
}

fn first_palindrome_cyclops_primes(n: usize) -> Vec<i32> {
    let mut result = Vec::new();
    let mut i = 0;
    
    while result.len() < n {
        if is_cyclops_number(i) && is_prime(i) && is_palindrome(i) {
            result.push(i);
        }
        i += 1;
    }
    
    result
}
