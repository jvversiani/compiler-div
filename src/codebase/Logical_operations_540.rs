// Rosetta Code task: Logical operations
// Source: https://rosettacode.org/wiki/Logical_operations#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// true and true -> true
// true or true -> true
// true xor true -> false
// not true -> false
//
// true and false -> false
// true or false -> true
// true xor false -> true
// not true -> false
//
// false and true -> false
// false or true -> true
// false xor true -> true
// not false -> true
//
// false and false -> false
// false or false -> false
// false xor false -> false
// not false -> true
//
// =======================

fn boolean_ops(a: bool, b: bool) {
    println!("{} and {} -> {}", a, b, a && b);
    println!("{} or {} -> {}", a, b, a || b);
    println!("{} xor {} -> {}", a, b, a ^ b);
    println!("not {} -> {}\n", a, !a);
}

fn main() {
    boolean_ops(true, true);
    boolean_ops(true, false);
    boolean_ops(false, true);
    boolean_ops(false, false)
}
