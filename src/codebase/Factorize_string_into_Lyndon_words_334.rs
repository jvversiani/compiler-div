// Rosetta Code task: Factorize string into Lyndon words
// Source: https://rosettacode.org/wiki/Factorize_string_into_Lyndon_words#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// ["011", "01", "0011", "00101101", "0010110011010011", "00101100110100101101001100101101", "001011001101001011010011001011001101001100101101", "001011001101", "001"]
// =======================

fn chen_fox_lyndon_factorization(s: &str) -> Vec<String> {
    let n = s.len();
    let mut i = 0;
    let mut factorization = Vec::new();
    while i < n {
        let (mut j, mut k) = (i + 1, i);
        while j < n && s.as_bytes()[k] <= s.as_bytes()[j] {
            if s.as_bytes()[k] < s.as_bytes()[j] {
                k = i;
            } else {
                k += 1;
            }
            j += 1;
        }
        while i <= k {
            factorization.push(s[i..i + j - k].to_string());
            i += j - k;
        }
    }
    factorization
}

fn main() {
    let mut m = String::from("0");
    for _ in 0..7 {
        let m0 = m.clone();
        m = m.replace("0", "a");
        m = m.replace("1", "0");
        m = m.replace("a", "1");
        m = m0 + &m;
    }

    let factorization = chen_fox_lyndon_factorization(&m);
    println!("{:?}", factorization);
}
