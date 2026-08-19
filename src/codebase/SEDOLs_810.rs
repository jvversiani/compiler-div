// Rosetta Code task: SEDOLs
// Source: https://rosettacode.org/wiki/SEDOLs#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 710889 SEDOL: "7108899"
// B0YBKJ SEDOL: "B0YBKJ7"
// 406566 SEDOL: "4065663"
// B0YBLH SEDOL: "B0YBLH2"
// 228276 SEDOL: "2282765"
// B0YBKL SEDOL: "B0YBKL9"
// 557910 SEDOL: "5579107"
// B0YBKR SEDOL: "B0YBKR5"
// 585284 SEDOL: "5852842"
// B0YBKT SEDOL: "B0YBKT7"
// B00030 SEDOL: "B000300"
// =======================

fn sedol(input: &str) -> Option<String> {
    let weights = vec![1, 3, 1, 7, 3, 9, 1];
    let valid_chars = "0123456789BCDFGHJKLMNPQRSTVWXYZ";

    if input.len() != 6 {
        return None;
    }

    // could be done by regex if needed
    for c in input.chars() {
        if !valid_chars.contains(c) {
            return None;
        }
    }

    let mut result: u32 = input
        .chars()
        .map(|c| {
            if c.is_digit(10) {
                c as u32 - 48
            } else {
                c as u32 - 55
            }
        })
        .zip(weights)
        .map(|(cnum, w)| w * cnum)
        .collect::<Vec<u32>>()
        .iter()
        .sum();

    result = (10 - result % 10) % 10;

    Some(input.to_owned() + &result.to_string())
}

fn main() {
    let inputs = vec![
        "710889", "B0YBKJ", "406566", "B0YBLH", "228276", "B0YBKL", "557910", "B0YBKR", "585284",
        "B0YBKT", "B00030",
    ];

    for input in inputs {
        println!("{} SEDOL: {:?}", &input, sedol(&input).unwrap());
    }
}
