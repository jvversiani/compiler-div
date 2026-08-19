// Rosetta Code task: Left factorials
// Source: https://rosettacode.org/wiki/Left_factorials#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// !0 = 0
// !1 = 1
// !2 = 2
// !3 = 4
// !4 = 10
// !5 = 34
// !6 = 154
// !7 = 874
// !8 = 5914
// !9 = 46234
// !10 = 409114
// !20 = 128425485935180314
// !30 = 9157958657951075573395300940314
// !40 = 20935051082417771847631371547939998232420940314
// !50 = 620960027832821612639424806694551108812720525606160920420940314
// !60 = 141074930726669571000530822087000522211656242116439949000980378746128920420940314
// !70 = 173639511802987526699717162409282876065556519849603157850853034644815111221599509216528920420940314
// !80 = 906089587987695346534516804650290637694024830011956365184327674619752094289696314882008531991840922336528920420940314
// !90 = 16695570072624210767034167688394623360733515163575864136345910335924039962404869510225723072235842668787507993136908442336528920420940314
// !100 = 942786239765826579160595268206839381354754349601050974345395410407078230249590414458830117442618180732911203520208889371641659121356556442336528920420940314
// !110 = 145722981061585297004706728001906071948635199234860720988658042536179281328615541936083296163475394237524337422204397431927131629058103519228197429698252556442336528920420940314
// !1000 has 2565 digits.
// !2000 has 5733 digits.
// !3000 has 9128 digits.
// !4000 has 12670 digits.
// !5000 has 16322 digits.
// !6000 has 20062 digits.
// !7000 has 23875 digits.
// !8000 has 27749 digits.
// !9000 has 31678 digits.
// !10000 has 35656 digits.
// =======================

#[cfg(target_pointer_width = "64")]
type USingle = u32;
#[cfg(target_pointer_width = "64")]
type UDouble = u64;
#[cfg(target_pointer_width = "64")]
const WORD_LEN: i32 = 32;

#[cfg(not(target_pointer_width = "64"))]
type USingle = u16;
#[cfg(not(target_pointer_width = "64"))]
type UDouble = u32;
#[cfg(not(target_pointer_width = "64"))]
const WORD_LEN: i32 = 16;

use std::cmp;

#[derive(Debug,Clone)]
struct BigNum {
    // rep_.size() == 0 if and only if the value is zero.
    // Otherwise, the word rep_[0] keeps the least significant bits.
    rep_: Vec<USingle>,
}

impl BigNum {
    pub fn new(n: USingle) -> BigNum {
        let mut result = BigNum { rep_: vec![] };
        if n > 0 { result.rep_.push(n); }
        result
    }
    pub fn equals(&self, n: USingle) -> bool {
        if n == 0 { return self.rep_.is_empty() }
        if self.rep_.len() > 1 { return false }
        self.rep_[0] == n
    }
    pub fn add_big(&self, addend: &BigNum) -> BigNum {
        let mut result = BigNum::new(0);
        let mut sum = 0 as UDouble;
        let sz1 = self.rep_.len();
        let sz2 = addend.rep_.len();
        for i in 0..cmp::max(sz1, sz2) {
            if i < sz1 { sum += self.rep_[i] as UDouble }
            if i < sz2 { sum += addend.rep_[i] as UDouble }
            result.rep_.push(sum as USingle);
            sum >>= WORD_LEN;
        }
        if sum > 0 { result.rep_.push(sum as USingle) }
        result
    }
    pub fn multiply(&self, factor: USingle) -> BigNum {
        let mut result = BigNum::new(0);
        let mut product = 0 as UDouble;
        for i in 0..self.rep_.len() {
            product += self.rep_[i] as UDouble * factor as UDouble;
            result.rep_.push(product as USingle);
            product >>= WORD_LEN;
        }
        if product > 0 {
            result.rep_.push(product as USingle);
        }
        result
    }
    pub fn divide(&self, divisor: USingle, quotient: &mut BigNum,
        remainder: &mut USingle) {
        quotient.rep_.truncate(0);
        let mut dividend: UDouble;
        *remainder = 0;
        for i in 0..self.rep_.len() {
            let j = self.rep_.len() - 1 - i;
            dividend = ((*remainder as UDouble) << WORD_LEN)
                + self.rep_[j] as UDouble;
            let quo = (dividend / divisor as UDouble) as USingle;
            *remainder = (dividend % divisor as UDouble) as USingle;
            if quo > 0 || j < self.rep_.len() - 1 {
                quotient.rep_.push(quo);
            }
        }
        quotient.rep_.reverse();
    }
    fn to_string(&self) -> String {
        let mut rep = String::new();
        let mut dividend = (*self).clone();
        let mut remainder = 0 as USingle;
        let mut quotient = BigNum::new(0);
        loop {
            dividend.divide(10, &mut quotient, &mut remainder);
            rep.push(('0' as USingle + remainder) as u8 as char);
            if quotient.equals(0) { break; }
            dividend = quotient.clone();
        }
        rep.chars().rev().collect::<String>()
    }
}

use std::fmt;
impl fmt::Display for BigNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

fn lfact(n: USingle) -> BigNum {
    let mut result = BigNum::new(0);
    let mut f = BigNum::new(1);
    for k in 1 as USingle..n + 1 {
        result = result.add_big(&f);
        f = f.multiply(k);
    }
    result
}

fn main() {
    for i in 0..11 {
        println!("!{} = {}", i, lfact(i));
    }
    for i in 2..12 {
        let j = i * 10;
        println!("!{} = {}", j, lfact(j));
    }
    for i in 1..11 {
        let j = i * 1000;
        println!("!{} has {} digits.", j, lfact(j).to_string().len());
    }
}
