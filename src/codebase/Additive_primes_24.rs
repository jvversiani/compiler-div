// Rosetta Code task: Additive primes
// Source: https://rosettacode.org/wiki/Additive_primes#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
//
//    2   3   5   7  11  23  29  41  43  47
//   61  67  83  89 101 113 131 137 139 151
//  157 173 179 191 193 197 199 223 227 229
//  241 263 269 281 283 311 313 317 331 337
//  353 359 373 379 397 401 409 421 443 449
//  461 463 467 487
// ---
// Found 54 additive primes less than 500
// =======================

fn main() {
    let limit = 500;
    let column_w = limit.to_string().len() + 1;
    let mut pms = Vec::with_capacity(limit / 2 - limit / 3 / 2 - limit / 5 / 3 / 2 + 1);
    let mut count = 0;
    for u in (2..3).chain((3..limit).step_by(2)) {
        if pms.iter().take_while(|&&p| p * p <= u).all(|&p| u % p != 0) {
            pms.push(u);
            let dgs = std::iter::successors(Some(u), |&n| (n > 9).then(|| n / 10)).map(|n| n % 10);
            if pms.binary_search(&dgs.sum()).is_ok() {
                print!("{}{u:column_w$}", if count % 10 == 0 { "\n" } else { "" });
                count += 1;
            }
        }
    }
    println!("\n---\nFound {count} additive primes less than {limit}");
}
