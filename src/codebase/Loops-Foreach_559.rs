// Rosetta Code task: Loops/Foreach
// Source: https://rosettacode.org/wiki/Loops/Foreach#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 1
// 2
// 3
// 4
// 5
// =======================

fn main(){
    let collection = vec![1,2,3,4,5];
    for elem in collection {
        println!("{}", elem);
    }
}
