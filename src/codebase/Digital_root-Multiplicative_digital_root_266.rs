// Rosetta Code task: Digital root/Multiplicative digital root
// Source: https://rosettacode.org/wiki/Digital_root/Multiplicative_digital_root#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Number: (MP, MDR)
// ======  =========
// 123321: (3, 8)
//   7739: (3, 8)
//    893: (3, 2)
// 899998: (2, 0)
//
// MDR     First 5 with matching MDR
// ===     =========================
//  0:     0    10    20    25    30
//  1:     1    11   111  1111 11111
//  2:     2    12    21    26    34
//  3:     3    13    31   113   131
//  4:     4    14    22    27    39
//  5:     5    15    35    51    53
//  6:     6    16    23    28    32
//  7:     7    17    71   117   171
//  8:     8    18    24    29    36
//  9:     9    19    33    91   119
// =======================

// Multiplicative digital root
fn mdroot(n: u32) -> (u32, u32) {
    let mut count = 0;
    let mut mdr = n; 
    while mdr > 9 {
        let mut m = mdr;
        let mut digits_mul = 1;
        while m > 0 {
            digits_mul *= m % 10;
            m /= 10;
        }
        mdr = digits_mul;
        count += 1;
    }
    return (count, mdr);
}

fn main() {
    println!("Number: (MP, MDR)\n======  =========");
    for n in [123321, 7739, 893, 899998] {
        println!("{:6}: {:?}", n, mdroot(n));
    }        
    let mut table = vec![vec![0_u32; 0]; 10];
    let mut n = 0;
    while table.iter().map(|row| row.len()).min().unwrap() < 5 {
        let (_, mdr) = mdroot(n);
        table[mdr as usize].push(n);
        n += 1;
    }
    println!("\nMDR     First 5 with matching MDR\n===     =========================");
    table.sort();
    for a in table {
        println!("{:2}: {:5}{:6}{:6}{:6}{:6}", a[0], a[0], a[1], a[2], a[3], a[4]);
    }
}
