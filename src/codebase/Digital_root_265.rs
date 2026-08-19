// Rosetta Code task: Digital root
// Source: https://rosettacode.org/wiki/Digital_root#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 627615 has digital root 9 and additive persistance 2
// 39390 has digital root 6 and additive persistance 2
// 588225 has digital root 3 and additive persistance 2
// 393900588225 has digital root 9 and additive persistance 2
//
// 0x7e0 has digital root 0x6 and additive persistance 0x2
// 0x14e344 has digital root 0xf and additive persistance 0x2
// 0xd60141 has digital root 0xa and additive persistance 0x2
// 0x12343210 has digital root 0x1 and additive persistance 0x2
// =======================

fn sum_digits(mut n: u64, base: u64) -> u64 {
    let mut sum = 0u64;
    while n > 0 {
        sum = sum + (n % base);
        n = n / base;
    }
    sum
}

// Returns tuple of (additive-persistence, digital-root)
fn digital_root(mut num: u64, base: u64) -> (u64, u64) {
    let mut pers = 0;
    while num >= base {
        pers = pers + 1;
        num = sum_digits(num, base);
    }
    (pers, num)
}

fn main() {

    // Test base 10
    let values = [627615u64, 39390u64, 588225u64, 393900588225u64];
    for &value in values.iter() {
        let (pers, root) = digital_root(value, 10);
        println!("{} has digital root {} and additive persistance {}",
                 value,
                 root,
                 pers);
    }

    println!("");

    // Test base 16
    let values_base16 = [0x7e0, 0x14e344, 0xd60141, 0x12343210];
    for &value in values_base16.iter() {
        let (pers, root) = digital_root(value, 16);
        println!("0x{:x} has digital root 0x{:x} and additive persistance 0x{:x}",
                 value,
                 root,
                 pers);
    }
}
