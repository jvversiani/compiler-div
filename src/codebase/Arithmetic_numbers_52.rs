// Rosetta Code task: Arithmetic numbers
// Source: https://rosettacode.org/wiki/Arithmetic_numbers#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
//   1   3   5   6   7  11  13  14  15  17 
//  19  20  21  22  23  27  29  30  31  33 
//  35  37  38  39  41  42  43  44  45  46 
//  47  49  51  53  54  55  56  57  59  60 
//  61  62  65  66  67  68  69  70  71  73 
//  77  78  79  83  85  86  87  89  91  92 
//  93  94  95  96  97  99 101 102 103 105 
// 107 109 110 111 113 114 115 116 118 119 
// 123 125 126 127 129 131 132 133 134 135 
// 137 138 139 140 141 142 143 145 147 149 
//
// 1000th arithmetic number is 1361
// Number of composite arithmetic numbers <= 1361: 782
//
// 10000th arithmetic number is 12953
// Number of composite arithmetic numbers <= 12953: 8458
//
// 100000th arithmetic number is 125587
// Number of composite arithmetic numbers <= 125587: 88219
//
// 1000000th arithmetic number is 1228663
// Number of composite arithmetic numbers <= 1228663: 905043
// =======================

fn divisor_count_and_sum(mut n: u32) -> (u32, u32) {
    let mut divisor_count = 1;
    let mut divisor_sum = 1;
    let mut power = 2;
    while (n & 1) == 0 {
        divisor_count += 1;
        divisor_sum += power;
        power <<= 1;
        n >>= 1;
    }
    let mut p = 3;
    while p * p <= n {
        let mut count = 1;
        let mut sum = 1;
        power = p;
        while n % p == 0 {
            count += 1;
            sum += power;
            power *= p;
            n /= p;
        }
        divisor_count *= count;
        divisor_sum *= sum;
        p += 2;
    }
    if n > 1 {
        divisor_count *= 2;
        divisor_sum *= n + 1;
    }
    (divisor_count, divisor_sum)
}

fn main() {
    let mut arithmetic_count = 0;
    let mut composite_count = 0;
    let mut n = 1;
    while arithmetic_count <= 1000000 {
        let (divisor_count, divisor_sum) = divisor_count_and_sum(n);
        if divisor_sum % divisor_count != 0 {
            n += 1;
            continue;
        }
        arithmetic_count += 1;
        if divisor_count > 2 {
            composite_count += 1;
        }
        if arithmetic_count <= 100 {
            print!("{:3} ", n);
            if arithmetic_count % 10 == 0 {
                println!();
            }
        }
        if arithmetic_count == 1000
            || arithmetic_count == 10000
            || arithmetic_count == 100000
            || arithmetic_count == 1000000
        {
            println!("\n{}th arithmetic number is {}", arithmetic_count, n);
            println!(
                "Number of composite arithmetic numbers <= {}: {}",
                n, composite_count
            );
        }
        n += 1;
    }
}
