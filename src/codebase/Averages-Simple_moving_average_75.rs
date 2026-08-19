// Rosetta Code task: Averages/Simple moving average
// Source: https://rosettacode.org/wiki/Averages/Simple_moving_average#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Moving average with period 3
// Number: 1 | Average: 1
// Number: 2 | Average: 1.5
// Number: 3 | Average: 2
// Number: 4 | Average: 3
// Number: 5 | Average: 4
// Number: 5 | Average: 4.666666666666667
// Number: 4 | Average: 4.666666666666667
// Number: 3 | Average: 4
// Number: 2 | Average: 3
// Number: 1 | Average: 2
// Moving average with period 5
// Number: 1 | Average: 1
// Number: 2 | Average: 1.5
// Number: 3 | Average: 2
// Number: 4 | Average: 2.5
// Number: 5 | Average: 3
// Number: 5 | Average: 3.8
// Number: 4 | Average: 4.2
// Number: 3 | Average: 4.2
// Number: 2 | Average: 3.8
// Number: 1 | Average: 3
// =======================

struct SimpleMovingAverage {
    period: usize,
    numbers: Vec<usize>
}

impl SimpleMovingAverage {
    fn new(p: usize) -> SimpleMovingAverage {
        SimpleMovingAverage {
            period: p,
            numbers: Vec::new()
        }
    }

    fn add_number(&mut self, number: usize) -> f64 {
        self.numbers.push(number);
        
        if self.numbers.len() > self.period {
            self.numbers.remove(0);
        }
        
        if self.numbers.is_empty() {
            return 0f64;
        }else {
            let sum = self.numbers.iter().fold(0, |acc, x| acc+x);
            return sum as f64 / self.numbers.len() as f64;
        }
    }
}

fn main() {
    for period in [3, 5].iter() {
        println!("Moving average with period {}", period);

        let mut sma = SimpleMovingAverage::new(*period);
        for i in [1, 2, 3, 4, 5, 5, 4, 3, 2, 1].iter() {
            println!("Number: {} | Average: {}", i, sma.add_number(*i));
        }
    }
}
