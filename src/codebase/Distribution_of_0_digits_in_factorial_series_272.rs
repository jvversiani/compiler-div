// Rosetta Code task: Distribution of 0 digits in factorial series
// Source: https://rosettacode.org/wiki/Distribution_of_0_digits_in_factorial_series#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Mean proportion of zero digits in factorials to 100 is 0.2467531862.
// Mean proportion of zero digits in factorials to 1000 is 0.2035445511.
// Mean proportion of zero digits in factorials to 10000 is 0.1730038482.
// The mean proportion dips permanently below 0.16 at 47332.
// =======================

fn init_zc() -> Vec<usize> {
    let mut zc = vec![0; 1000];
    zc[0] = 3;
    for x in 1..=9 {
        zc[x] = 2;
        zc[10 * x] = 2;
        zc[100 * x] = 2;
        let mut y = 10;
        while y <= 90 {
            zc[y + x] = 1;
            zc[10 * y + x] = 1;
            zc[10 * (y + x)] = 1;
            y += 10;
        }
    }
    zc
}

fn main() {
    let zc = init_zc();
    let mut trail = 1;
    let mut first = 0;
    let mut total: f64 = 0.0;
    let mut rfs = vec![1];

    for f in 2..=50000 {
        let mut carry = 0;
        let mut d999: usize;
        let mut zeroes = (trail - 1) * 3;
        let len = rfs.len();
        let mut j = trail - 1;
        while j < len || carry != 0 {
            if j < len {
                carry += rfs[j] * f;
            }
            d999 = carry % 1000;
            if j < len {
                rfs[j] = d999;
            } else {
                rfs.push(d999);
            }
            zeroes += zc[d999];
            carry /= 1000;
            j += 1;
        }
        while rfs[trail - 1] == 0 {
            trail += 1;
        }
        d999 = rfs[rfs.len() - 1];
        d999 = if d999 < 100 {
            if d999 < 10 {
                2
            } else {
                1
            }
        } else {
            0
        };
        zeroes -= d999;
        let digits = rfs.len() * 3 - d999;
        total += (zeroes as f64) / (digits as f64);
        let ratio = total / (f as f64);
        if ratio >= 0.16 {
            first = 0;
        } else if first == 0 {
            first = f;
        }
        if f == 100 || f == 1000 || f == 10000 {
            println!(
                "Mean proportion of zero digits in factorials to {} is {:.10}.",
                f,
                ratio
            );
        }
    }
    println!(
        "The mean proportion dips permanently below 0.16 at {}.",
        first
    );
}
