// Rosetta Code task: Hofstadter-Conway $10,000 sequence
// Source: https://rosettacode.org/wiki/Hofstadter-Conway_$10,000_sequence#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 2^ 0-2^ 1, 1.00000000
// 2^ 1-2^ 2, 0.66666667
// 2^ 2-2^ 3, 0.66666667
// 2^ 3-2^ 4, 0.63636364
// 2^ 4-2^ 5, 0.60869565
// 2^ 5-2^ 6, 0.59090909
// 2^ 6-2^ 7, 0.57608696
// 2^ 7-2^ 8, 0.56741573
// 2^ 8-2^ 9, 0.55945946
// 2^ 9-2^10, 0.55493741
// 2^10-2^11, 0.55010087
// 2^11-2^12, 0.54746289
// 2^12-2^13, 0.54414475
// 2^13-2^14, 0.54244271
// 2^14-2^15, 0.54007110
// 2^15-2^16, 0.53878402
// 2^16-2^17, 0.53704366
// 2^17-2^18, 0.53602007
// 2^18-2^19, 0.53464543
// 2^19-2^20, 0.53377923
// Winning number: 1489
// =======================

struct HofstadterConway {
    current: usize,
    sequence: Vec<usize>,
}

impl HofstadterConway {
    /// Define a constructor for the struct.
    fn new() -> HofstadterConway {
        HofstadterConway {
            current: 0,
            sequence: vec![1, 1],
        }
    }
}

impl Default for HofstadterConway {
    fn default() -> Self {
        Self::new()
    }
}

/// Implement the hofstadter q iteration sequence.
impl Iterator for HofstadterConway {
    type Item = usize;

    /// This gets called to fetch the next item of the iterator.
    fn next(&mut self) -> Option<usize> {
        let max_index = self.sequence.len() - 1;
        let last_value = self.sequence[max_index];

        if self.current > max_index {
            let new_x = self.sequence[last_value - 1] + self.sequence[max_index - last_value + 1];
            self.sequence.push(new_x);
        }
        self.current += 1;
        Some(self.sequence[self.current - 1])
    }
}

#[allow(clippy::cast_precision_loss)]
fn main() {
    let mut hof = HofstadterConway::new();
    let mut winning_num = 0_usize;

    for p in 0..20 {
        let max_hof = (2_usize.pow(p)..2_usize.pow(p + 1))
            .map(|n| (n, hof.next().unwrap() as f64 / n as f64))
            .fold(f64::NAN, |a, (n, b)| {
                if b >= 0.55 {
                    winning_num = n;
                }
                a.max(b)
            });

        println!("2^{:>2}-2^{:>2}, {:>.8}", p, p + 1, max_hof);
    }

    println!("Winning number: {}", winning_num);
}
