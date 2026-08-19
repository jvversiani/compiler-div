// Rosetta Code task: Jordan-Pólya numbers
// Source: https://rosettacode.org/wiki/Jordan-Pólya_numbers#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// The first 50 Jordan-Polya numbers:
//     1    2    4    6    8   12   16   24   32   36
//    48   64   72   96  120  128  144  192  216  240
//   256  288  384  432  480  512  576  720  768  864
//   960 1024 1152 1296 1440 1536 1728 1920 2048 2304
//  2592 2880 3072 3456 3840 4096 4320 4608 5040 5184
//
// The largest Jordan-Polya number less than 100 million: 99532800
//
// The 800th Jordan-Polya number is: 18345885696 = 4!^7 * 2!^2
// The 1050th Jordan-Polya number is: 139345920000 = 8! * 5!^3 * 2!
// The 1800th Jordan-Polya number is: 9784472371200 = 6!^2 * 4!^2 * 2!^15
// The 2800th Jordan-Polya number is: 439378587648000 = 14! * 7!
// The 3800th Jordan-Polya number is: 7213895789838336 = 4!^8 * 2!^16
// =======================

use std::collections::{HashMap, HashSet};

const LIMIT: u64 = 1 << 53;

fn to_string(map: &HashMap<u64, u64>) -> String {
    let mut result = String::new();
    let mut entries: Vec<_> = map.iter().collect();
    entries.sort();
    for (&key, &value) in entries {
        // println!("key={} value={}", key, value);
        let part = if value == 1 {
            format!("{}!", key)
        } else {
            format!("{}!^{}", key, value)
        };
        result = part + " * " + &result;
    }
    result.trim_end_matches(" * ").to_string()
}

fn insert_or_update(map: &mut HashMap<u64, u64>, entry: u64) {
    let count = map.entry(entry).or_insert(0);
    *count += 1;
}

fn create_jordan_polya() -> (HashSet<u64>, HashMap<u64, HashMap<u64, u64>>) {
    let mut jordan_polya_set = HashSet::new();
    let mut decompositions = HashMap::new();

    jordan_polya_set.insert(1);
    decompositions.insert(1, HashMap::new());
    let mut factorial = 1u64;

    for multiplier in 2..=20 {
        factorial *= multiplier; // Using u64 for multiplier
        let mut to_update = Vec::new();

        for &number in &jordan_polya_set {
            let mut current = number;
            while current <= LIMIT / factorial {
                to_update.push((current, current * factorial)); // Store original and new number
                current *= factorial;
            }
        }

        for (original, new_number) in to_update {
            jordan_polya_set.insert(new_number);
            let mut new_decomposition = decompositions[&original].clone();
            insert_or_update(&mut new_decomposition, multiplier);
            decompositions.insert(new_number, new_decomposition);
        }
    }

    (jordan_polya_set, decompositions)
}


fn main() {
    let (jordan_polya_set, decompositions) = create_jordan_polya();
    let mut jordan_polya: Vec<_> = jordan_polya_set.into_iter().collect();
    jordan_polya.sort();

    println!("The first 50 Jordan-Polya numbers:");
    for i in 0..50 {
        print!("{:5}", jordan_polya[i]);
        if i % 10 == 9 {
            println!();
        }
    }

    let hundred_million = jordan_polya.iter().position(|&x| x >= 100_000_000).unwrap();
    println!(
        "\nThe largest Jordan-Polya number less than 100 million: {}\n",
        jordan_polya[hundred_million - 1]
    );

    for &i in &[800, 1050, 1800, 2800, 3800] {
        println!(
            "The {}th Jordan-Polya number is: {} = {}",
            i,
            jordan_polya[i - 1],
            to_string(&decompositions[&jordan_polya[i - 1]])
        );
    }
}
