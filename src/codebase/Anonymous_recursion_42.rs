// Rosetta Code task: Anonymous recursion
// Source: https://rosettacode.org/wiki/Anonymous_recursion#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Fib(-1) = None
// Fib(0) = Some(0)
// Fib(1) = Some(1)
// Fib(2) = Some(1)
// Fib(3) = Some(2)
// Fib(4) = Some(3)
// Fib(5) = Some(5)
// Fib(10) = Some(55)
// =======================

fn fib(n: i64) -> Option<i64> {
    // A function declared inside another function does not pollute the outer namespace.
    fn actual_fib(n: i64) -> i64 {
        if n < 2 {
            n
        } else {
            actual_fib(n - 1) + actual_fib(n - 2)
        }
    }

    if n < 0 {
        None
    } else {
        Some(actual_fib(n))
    }
}

fn main() {
    println!("Fib(-1) = {:?}", fib(-1));
    println!("Fib(0) = {:?}", fib(0));
    println!("Fib(1) = {:?}", fib(1));
    println!("Fib(2) = {:?}", fib(2));
    println!("Fib(3) = {:?}", fib(3));
    println!("Fib(4) = {:?}", fib(4));
    println!("Fib(5) = {:?}", fib(5));
    println!("Fib(10) = {:?}", fib(10));
}

#[test]
fn test_fib() {
    assert_eq!(fib(0).unwrap(), 0);
    assert_eq!(fib(1).unwrap(), 1);
    assert_eq!(fib(2).unwrap(), 1);
    assert_eq!(fib(3).unwrap(), 2);
    assert_eq!(fib(4).unwrap(), 3);
    assert_eq!(fib(5).unwrap(), 5);
    assert_eq!(fib(10).unwrap(), 55);
}

#[test]
fn test_invalid_argument() {
    assert_eq!(fib(-1), None);
}
