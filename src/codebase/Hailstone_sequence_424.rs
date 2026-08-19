// Rosetta Code task: Hailstone sequence
// Source: https://rosettacode.org/wiki/Hailstone_sequence#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// For 27 number of elements is 112 
//   hailstone starting with 27, 82, 41, 124,  ending with 8, 4, 2, 1,  
// Longest sequence is 351 element long for seed 77031
// =======================

fn hailstone(start : u32) -> Vec<u32> {
    let mut res = Vec::new();
    let mut next = start;

    res.push(start);

    while next != 1  {
        next = if next % 2 == 0 { next/2 } else { 3*next+1 };
        res.push(next);
    }
    res
}

 
fn main() {
    let test_num = 27;
    let test_hailseq = hailstone(test_num);

    println!("For {} number of elements is {} ", test_num, test_hailseq.len());

    let fst_slice = test_hailseq[0..4].iter()
                        .fold("".to_owned(), |acc, i| { acc + &*(i.to_string()).to_owned() + ", " });
    let last_slice = test_hailseq[test_hailseq.len()-4..].iter()
                        .fold("".to_owned(), |acc, i| { acc + &*(i.to_string()).to_owned() + ", " });
    
    println!("  hailstone starting with {} ending with {} ", fst_slice, last_slice);

    let max_range = 100000;
    let mut max_len = 0;
    let mut max_seed = 0;
    for i_seed in 1..max_range {
        let i_len = hailstone(i_seed).len();

        if i_len > max_len {
            max_len = i_len;
            max_seed = i_seed;
        }
    }
    println!("Longest sequence is {} element long for seed {}", max_len, max_seed);
}
