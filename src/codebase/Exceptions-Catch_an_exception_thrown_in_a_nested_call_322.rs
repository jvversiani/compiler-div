// Rosetta Code task: Exceptions/Catch an exception thrown in a nested call
// Source: https://rosettacode.org/wiki/Exceptions/Catch_an_exception_thrown_in_a_nested_call#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Caught U0 in foo: 42
// Error: U1("This will be returned from main")
// =======================

#[derive(Debug)]
enum U {
    U0(i32),
    U1(String),
}

fn baz(i: u8) -> Result<(), U> {
    match i {
        0 => Err(U::U0(42)),
        1 => Err(U::U1("This will be returned from main".into())),
        _ => Ok(()),
    }
}

fn bar(i: u8) -> Result<(), U> {
    baz(i)
}

fn foo() -> Result<(), U> {
    for i in 0..2 {
        match bar(i) {
            Ok(()) => {},
            Err(U::U0(n)) => println!("Caught U0 in foo: {}", n),
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

fn main() {
    match foo() {
        Ok(()) => {}
        Err(e) => println!("Error: {:?}", e),
    }
}
