// Rosetta Code task: Non-decimal radices/Output
// Source: https://rosettacode.org/wiki/Non-decimal_radices/Output#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Binary: 11011110101011011011111011101111
// Binary with 0b prefix: 0b11011110101011011011111011101111
// Octal: 33653337357
// Octal with 0o prefix: 0o33653337357
// Decimal: 3735928559
// Lowercase hexadecimal: deadbeef
// Lowercase hexadecimal with 0x prefix: 0xdeadbeef
// Uppercase hexadecimal: DEADBEEF
// Uppercase hexadecimal with 0x prefix: 0xDEADBEEF
// =======================

fn main() {
    // To render the number as string, use format! macro instead
    println!("Binary: {:b}", 0xdeadbeefu32);
    println!("Binary with 0b prefix: {:#b}", 0xdeadbeefu32);
    println!("Octal: {:o}", 0xdeadbeefu32);
    println!("Octal with 0o prefix: {:#o}", 0xdeadbeefu32);
    println!("Decimal: {}", 0xdeadbeefu32);
    println!("Lowercase hexadecimal: {:x}", 0xdeadbeefu32);
    println!("Lowercase hexadecimal with 0x prefix: {:#x}", 0xdeadbeefu32);
    println!("Uppercase hexadecimal: {:X}", 0xdeadbeefu32);
    println!("Uppercase hexadecimal with 0x prefix: {:#X}", 0xdeadbeefu32);
}
