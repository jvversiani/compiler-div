// Rosetta Code task: Nonoblock
// Source: https://rosettacode.org/wiki/Nonoblock#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 5 cells and [2, 1] blocks
// ===========
// |A|A|_|B|_|
// |A|A|_|_|B|
// |_|A|A|_|B|
//
// 5 cells and [] blocks
// ===========
// |_|_|_|_|_|
//
// 10 cells and [8] blocks
// =====================
// |A|A|A|A|A|A|A|A|_|_|
// |_|A|A|A|A|A|A|A|A|_|
// |_|_|A|A|A|A|A|A|A|A|
//
// 15 cells and [2, 3, 2, 3] blocks
// ===============================
// |A|A|_|B|B|B|_|C|C|_|D|D|D|_|_|
// |A|A|_|B|B|B|_|C|C|_|_|D|D|D|_|
// |A|A|_|B|B|B|_|C|C|_|_|_|D|D|D|
// |A|A|_|B|B|B|_|_|C|C|_|D|D|D|_|
// |A|A|_|B|B|B|_|_|C|C|_|_|D|D|D|
// |A|A|_|B|B|B|_|_|_|C|C|_|D|D|D|
// |A|A|_|_|B|B|B|_|C|C|_|D|D|D|_|
// |A|A|_|_|B|B|B|_|C|C|_|_|D|D|D|
// |A|A|_|_|B|B|B|_|_|C|C|_|D|D|D|
// |A|A|_|_|_|B|B|B|_|C|C|_|D|D|D|
// |_|A|A|_|B|B|B|_|C|C|_|D|D|D|_|
// |_|A|A|_|B|B|B|_|C|C|_|_|D|D|D|
// |_|A|A|_|B|B|B|_|_|C|C|_|D|D|D|
// |_|A|A|_|_|B|B|B|_|C|C|_|D|D|D|
// |_|_|A|A|_|B|B|B|_|C|C|_|D|D|D|
//
// 5 cells and [2, 3] blocks
// ===========
// No solutions.
// =======================

struct Nonoblock {
  width: usize,
  config: Vec<usize>,
  spaces: Vec<usize>,
}

impl Nonoblock {
  pub fn new(width: usize, config: Vec<usize>) -> Nonoblock {
    Nonoblock {
      width: width,
      config: config,
      spaces: Vec::new(),
    }
  }

  pub fn solve(&mut self) -> Vec<Vec<i32>> {
    let mut output: Vec<Vec<i32>> = Vec::new();
    self.spaces = (0..self.config.len()).fold(Vec::new(), |mut s, i| {
      s.push(match i {
        0 => 0,
        _ => 1,
      });
      s
    });
    if self.spaces.iter().sum::<usize>() + self.config.iter().sum::<usize>() <= self.width {
      'finished: loop {
        match self.spaces.iter().enumerate().fold((0, vec![0; self.width]), |mut a, (i, s)| {
            (0..self.config[i]).for_each(|j| a.1[a.0 + j + *s] = 1 + i as i32);
            return (a.0 + self.config[i] + *s, a.1);
          }) {
          (_, out) => output.push(out),
        }
        let mut i: usize = 1;
        'calc: loop {
          let len = self.spaces.len();
          if i > len {
            break 'finished;
          } else {
            self.spaces[len - i] += 1
          }
          if self.spaces.iter().sum::<usize>() + self.config.iter().sum::<usize>() > self.width {
            self.spaces[len - i] = 1;
            i += 1;
          } else {
            break 'calc;
          }
        }
      }
    }
    output
  }
}

fn main() {
  let mut blocks = [
    Nonoblock::new(5, vec![2, 1]),
    Nonoblock::new(5, vec![]),
    Nonoblock::new(10, vec![8]),
    Nonoblock::new(15, vec![2, 3, 2, 3]),
    Nonoblock::new(5, vec![2, 3]),
  ];

  for block in blocks.iter_mut() {
    println!("{} cells and {:?} blocks", block.width, block.config);
    println!("{}",(0..block.width).fold(String::from("="), |a, _| a + "=="));
    let solutions = block.solve();
    if solutions.len() > 0 {
      for solution in solutions.iter() {
        println!("{}", solution.iter().fold(String::from("|"), |s, f| s + &match f {
          i if *i > 0 => (('A' as u8 + ((*i - 1) as u8) % 26) as char).to_string(),
          _ => String::from("_"),
        }+ "|"));
      }
    } else {
      println!("No solutions. ");
    }
    println!();
  }
}
