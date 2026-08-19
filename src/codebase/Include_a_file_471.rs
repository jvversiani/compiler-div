// Rosetta Code task: Include a file
// Source: https://rosettacode.org/wiki/Include_a_file#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Hello from the test module!
// =======================

mod test {
    pub fn some_function() {
        println!("Hello from the test module!");
    }
}

fn main() {
    test::some_function();
}