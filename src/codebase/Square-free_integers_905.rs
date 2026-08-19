// Rosetta Code task: Square-free integers
// Source: https://rosettacode.org/wiki/Square-free_integers#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Square-free numbers between 1 and 145:
// 1 2 3 5 6 7 10 11 13 14 15 17 19 21 22 23 26 29 30 31 33 34 35 37 38 39 41 42 43
// 46 47 51 53 55 57 58 59 61 62 65 66 67 69 70 71 73 74 77 78 79 82 83 85 86 87 89
// 91 93 94 95 97 101 102 103 105 106 107 109 110 111 113 114 115 118 119 122 123 127
// 129 130 131 133 134 137 138 139 141 142 143 145
// Square-free numbers between 1000000000000 and 1000000000145:
// 1000000000001 1000000000002 1000000000003 1000000000005 1000000000006 1000000000007
// 1000000000009 1000000000011 1000000000013 1000000000014 1000000000015 1000000000018
// 1000000000019 1000000000021 1000000000022 1000000000023 1000000000027 1000000000029
// 1000000000030 1000000000031 1000000000033 1000000000037 1000000000038 1000000000039
// 1000000000041 1000000000042 1000000000043 1000000000045 1000000000046 1000000000047
// 1000000000049 1000000000051 1000000000054 1000000000055 1000000000057 1000000000058
// 1000000000059 1000000000061 1000000000063 1000000000065 1000000000066 1000000000067
// 1000000000069 1000000000070 1000000000073 1000000000074 1000000000077 1000000000078
// 1000000000079 1000000000081 1000000000082 1000000000085 1000000000086 1000000000087
// 1000000000090 1000000000091 1000000000093 1000000000094 1000000000095 1000000000097
// 1000000000099 1000000000101 1000000000102 1000000000103 1000000000105 1000000000106
// 1000000000109 1000000000111 1000000000113 1000000000114 1000000000115 1000000000117
// 1000000000118 1000000000119 1000000000121 1000000000122 1000000000123 1000000000126
// 1000000000127 1000000000129 1000000000130 1000000000133 1000000000135 1000000000137
// 1000000000138 1000000000139 1000000000141 1000000000142 1000000000145
// Number of square-free numbers between 1 and 100: 61
// Number of square-free numbers between 1 and 1000: 608
// Number of square-free numbers between 1 and 10000: 6083
// Number of square-free numbers between 1 and 100000: 60794
// Number of square-free numbers between 1 and 1000000: 607926
// =======================

fn square_free(mut n: usize) -> bool {
    if n & 3 == 0 {
        return false;
    }
    let mut p: usize = 3;
    while p * p <= n {
        let mut count = 0;
        while n % p == 0 {
            count += 1;
            if count > 1 {
                return false;
            }
            n /= p;
        }
        p += 2;
    }
    true
}

fn print_square_free_numbers(from: usize, to: usize) {
    println!("Square-free numbers between {} and {}:", from, to);
    let mut line = String::new();
    for i in from..=to {
        if square_free(i) {
            if !line.is_empty() {
                line.push_str(" ");
            }
            line.push_str(&i.to_string());
            if line.len() >= 80 {
                println!("{}", line);
                line.clear();
            }
        }
    }
    if !line.is_empty() {
        println!("{}", line);
    }
}

fn print_square_free_count(from: usize, to: usize) {
    let mut count = 0;
    for i in from..=to {
        if square_free(i) {
            count += 1;
        }
    }
    println!(
        "Number of square-free numbers between {} and {}: {}",
        from, to, count
    )
}

fn main() {
    print_square_free_numbers(1, 145);
    print_square_free_numbers(1000000000000, 1000000000145);
    let mut n: usize = 100;
    while n <= 1000000 {
        print_square_free_count(1, n);
        n *= 10;
    }
}
