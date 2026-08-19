// Rosetta Code task: Klarner-Rado sequence
// Source: https://rosettacode.org/wiki/Klarner-Rado_sequence#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// The first 100 elements of the Klarner-Rado sequence:
//   1   3   4   7   9  10  13  15  19  21
//  22  27  28  31  39  40  43  45  46  55
//  57  58  63  64  67  79  81  82  85  87
//  91  93  94 111 115 117 118 121 127 129
// 130 135 136 139 159 163 165 166 171 172
// 175 183 187 189 190 193 202 223 231 235
// 237 238 243 244 247 255 256 259 261 262
// 271 273 274 279 280 283 319 327 331 333
// 334 343 345 346 351 352 355 364 367 375
// 379 381 382 387 388 391 405 406 409 418
//
// The 1000th element of Klarner-Rado sequence is 8487
// The 10000th element of Klarner-Rado sequence is 157653
// The 100000th element of Klarner-Rado sequence is 2911581
// The 1000000th element of Klarner-Rado sequence is 54381285
// =======================

fn main() {
    let limit = 1_000_000;
    let klarner_rado = initialise_klarner_rado_sequence(limit);

    println!("The first 100 elements of the Klarner-Rado sequence:");
    for i in 1..=100 {
        print!("{:3}", klarner_rado[i]);
        if i % 10 == 0 {
            println!();
        } else {
            print!(" ");
        }
    }
    println!();

    let mut index = 1_000;
    while index <= limit {
        println!("The {}th element of Klarner-Rado sequence is {}", index, klarner_rado[index]);
        index *= 10;
    }
}

fn initialise_klarner_rado_sequence(limit: usize) -> Vec<usize> {
    let mut result = vec![0; limit + 1];
    let mut i2 = 1;
    let mut i3 = 1;
    let mut m2 = 1;
    let mut m3 = 1;

    for i in 1..=limit {
        let minimum = std::cmp::min(m2, m3);
        result[i] = minimum;
        if m2 == minimum {
            m2 = result[i2] * 2 + 1;
            i2 += 1;
        }
        if m3 == minimum {
            m3 = result[i3] * 3 + 1;
            i3 += 1;
        }
    }
    result
}
