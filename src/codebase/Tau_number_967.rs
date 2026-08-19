// Rosetta Code task: Tau number
// Source: https://rosettacode.org/wiki/Tau_number#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// First 100 Tau numbers:
//    1,   2,   8,   9,  12,  18,  24,  36,  40,  56,  60,  72,  80,  84,  88,  96, 104, 108, 128, 132
//  136, 152, 156, 180, 184, 204, 225, 228, 232, 240, 248, 252, 276, 288, 296, 328, 344, 348, 360, 372
//  376, 384, 396, 424, 441, 444, 448, 450, 468, 472, 480, 488, 492, 504, 516, 536, 560, 564, 568, 584
//  600, 612, 625, 632, 636, 640, 664, 672, 684, 708, 712, 720, 732, 776, 792, 804, 808, 824, 828, 852
//  856, 864, 872, 876, 880, 882, 896, 904, 936, 948, 972, 996,1016,1040,1044,1048,1056,1068,1089,1096
// =======================

/// Gets all divisors of a number, including itself
fn get_divisors(n: u32) -> Vec<u32> {
    let mut results = Vec::new();

    for i in 1..(n / 2 + 1) {
        if n % i == 0 {
            results.push(i);
        }
    }
    results.push(n);
    results
}

fn is_tau_number(i: u32) -> bool {
    0 == i % get_divisors(i).len() as u32
}

fn main() {
    println!("First 100 Tau numbers:");
    let mut counter: u32 = 0;
    let mut i: u32 = 1;
    while counter < 100 {
        if is_tau_number(i) {
            print!("{:>4}", i);
            counter += 1;
            print!("{}", if counter % 20 == 0 { "\n" } else { "," });
        }
        i += 1;
    }
}
