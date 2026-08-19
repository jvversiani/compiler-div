// Rosetta Code task: Zumkeller numbers
// Source: https://rosettacode.org/wiki/Zumkeller_numbers#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// First 220 Zumkeller numbers:
//   6, 12, 20, 24, 28, 30, 40, 42, 48, 54, 56, 60, 66, 70, 78, 80, 84, 88, 90, 96
// 102,104,108,112,114,120,126,132,138,140,150,156,160,168,174,176,180,186,192,198
// 204,208,210,216,220,222,224,228,234,240,246,252,258,260,264,270,272,276,280,282
// 294,300,304,306,308,312,318,320,330,336,340,342,348,350,352,354,360,364,366,368
// 372,378,380,384,390,396,402,408,414,416,420,426,432,438,440,444,448,456,460,462
// 464,468,474,476,480,486,490,492,496,498,500,504,510,516,520,522,528,532,534,540
// 544,546,550,552,558,560,564,570,572,580,582,588,594,600,606,608,612,616,618,620
// 624,630,636,640,642,644,650,654,660,666,672,678,680,684,690,696,700,702,704,708
// 714,720,726,728,732,736,740,744,750,756,760,762,768,770,780,786,792,798,804,810
// 812,816,820,822,828,832,834,836,840,852,858,860,864,868,870,876,880,888,894,896
// 906,910,912,918,920,924,928,930,936,940,942,945,948,952,960,966,972,978,980,984
// =======================

use std::convert::TryInto;

/// Gets all divisors of a number, including itself
fn get_divisors(n: u32) -> Vec<u32> {
    let mut results = Vec::new();

    for i in 1..(n / 2 + 1) {
        if n % i == 0 {
            results.push(i);
        }
    }
    results.push(n);
    results
}

/// Calculates whether the divisors can be partitioned into two disjoint
/// sets that sum to the same value
fn is_summable(x: i32, divisors: &[u32]) -> bool {
    if !divisors.is_empty() {
        if divisors.contains(&(x as u32)) {
            return true;
        } else if let Some((first, t)) = divisors.split_first() {
            return is_summable(x - *first as i32, &t) || is_summable(x, &t);
        }
    }
    false
}

/// Calculates whether the number is a Zumkeller number
/// Zumkeller numbers are the set of numbers whose divisors can be partitioned
/// into two disjoint sets that sum to the same value. Each sum must contain
/// divisor values that are not in the other sum, and all of the divisors must
/// be in one or the other.
fn is_zumkeller_number(number: u32) -> bool {
    if number % 18 == 6 || number % 18 == 12 {
        return true;
    }

    let div = get_divisors(number);
    let divisor_sum: u32 = div.iter().sum();
    if divisor_sum == 0 {
        return false;
    }
    if divisor_sum % 2 == 1 {
        return false;
    }

    // numbers where n is odd and the abundance is even are Zumkeller numbers
    let abundance = divisor_sum as i32 - 2 * number as i32;
    if number % 2 == 1 && abundance > 0 && abundance % 2 == 0 {
        return true;
    }

    let half = divisor_sum / 2;
    return div.contains(&half)
        || (div.iter().filter(|&&d| d < half).count() > 0
            && is_summable(half.try_into().unwrap(), &div));
}

fn main() {
    println!("First 220 Zumkeller numbers:");
    let mut counter: u32 = 0;
    let mut i: u32 = 0;
    while counter < 220 {
        if is_zumkeller_number(i) {
            print!("{:>3}", i);
            counter += 1;
            print!("{}", if counter % 20 == 0 { "\n" } else { "," });
        }
        i += 1;
    }
}
