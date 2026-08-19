// Rosetta Code task: Leap year
// Source: https://rosettacode.org/wiki/Leap_year#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// The year 2022 is leap? no
// The year 2024 is leap? yes
// =======================

fn is_leap(year: i32) -> bool {
    let factor = |x| year % x == 0;
    factor(4) && (!factor(100) || factor(400))
}

fn main(){
    println!("The year 2022 is leap? {}", if is_leap(2022) {"yes"} else {"no"});
    println!("The year 2024 is leap? {}", if is_leap(2024) {"yes"} else {"no"});
}