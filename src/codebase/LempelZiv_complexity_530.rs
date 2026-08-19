// Rosetta Code task: Lempel–Ziv complexity
// Source: https://rosettacode.org/wiki/Lempel–Ziv_complexity#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// String                                               LZ Complexity
// ==================================================================
// AZSEDRFTGYGUJIJOKB                                   16
// ABCABCABCABCABCABC                                   4
// 111011111001111011111001                             6
// 101001010010111110                                   5
// 1001111011000010                                     6
// 1010101010                                           3
// 1010101010101010                                     3
// 1001111011000010000010                               7
// 100111101100001000001010                             8
// 0001101001000101                                     6
// 1111111                                              2
// 0001                                                 2
// 010                                                  3
// 1                                                    1
//                                                      0
// 01011010001101110010                                 7
// ABCDEFGHIJKLMNOPQRSTUVWXYZ                           26
// HELLO WORLD! HELLO WORLD! HELLO WORLD! HELLO WORLD!  11
// =======================

fn lempel_ziv_complexity(sequence: &str) -> usize {
    if sequence.is_empty() {
        return 0;
    }

    let mut complexity = 0;
    let mut pointer = 0;
    let n = sequence.len();
    
    while pointer < n {
        complexity += 1;
        let mut k = 1;
        while pointer + k <= n {
            let current_sub = &sequence[pointer..pointer + k];
//          let search_window = &sequence[0..pointer];
            let search_window = &sequence[0..pointer+k-1];
            
            if search_window.contains(current_sub) {
                k += 1;
            } else {
                pointer += k;
                k = 0; // bugfix part 2, as per talk page
                break;
            }
        }
        if pointer + k > n {
            pointer = n;
        }
    }
    complexity
}

fn main() {
    const TESTS: &[&str] = &[
        "AZSEDRFTGYGUJIJOKB",
        "ABCABCABCABCABCABC",
        "111011111001111011111001",
        "101001010010111110",
        "1001111011000010",
        "1010101010",
        "1010101010101010",
        "1001111011000010000010",
        "100111101100001000001010",
        "0001101001000101",
        "1111111",
        "0001",
        "010",
        "1",
        "",
        "01011010001101110010",
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        "HELLO WORLD! HELLO WORLD! HELLO WORLD! HELLO WORLD!",
    ];
    
    println!("{:<52} {}", "String", "LZ Complexity");
    println!("{}", "=".repeat(66));
    
    for test in TESTS {
        let lz_complexity = lempel_ziv_complexity(test);
        println!("{:<52} {}", test, lz_complexity);
    }
}
