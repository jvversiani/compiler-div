// Rosetta Code task: N'th
// Source: https://rosettacode.org/wiki/N'th#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// [0, 26) :
// 0th, 1st, 2nd, 3rd, 4th, 5th, 6th, 7th, 8th, 9th, 10th, 11th, 12th, 13th, 14th, 15th, 16th, 17th, 18th, 19th, 20th, 21st, 22nd, 23rd, 24th, 25th, 
// [250, 266) :
// 250th, 251st, 252nd, 253rd, 254th, 255th, 256th, 257th, 258th, 259th, 260th, 261st, 262nd, 263rd, 264th, 265th, 
// [1000, 1026) :
// 1000th, 1001st, 1002nd, 1003rd, 1004th, 1005th, 1006th, 1007th, 1008th, 1009th, 1010th, 1011th, 1012th, 1013th, 1014th, 1015th, 1016th, 1017th, 1018th, 1019th, 1020th, 1021st, 1022nd, 1023rd, 1024th, 1025th,
// =======================

fn nth(num: isize) -> String {
    format!("{}{}", num, match (num % 10, num % 100) {
        (1, 11) | (2, 12) | (3, 13) => "th",
        (1, _) => "st",
        (2, _) => "nd",
        (3, _) => "rd",
        _ => "th",
    })
}

fn main() {
    let ranges = [(0, 26), (250, 266), (1000, 1026)];
    for &(s, e) in &ranges {
        println!("[{}, {}) :", s, e);
        for i in s..e {
            print!("{}, ", nth(i));
        }
        println!();
    }
}
