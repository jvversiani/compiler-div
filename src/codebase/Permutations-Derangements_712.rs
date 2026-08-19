// Rosetta Code task: Permutations/Derangements
// Source: https://rosettacode.org/wiki/Permutations/Derangements#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Deranged Four:
// dabc
// dcab
// dcba
// cdba
// cdab
// cadb
// bdac
// bcda
// badc
//
// Compare list vs calc:
// 0:      1       1
// 1:      0       0
// 2:      1       1
// 3:      2       2
// 4:      9       9
// 5:      44      44
// 6:      265     265
// 7:      1854    1854
// 8:      14833   14833
// 9:      133496  133496
//
// further calc:
// 10: 1334961
// 11: 14684570
// 12: 176214841
// 13: 2290792932
// 14: 32071101049
// 15: 481066515734
// 16: 7697064251745
// 17: 130850092279664
// 18: 2355301661033953
// 19: 44750731559645106
// 20: 895014631192902121
// 21: 18795307255050944540
// 22: 413496759611120779881
// 23: 9510425471055777937262
// 24: 228250211305338670494289
// 25: 5706255282633466762357224
// 26: 148362637348470135821287825
// 27: 4005791208408693667174771274
// 28: 112162153835443422680893595673
// 29: 3252702461227859257745914274516
// 30: 97581073836835777732377428235481
// 31: 3025013288941909109703700275299910
// 32: 96800425246141091510518408809597121
// =======================

fn deranged(depth: usize, len: usize, d: &mut Vec<u8>, show: bool) -> i128 {
    let mut count: i128 = 0;

    if depth == len {
        if show {
            for i in 0..len {
                print!("{}", (d[i] + 'a' as u8) as char);
            }
            println!();
        }
        return 1_i128;
    }
    for i in (depth..len).rev() {
        if i != d[depth].into() {
            let mut tmp = d[i];
            d[i] = d[depth];
            d[depth] = tmp;
            count += deranged(depth + 1, len, d, show);
            tmp = d[i];
            d[i] = d[depth];
            d[depth] = tmp;
        }
    }
    return count;
}

fn gen_n(n: usize, show: bool) -> i128 {
    let a = &mut [0_u8; 1024].to_vec();
    for i in 0..n {
        a[i] = i as u8;
    }
    return deranged(0, n, a, show);
}

fn sub_fact(n: usize) -> i128 {
    return if n < 2 {
        (1 - n) as i128
    } else {
        (sub_fact(n - 1) + sub_fact(n - 2)) * ((n - 1) as i128)
    };
}

fn main() {
    println!("Deranged Four:");
    gen_n(4, true);

    println!("\nCompare list vs calc:");
    for i in 0..10 {
        println!("{}:\t{}\t{}", i, gen_n(i, false), sub_fact(i));
    }

    println!("\nfurther calc:");
    for i in 10..33 {
        println!("{}: {}", i, sub_fact(i));
    }
}
