// Rosetta Code task: Loops/For with a specified step
// Source: https://rosettacode.org/wiki/Loops/For_with_a_specified_step#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 2468who do we appreciate?!
// =======================

fn main() {
  for i in (2..=8).step_by(2) {
    print!("{}", i);
  }
  println!("who do we appreciate?!");
}
