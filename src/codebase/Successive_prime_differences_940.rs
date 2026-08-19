// Rosetta Code task: Successive prime differences
// Source: https://rosettacode.org/wiki/Successive_prime_differences#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// [1] number: 1
// 	first: [2, 3]
//
// [2] number: 8169
// 	first: [3, 5]
// 	last: [999959, 999961]
//
// [2, 2] number: 1
// 	first: [3, 5, 7]
//
// [2, 4] number: 1393
// 	first: [5, 7, 11]
// 	last: [999431, 999433, 999437]
//
// [4, 2] number: 1444
// 	first: [7, 11, 13]
// 	last: [997807, 997811, 997813]
//
// [6, 4, 2] number: 306
// 	first: [31, 37, 41, 43]
// 	last: [997141, 997147, 997151, 997153]
//
// [2, 4, 6] number: 279
// 	first: [17, 19, 23, 29]
// 	last: [997097, 997099, 997103, 997109]
// =======================

fn is_prime(num: u32) -> bool {
  match num {
      x if x < 4 => x > 1,
      x if x % 2 == 0 => false,
      x => { let limit = (x as f32).sqrt().ceil() as u32;
              (3..=limit).step_by(2).all(|a| x % a != 0)
            }
  } 
}

fn primes_by_diffs(primes: &[u32], diffs: &[u32]) -> Vec<Vec<u32>> {

  fn select(diffs: &[u32], prime_win: &[u32], acc: bool) -> bool {
    if diffs.is_empty() || !acc {
      acc
    }
    else {
      let acc1 = prime_win[0] + diffs[0] == prime_win[1];
      select(&diffs[1..], &prime_win[1..], acc1)
    }
  }
  
  primes.windows(diffs.len() + 1)
        .filter(|&win| select(diffs, win, true))
        .map(|win| win.to_vec())
        .collect()
}

fn main() {
  let limit = 1_000_000u32;
  let primes = (2..).filter(|&i| is_prime(i));
  let prime_list: Vec<u32> = primes.take_while(|&p| p <= limit).collect();
  for diffs in vec!(vec!(1), vec!(2), vec!(2,2), vec!(2,4), vec!(4,2), vec!(6,4,2), vec!(2,4,6)) {
    let result_list = primes_by_diffs(&prime_list, &diffs);
    let len = result_list.len();
    println!("{:?} number: {}\n\tfirst: {:?}", diffs, len, result_list[0]);
    if len == 1 {
      println!()
    }
    if len > 1 {
      println!("\tlast: {:?}\n", result_list.last().unwrap())
    }
  }
}
