// Rosetta Code task: Sum digits of an integer
// Source: https://rosettacode.org/wiki/Sum_digits_of_an_integer#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 10
// =======================

struct DigitIter(usize, usize);

impl Iterator for DigitIter {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let ret = self.0 % self.1;
            self.0 /= self.1;
            Some(ret)
        }
    }
}

fn main() {
    println!("{}", DigitIter(1234,10).sum::<usize>());
}
