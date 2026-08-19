// Rosetta Code task: State name puzzle
// Source: https://rosettacode.org/wiki/State_name_puzzle#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Real states only:
//  1  North Carolina + South Dakota = North Dakota + South Carolina
// 
// Real and fictitious states:
//  1  Kory New + New Kory = New York + Wen Kory
//  2  Kory New + New Kory = New York + York New
//  3  Kory New + New Kory = Wen Kory + York New
//  4  Kory New + New York = New Kory + Wen Kory
//  5  Kory New + New York = New Kory + York New
//  6  Kory New + New York = Wen Kory + York New
//  7  Kory New + Wen Kory = New Kory + New York
//  8  Kory New + Wen Kory = New Kory + York New
//  9  Kory New + Wen Kory = New York + York New
// 10  Kory New + York New = New Kory + New York
// 11  Kory New + York New = New Kory + Wen Kory
// 12  Kory New + York New = New York + Wen Kory
// 13  New Kory + New York = Wen Kory + York New
// 14  New Kory + Wen Kory = New York + York New
// 15  New Kory + York New = New York + Wen Kory
// 16  North Carolina + South Dakota = North Dakota + South Carolina
// =======================

use std::collections::HashMap;

fn solve(states: Vec<&str>) {
    let mut dict = HashMap::new();
    for state in states {
        let key = state.to_lowercase().replace(" ", "");
        if !dict.contains_key(&key) {
            dict.insert(key, state);
        }
    }
    let mut keys: Vec<&String> = dict.keys().collect();
    keys.sort();
    let mut solutions: Vec<String> = vec![];
    let mut duplicates: Vec<String> = vec![];
    for i in 0..keys.len() {
        for j in i + 1..keys.len() {
            let len = keys[i].len() + keys[j].len();
            let mut chars: Vec<char> = (String::new() + keys[i] + keys[j]).chars().collect();
            chars.sort();
            let combined: String = chars.into_iter().collect();
            for k in 0..keys.len() {
                for m in k + 1..keys.len() {
                    if k == i || k == j || m == i || m == j {
                        continue;
                    }
                    let len2 = keys[k].len() + keys[m].len();
                    if len2 != len {
                        continue;
                    }
                    let mut chars2 = (String::new() + keys[k] + keys[m])
                        .chars()
                        .collect::<Vec<char>>();
                    chars2.sort();
                    let combined2: String = chars2.into_iter().collect();
                    if combined == combined2 {
                        let f1 = format!("{} + {}", dict[keys[i]], dict[keys[j]]);
                        let f2 = format!("{} + {}", dict[keys[k]], dict[keys[m]]);
                        let f3 = format!("{f1} = {f2}");
                        if duplicates.contains(&f3) {
                            continue;
                        }
                        solutions.push(f3);
                        let f4 = format!("{f2} = {f1}");
                        duplicates.push(f4);
                    }
                }
            }
        }
    }
    solutions.sort();
    for (i, sol) in solutions.iter().enumerate() {
        println!("{:>2}  {}", i + 1, sol);
    }
}

fn main() {
    let mut states = [
        "Alabama", "Alaska", "Arizona", "Arkansas", "California", "Colorado", "Connecticut",
        "Delaware", "Florida", "Georgia", "Hawaii", "Idaho", "Illinois", "Indiana", "Iowa",
        "Kansas", "Kentucky", "Louisiana", "Maine", "Maryland", "Massachusetts", "Michigan",
        "Minnesota", "Mississippi", "Missouri", "Montana", "Nebraska", "Nevada",
        "New Hampshire", "New Jersey", "New Mexico", "New York", "North Carolina",
        "North Dakota", "Ohio", "Oklahoma", "Oregon", "Pennsylvania", "Rhode Island",
        "South Carolina", "South Dakota", "Tennessee", "Texas", "Utah", "Vermont",
         "Virginia", "Washington", "West Virginia", "Wisconsin", "Wyoming",].to_vec();
    println!("Real states only:");
    solve(states.clone());
    let mut fictitious = ["New Kory", "Wen Kory", "York New", "Kory New", "New Kory"].to_vec();
    println!("\nReal and fictitious states:");
    states.append(&mut fictitious);
    solve(states.clone());
}
