// Rosetta Code task: Roman numerals/Encode
// Source: https://rosettacode.org/wiki/Roman_numerals/Encode#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 2014 = MMXIV
// 1999 = MCMXCIX
//   25 = XXV
// 1666 = MDCLXVI
// 3888 = MMMDCCCLXXXVIII
// =======================

struct RomanNumeral {
    symbol: &'static str,
    value: u32
}

const NUMERALS: [RomanNumeral; 13] = [
    RomanNumeral {symbol: "M",  value: 1000},
    RomanNumeral {symbol: "CM", value: 900},
    RomanNumeral {symbol: "D",  value: 500},
    RomanNumeral {symbol: "CD", value: 400},
    RomanNumeral {symbol: "C",  value: 100},
    RomanNumeral {symbol: "XC", value: 90},
    RomanNumeral {symbol: "L",  value: 50},
    RomanNumeral {symbol: "XL", value: 40},
    RomanNumeral {symbol: "X",  value: 10},
    RomanNumeral {symbol: "IX", value: 9},
    RomanNumeral {symbol: "V",  value: 5},
    RomanNumeral {symbol: "IV", value: 4},
    RomanNumeral {symbol: "I",  value: 1}
];

fn to_roman(mut number: u32) -> String {
    let mut min_numeral = String::new();
    for numeral in NUMERALS.iter() {
        while numeral.value <= number {
            min_numeral = min_numeral + numeral.symbol;
            number -= numeral.value;
        }
    }
    min_numeral
}

fn main() {
    let nums = [2014, 1999, 25, 1666, 3888];
    for &n in nums.iter() {
        // 4 is minimum printing width, for alignment
        println!("{:2$} = {}", n, to_roman(n), 4);
    }
}
