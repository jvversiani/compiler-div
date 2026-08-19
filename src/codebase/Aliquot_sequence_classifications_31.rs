// Rosetta Code task: Aliquot sequence classifications
// Source: https://rosettacode.org/wiki/Aliquot_sequence_classifications#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 1 (Terminating, [1, 0])
// 2 (Terminating, [2, 1, 0])
// 3 (Terminating, [3, 1, 0])
// 4 (Terminating, [4, 3, 1, 0])
// 5 (Terminating, [5, 1, 0])
// 6 (Perfect, [6, 6])
// 7 (Terminating, [7, 1, 0])
// 8 (Terminating, [8, 7, 1, 0])
// 9 (Terminating, [9, 4, 3, 1, 0])
// 10 (Terminating, [10, 8, 7, 1, 0])
// 11 (Terminating, [11, 1, 0])
// 12 (Terminating, [12, 16, 15, 9, 4, 3, 1, 0])
// 28 (Perfect, [28, 28])
// 496 (Perfect, [496, 496])
// 220 (Amicable, [220, 284, 220])
// 1184 (Amicable, [1184, 1210, 1184])
// 12496 (Sociable, [12496, 14288, 15472, 14536, 14264, 12496])
// 1264460 (Sociable, [1264460, 1547860, 1727636, 1305184, 1264460])
// 790 (Aspiring, [790, 650, 652, 496, 496])
// 909 (Aspiring, [909, 417, 143, 25, 6, 6])
// 562 (Cyclic, [562, 284, 220, 284])
// 1064 (Cyclic, [1064, 1336, 1184, 1210, 1184])
// 1488 (NonTerminating, [1488, 2480, 3472, 4464, 8432, 9424, 10416, 21328, 22320, 55056, 95728, 96720, 236592, 459792, 881392, 882384, 1474608])
// =======================

#[derive(Debug)]
enum AliquotType { Terminating, Perfect, Amicable, Sociable, Aspiring, Cyclic, NonTerminating }

fn classify_aliquot(num: i64) -> (AliquotType, Vec<i64>) {
    let limit = 1i64 << 47; //140737488355328
    let mut terms = Some(num).into_iter().collect::<Vec<_>>();
    for i in 0..16 {
        let n = terms[i];
        let divsum = (1..(n + 1) / 2 + 1).filter(|&x| n % x == 0 && n != x).fold(0, |sum, x| sum + x);
        let classification = if divsum == 0 {
            Some(AliquotType::Terminating)
        }
        else if divsum > limit {
            Some(AliquotType::NonTerminating)
        }
        else if let Some(prev_idx) = terms.iter().position(|&x| x == divsum) {
            let cycle_len = terms.len() - prev_idx;
            Some(if prev_idx == 0 {
                match cycle_len {
                    1 => AliquotType::Perfect,
                    2 => AliquotType::Amicable,
                    _ => AliquotType::Sociable
                }
            }
            else {
                if cycle_len == 1 {AliquotType::Aspiring} else {AliquotType::Cyclic}
            })
        }
        else {
            None
        };
        terms.push(divsum);
        if let Some(result) = classification {
            return (result, terms);
        }
    }
    (AliquotType::NonTerminating, terms)
}

fn main() {
    let nums = [1i64, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 28, 496, 220, 1184, 12496, 1264460, 790, 909, 562, 1064, 1488/*, 15355717786080*/];
    for num in &nums {
        println!("{} {:?}", num, classify_aliquot(*num));
    }
}
