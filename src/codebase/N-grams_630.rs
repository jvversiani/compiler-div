// Rosetta Code task: N-grams
// Source: https://rosettacode.org/wiki/N-grams#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 2-grams of 'LIVE AND LET LIVE':
// ' A' - 1	' L' - 2	'AN' - 1	'D ' - 1	'E ' - 1
// 'ET' - 1	'IV' - 2	'LE' - 1	'LI' - 2	'ND' - 1
// 'T ' - 1	'VE' - 2	
// 3-grams of 'LIVE AND LET LIVE':
// ' AN' - 1	' LE' - 1	' LI' - 1	'AND' - 1	'D L' - 1
// 'E A' - 1	'ET ' - 1	'IVE' - 2	'LET' - 1	'LIV' - 2
// 'ND ' - 1	'T L' - 1	'VE ' - 1	
// 4-grams of 'LIVE AND LET LIVE':
// ' AND' - 1	' LET' - 1	' LIV' - 1	'AND ' - 1	'D LE' - 1
// 'E AN' - 1	'ET L' - 1	'IVE ' - 1	'LET ' - 1	'LIVE' - 2
// 'ND L' - 1	'T LI' - 1	'VE A' - 1	
// =======================

use std::collections::BTreeMap;

fn find_ngrams(n: usize, s: &str) -> BTreeMap<String, i32> {
    let mut ngrams = BTreeMap::new();
    let chars: Vec<char> = s.chars().collect();
    
    if chars.len() < n {
        return ngrams;
    }
    
    let max_loc = chars.len() - n;
    for i in 0..=max_loc {
        let ngram: String = chars[i..i + n].iter().collect();
        *ngrams.entry(ngram).or_insert(0) += 1;
    }
    
    ngrams
}

fn print_ngrams(ngrams: &BTreeMap<String, i32>) {
    let mut col = 0;
    for (ngram, count) in ngrams {
        print!("'{}' - {}", ngram, count);
        if col % 5 == 4 {
            println!();
        } else {
            print!("\t");
        }
        col += 1;
    }
    println!();
}

fn main() {
    let s = "LIVE AND LET LIVE";
    
    for n in 2..=4 {
        println!("{}-grams of '{}':", n, s);
        print_ngrams(&find_ngrams(n, s));
    }
}
