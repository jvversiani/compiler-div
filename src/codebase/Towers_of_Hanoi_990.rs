// Rosetta Code task: Towers of Hanoi
// Source: https://rosettacode.org/wiki/Towers_of_Hanoi#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Move disk from pole 1 to pole 3
// Move disk from pole 1 to pole 2
// Move disk from pole 3 to pole 2
// Move disk from pole 1 to pole 3
// Move disk from pole 2 to pole 1
// Move disk from pole 2 to pole 3
// Move disk from pole 1 to pole 3
// Move disk from pole 1 to pole 2
// Move disk from pole 3 to pole 2
// Move disk from pole 3 to pole 1
// Move disk from pole 2 to pole 1
// Move disk from pole 3 to pole 2
// Move disk from pole 1 to pole 3
// Move disk from pole 1 to pole 2
// Move disk from pole 3 to pole 2
// =======================

fn move_(n: i32, from: i32, to: i32, via: i32) {
    if n > 0 {
        move_(n - 1, from, via, to);
        println!("Move disk from pole {} to pole {}", from, to);
        move_(n - 1, via, to, from);
    }
}

fn main() {
    move_(4, 1,2,3);
}
