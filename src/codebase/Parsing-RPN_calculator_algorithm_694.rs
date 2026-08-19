// Rosetta Code task: Parsing/RPN calculator algorithm
// Source: https://rosettacode.org/wiki/Parsing/RPN_calculator_algorithm#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// input operation stack
//   3   push      [3.0]
//   4   push      [3.0, 4.0]
//   2   push      [3.0, 4.0, 2.0]
//   *   calculate [3.0, 8.0]
//   1   push      [3.0, 8.0, 1.0]
//   5   push      [3.0, 8.0, 1.0, 5.0]
//   -   calculate [3.0, 8.0, -4.0]
//   2   push      [3.0, 8.0, -4.0, 2.0]
//   3   push      [3.0, 8.0, -4.0, 2.0, 3.0]
//   ^   calculate [3.0, 8.0, -4.0, 8.0]
//   ^   calculate [3.0, 8.0, 65536.0]
//   /   calculate [3.0, 0.0001220703125]
//   +   calculate [3.0001220703125]
//
// result: 3.0001220703125
// =======================

fn rpn(text: &str) -> f64 {
    let tokens = text.split_whitespace();
    let mut stack: Vec<f64> = vec![];
    println!("input operation stack");

    for token in tokens {
        print!("{:^5} ", token);
        match token.parse() {
            Ok(num) => {
                stack.push(num);
                println!("push      {:?}", stack);
            }
            Err(_) => {
                match token {
                    "+" => {
                        let b = stack.pop().expect("missing first operand");
                        let a = stack.pop().expect("missing second operand");
                        stack.push(a + b);
                    }
                    "-" => {
                        let b = stack.pop().expect("missing first operand");
                        let a = stack.pop().expect("missing second operand");
                        stack.push(a - b);
                    }
                    "*" => {
                        let b = stack.pop().expect("missing first operand");
                        let a = stack.pop().expect("missing second operand");
                        stack.push(a * b);
                    }
                    "/" => {
                        let b = stack.pop().expect("missing first operand");
                        let a = stack.pop().expect("missing second operand");
                        stack.push(a / b);
                    }
                    "^" => {
                        let b = stack.pop().expect("missing first operand");
                        let a = stack.pop().expect("missing second operand");
                        stack.push(a.powf(b));
                    }
                    _ => panic!("unknown operator {}", token),
                }
                println!("calculate {:?}", stack);
            }
        }
    }

    stack.pop().unwrap_or(0.0)
}

fn main() {
    let text = "3 4 2 * 1 5 - 2 3 ^ ^ / +";

    println!("\nresult: {}", rpn(text));
}
