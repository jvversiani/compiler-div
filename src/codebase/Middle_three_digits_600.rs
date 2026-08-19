// Rosetta Code task: Middle three digits
// Source: https://rosettacode.org/wiki/Middle_three_digits#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// middle_three_digits(123) returned: Success, 123
// middle_three_digits(12345) returned: Success, 234
// middle_three_digits(1234567) returned: Success, 345
// middle_three_digits(987654321) returned: Success, 654
// middle_three_digits(10001) returned: Success, 000
// middle_three_digits(-10001) returned: Success, 000
// middle_three_digits(-123) returned: Success, 123
// middle_three_digits(-100) returned: Success, 100
// middle_three_digits(100) returned: Success, 100
// middle_three_digits(-12345) returned: Success, 234
// middle_three_digits(1) returned: Failure, Too short
// middle_three_digits(2) returned: Failure, Too short
// middle_three_digits(-1) returned: Failure, Too short
// middle_three_digits(-10) returned: Failure, Too short
// middle_three_digits(2002) returned: Failure, Even number of digits
// middle_three_digits(-2002) returned: Failure, Even number of digits
// middle_three_digits(0) returned: Failure, Too short
// =======================

fn middle_three_digits(x: i32) -> Result<String, String> {
    let s: String = x.abs().to_string();
    let len = s.len();
    if len < 3 {
        Err("Too short".into())
    } else if len % 2 == 0 {
        Err("Even number of digits".into())
    } else {
        Ok(s[len/2 - 1 .. len/2 + 2].to_owned())
    }
}

fn print_result(x: i32) {
    print!("middle_three_digits({}) returned: ", x);
    match middle_three_digits(x) {
        Ok(s) => println!("Success, {}", s),
        Err(s) => println!("Failure, {}", s)
    }
}

fn main() {
    let passing = [123, 12345, 1234567, 987654321, 10001, -10001, -123, -100, 100, -12345];
    let failing = [1, 2, -1, -10, 2002, -2002, 0];
    for i in passing.iter() {
        print_result(*i);
    }
    for i in failing.iter() {
        print_result(*i);
    }
}
