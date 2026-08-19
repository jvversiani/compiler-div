// Rosetta Code task: Zeckendorf number representation
// Source: https://rosettacode.org/wiki/Zeckendorf_number_representation#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 1 : 1
// 2 : 10
// 3 : 100
// 4 : 101
// 5 : 1000
// 6 : 1001
// 7 : 1010
// 8 : 10000
// 9 : 10001
// 10 : 10010
// 11 : 10100
// 12 : 10101
// 13 : 100000
// 14 : 100001
// 15 : 100010
// 16 : 100100
// 17 : 100101
// 18 : 101000
// 19 : 101001
// 20 : 101010
// =======================

use std::collections::VecDeque;

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn zeckendorf(num: u32) -> String {
    let mut fibonacci_numbers = VecDeque::new();
    let mut fib_position = 2;
    let mut current_fibonacci_num = fibonacci(fib_position);

    while current_fibonacci_num <= num {
        fibonacci_numbers.push_front(current_fibonacci_num);
        fib_position += 1;
        current_fibonacci_num = fibonacci(fib_position);
    }

    let mut temp = num;
    let mut output = String::new();

    for item in fibonacci_numbers {
        if item <= temp {
            output.push('1');
            temp -= item;
        } else {
            output.push('0');
        }
    }

    output
}

fn main() {
    for i in 1..=20 {
        let zeckendorf_representation = zeckendorf(i);
        println!("{} : {}", i, zeckendorf_representation);
    }
}
