// Rosetta Code task: 100 doors
// Source: https://rosettacode.org/wiki/100_doors#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Door 1 is open.
// Door 2 is closed.
// Door 3 is closed.
// Door 4 is open.
// Door 5 is closed.
// Door 6 is closed.
// Door 7 is closed.
// Door 8 is closed.
// Door 9 is open.
// Door 10 is closed.
// Door 11 is closed.
// Door 12 is closed.
// Door 13 is closed.
// Door 14 is closed.
// Door 15 is closed.
// Door 16 is open.
// Door 17 is closed.
// Door 18 is closed.
// Door 19 is closed.
// Door 20 is closed.
// Door 21 is closed.
// Door 22 is closed.
// Door 23 is closed.
// Door 24 is closed.
// Door 25 is open.
// Door 26 is closed.
// Door 27 is closed.
// Door 28 is closed.
// Door 29 is closed.
// Door 30 is closed.
// Door 31 is closed.
// Door 32 is closed.
// Door 33 is closed.
// Door 34 is closed.
// Door 35 is closed.
// Door 36 is open.
// Door 37 is closed.
// Door 38 is closed.
// Door 39 is closed.
// Door 40 is closed.
// Door 41 is closed.
// Door 42 is closed.
// Door 43 is closed.
// Door 44 is closed.
// Door 45 is closed.
// Door 46 is closed.
// Door 47 is closed.
// Door 48 is closed.
// Door 49 is open.
// Door 50 is closed.
// Door 51 is closed.
// Door 52 is closed.
// Door 53 is closed.
// Door 54 is closed.
// Door 55 is closed.
// Door 56 is closed.
// Door 57 is closed.
// Door 58 is closed.
// Door 59 is closed.
// Door 60 is closed.
// Door 61 is closed.
// Door 62 is closed.
// Door 63 is closed.
// Door 64 is open.
// Door 65 is closed.
// Door 66 is closed.
// Door 67 is closed.
// Door 68 is closed.
// Door 69 is closed.
// Door 70 is closed.
// Door 71 is closed.
// Door 72 is closed.
// Door 73 is closed.
// Door 74 is closed.
// Door 75 is closed.
// Door 76 is closed.
// Door 77 is closed.
// Door 78 is closed.
// Door 79 is closed.
// Door 80 is closed.
// Door 81 is open.
// Door 82 is closed.
// Door 83 is closed.
// Door 84 is closed.
// Door 85 is closed.
// Door 86 is closed.
// Door 87 is closed.
// Door 88 is closed.
// Door 89 is closed.
// Door 90 is closed.
// Door 91 is closed.
// Door 92 is closed.
// Door 93 is closed.
// Door 94 is closed.
// Door 95 is closed.
// Door 96 is closed.
// Door 97 is closed.
// Door 98 is closed.
// Door 99 is closed.
// Door 100 is open.
// =======================

fn main() {
    let mut door_open = [false; 100];
    for pass in 1..101 {
        let mut door = pass;
        while door <= 100 {
            door_open[door - 1] = !door_open[door - 1];
            door += pass;
        }
    }
    for (i, &is_open) in door_open.iter().enumerate() {
        println!(
            "Door {} is {}.",
            i + 1,
            if is_open { "open" } else { "closed" }
        );
    }
}
