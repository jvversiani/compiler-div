// Rosetta Code task: Matrix-exponentiation operator
// Source: https://rosettacode.org/wiki/Matrix-exponentiation_operator#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Power of 0:
//      1      0      0
//      0      1      0
//      0      0      1
//
// Power of 1:
//      1      2      0
//      0      3      1
//      1      0      0
//
// Power of 2:
//      1      8      2
//      1      9      3
//      1      2      0
//
// Power of 3:
//      3     26      8
//      4     29      9
//      1      8      2
//
// Power of 4:
//     11     84     26
//     13     95     29
//      3     26      8
//
// Power of 5:
//     37    274     84
//     42    311     95
//     11     84     26
//
// Power of 6:
//    121    896    274
//    137   1017    311
//     37    274     84
//
// Power of 7:
//    395   2930    896
//    448   3325   1017
//    121    896    274
//
// Power of 8:
//   1291   9580   2930
//   1465  10871   3325
//    395   2930    896
//
// Power of 9:
//   4221  31322   9580
//   4790  35543  10871
//   1291   9580   2930
//
// Power of 10:
//  13801 102408  31322
//  15661 116209  35543
//   4221  31322   9580
// =======================

use std::fmt;
use std::ops;
const WIDTH: usize = 6;

#[derive(Clone)]
struct SqMat {
    data: Vec<Vec<i64>>,
}

impl fmt::Debug for SqMat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut row = "".to_string();
        for i in &self.data {
            for j in i {
                row += &format!("{:>w$} ", j, w = WIDTH);
            }
            row += &"\n";
        }
        write!(f, "{}", row)
    }
}

impl ops::BitXor<u32> for SqMat {
    type Output = Self;

    fn bitxor(self, n: u32) -> Self::Output {
        let mut aux = self.data.clone();
        let mut ans: SqMat = SqMat {
            data: vec![vec![0; aux.len()]; aux.len()],
        };
        for i in 0..aux.len() {
            ans.data[i][i] = 1;
        }
        let mut b = n;
        while b > 0 {
            if b & 1 > 0 {
                // ans = ans * aux
                let mut tmp = aux.clone();
                for i in 0..aux.len() {
                    for j in 0..aux.len() {
                        tmp[i][j] = 0;
                        for k in 0..aux.len() {
                            tmp[i][j] += ans.data[i][k] * aux[k][j];
                        }
                    }
                }
                ans.data = tmp;
            }
            b >>= 1;
            if b > 0 {
                // aux = aux * aux
                let mut tmp = aux.clone();
                for i in 0..aux.len() {
                    for j in 0..aux.len() {
                        tmp[i][j] = 0;
                        for k in 0..aux.len() {
                            tmp[i][j] += aux[i][k] * aux[k][j];
                        }
                    }
                }
                aux = tmp;
            }
        }
        ans
    }
}

fn main() {
    let sm: SqMat = SqMat {
        data: vec![vec![1, 2, 0], vec![0, 3, 1], vec![1, 0, 0]],
    };
    for i in 0..11 {
        println!("Power of {}:\n{:?}", i, sm.clone() ^ i);
    }
}
