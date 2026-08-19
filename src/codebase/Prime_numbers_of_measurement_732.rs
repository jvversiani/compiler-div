// Rosetta Code task: Prime numbers of measurement
// Source: https://rosettacode.org/wiki/Prime_numbers_of_measurement#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// First 100:
//   1   2   4   5   8  10  14  15  16  21 
//  22  25  26  28  33  34  35  36  38  40 
//  42  46  48  49  50  53  57  60  62  64 
//  65  70  77  80  81  83  85  86  90  91 
//  92 100 104 107 108 116 119 124 127 132 
// 133 137 141 144 145 148 150 151 154 158 
// 159 163 165 172 173 174 175 178 180 182 
// 184 187 188 195 198 201 206 207 208 213 
// 219 221 222 226 228 231 233 236 241 242 
// 245 247 248 253 256 262 266 268 272 274 
// =======================

fn seq(len: usize) -> Vec<usize> {
    let mut ruler = vec![];

    for i in 1.. {
        let mut intervals = (0..ruler.len()).flat_map(|size| ruler.windows(size + 1));

        if intervals.any(|window| i == window.iter().sum()) {
            continue;
        } else {
            ruler.push(i);
        }

        if ruler.len() == len {
            break;
        }
    }

    ruler
}

fn main() {
    let len = if cfg!(feature = "stretch_goal") {
        1000
    } else {
        100
    };
    let s = seq(len);

    println!("First 100:");

    for row in s.chunks(10).take(10) {
        for col in row {
            print!("{col:>3} ");
        }
        println!();
    }

    #[cfg(feature = "stretch_goal")]
    println!("\nOne Thousandth: {}", s[999]);
}
