// Rosetta Code task: De Polignac numbers
// Source: https://rosettacode.org/wiki/De_Polignac_numbers#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// First 50 de Polignac numbers:
//     1   127   149   251   331   337   373   509   599   701
//   757   809   877   905   907   959   977   997  1019  1087
//  1199  1207  1211  1243  1259  1271  1477  1529  1541  1549
//  1589  1597  1619  1649  1657  1719  1759  1777  1783  1807
//  1829  1859  1867  1927  1969  1973  1985  2171  2203  2213
//
// One thousandth: 31941
//
// Ten thousandth: 273421
// =======================

fn is_prime(n: i32) -> bool {
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

fn is_depolignac_number(n: i32) -> bool {
    let mut p = 1;
    while p < n {
        if is_prime(n - p) {
            return false;
        }
        p <<= 1;
    }
    true
}

fn main() {
    println!("First 50 de Polignac numbers:");
    let mut n = 1;
    let mut count = 0;
    while count < 10000 {
        if is_depolignac_number(n) {
            count += 1;
            if count <= 50 {
                print!("{:5}", n);
                if count % 10 == 0 {
                    println!();
                } else {
                    print!(" ");
                }
            } else if count == 1000 {
                println!("\nOne thousandth: {}", n);
            } else if count == 10000 {
                println!("\nTen thousandth: {}", n);
            }
        }
        n += 2;
    }
}
