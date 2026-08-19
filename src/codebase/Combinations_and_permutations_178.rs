// Rosetta Code task: Combinations and permutations
// Source: https://rosettacode.org/wiki/Combinations_and_permutations#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// P(12,1) = 12
// P(12,2) = 132
// P(12,3) = 1320
// P(12,4) = 11880
// P(12,5) = 95040
// P(12,6) = 665280
// P(12,7) = 3991680
// P(12,8) = 19958400
// P(12,9) = 79833600
// P(12,10) = 239500800
// P(12,11) = 479001600
// C(60,10) = 75394027566
// C(60,20) = 4191844505805495
// C(60,30) = 118264581564861420
// C(60,40) = 4191844505805495
// C(60,50) = 75394027566
// =======================

fn perm(n: f64, k: f64) -> f64 {
    let mut result: f64 = 1.0;
    let mut i: f64 = 0.0;

    while i < k {
        result *= n - i;
        i += 1.0;
    }

    result
}

fn comb(n: f64, k: f64) -> f64 {
    perm(n, k) / perm(k, k)
}

fn main() {
    const P: f64 = 12.0;
    const C: f64 = 60.0;

    let mut j: f64 = 1.0;
    let mut k: f64 = 10.0;

    while j < P {
        println!("P({},{}) = {}", P, j, perm(P, j).floor());
        j += 1.0;
    }

    while k < C {
        println!("C({},{}) = {}", C, k, comb(C, k).floor());
        k += 10.0;
    }

}
