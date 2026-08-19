// Rosetta Code task: Anagram generator
// Source: https://rosettacode.org/wiki/Anagram_generator#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// [1, 1, 12]
// [1, 2, 11]
// [1, 3, 10]
// [1, 4, 9]
// [1, 5, 8]
// [1, 6, 7]
// [2, 2, 10]
// [2, 3, 9]
// [2, 4, 8]
// [2, 5, 7]
// [2, 6, 6]
// [3, 3, 8]
// [3, 4, 7]
// [3, 5, 6]
// [4, 4, 6]
// [4, 5, 5]
// =======================

struct Sums {
    char_count: usize,
    current: Box<[usize]>,
    done: bool,
}

impl Sums {
    fn from_counts(word_count: usize, char_count: usize) -> Sums {
        Sums {
            char_count,
            current: vec![1; word_count].into(),
            done: false,
        }
    }

    fn sum(&self) -> usize {
        self.current.iter().sum()
    }

    fn is_valid(&self) -> bool {
        self.sum() == self.char_count
    }

    /// Advances the current array to next lexicographic-like array starting at the end
    fn advance(&mut self) {
        let len = self.current.len();

        for i in (0..len).rev() {
            if self.current[i] < self.char_count {
                self.current[i] += 1;

                // Only called if not on last index, meaning we have to reset those values
                for j in i + 1..len {
                    self.current[j] = self.current[i];
                }

                if Self::sum(self) <= self.char_count {
                    return;
                }

                // We incremented too high, we have to go back previous index
            }
        }
        self.done = true;
    }
}

impl Iterator for Sums {
    type Item = Box<[usize]>;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.done {
            if self.is_valid() {
                let result = self.current.clone();
                self.advance();
                return Some(result);
            }
            self.advance();
        }
        None
    }
}

fn main() {
    // "pizza party time": 3 words, 14 non-space characters.
    // Sums yields every way those 14 chars split across 3 words (each >= 1).
    let word_count = 3;
    let char_count = 14;

    let sums = Sums::from_counts(word_count, char_count);
    for combo in sums {
        println!("{:?}", combo);
    }
}