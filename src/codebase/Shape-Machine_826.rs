// Rosetta Code task: Shape-Machine
// Source: https://rosettacode.org/wiki/Shape-Machine#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
//
// 4 --> 18.428571428571416 to epsilon of 0.0000000000000002220446049250313 after 231 repetitions
//
// =======================

fn converge(start_value: f64) -> (f64, i32) {
    let mut current = start_value;
    let mut previous = start_value + 5.0;
    let mut i: i32 = 0;
    loop {
        if (previous - current).abs() <= f64::EPSILON {
            return (current, i);
        }
        previous = current;
        current += 3.0;
        current *= 0.86;
        i += 1;
    }
}

fn main() {
    let start_value = 4.0;
    let (end_value, iterations) = converge(start_value);
    println!(
        "\n{} --> {} to epsilon of {} after {} repetitions\n",
        start_value,
        end_value,
        f64::EPSILON,
        iterations
    );
}
