// Rosetta Code task: Pythagorean triples
// Source: https://rosettacode.org/wiki/Pythagorean_triples#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Primitive triples below 100 : 7
// Primitive triples below 1000 : 70
// Primitive triples below 10000 : 703
// Primitive triples below 100000 : 7026
// Triples below 100 : 17
// Triples below 1000 : 325
// Triples below 10000 : 4858
// Triples below 100000 : 64741
// =======================

use std::thread;

fn f1(a: i64, b: i64, c: i64, d: i64) -> u64 {
    let mut primitive_count = 0u64;
    for triangle in [
        [a - 2 * b + 2 * c, 2 * a - b + 2 * c, 2 * a - 2 * b + 3 * c],
        [a + 2 * b + 2 * c, 2 * a + b + 2 * c, 2 * a + 2 * b + 3 * c],
        [2 * b + 2 * c - a, b + 2 * c - 2 * a, 2 * b + 3 * c - 2 * a],
    ]
        .iter()
    {
        let l = triangle[0] + triangle[1] + triangle[2];
        if l > d {
            continue;
        }
        primitive_count += 1 + f1(triangle[0], triangle[1], triangle[2], d);
    }
    primitive_count
}

fn f2(a: i64, b: i64, c: i64, d: i64) -> u64 {
    let mut triplet_count = 0u64;
    for triangle in [
        [a - 2 * b + 2 * c, 2 * a - b + 2 * c, 2 * a - 2 * b + 3 * c],
        [a + 2 * b + 2 * c, 2 * a + b + 2 * c, 2 * a + 2 * b + 3 * c],
        [2 * b + 2 * c - a, b + 2 * c - 2 * a, 2 * b + 3 * c - 2 * a],
    ]
        .iter()
    {
        let l = triangle[0] + triangle[1] + triangle[2];
        if l > d {
            continue;
        }
        triplet_count += (d / l) as u64 + f2(triangle[0], triangle[1], triangle[2], d);
    }
    triplet_count
}

fn main() {
    let new_th_1 = thread::Builder::new()
        .stack_size(32 * 1024 * 1024)
        .spawn(move || {
            let mut lines = Vec::new();
            let mut i: i64 = 100;
            while i <= 100_000 {
                lines.push(format!("Primitive triples below {} : {}", i, f1(3, 4, 5, i) + 1));
                i *= 10;
            }
            lines
        })
        .unwrap();

    let new_th_2 = thread::Builder::new()
        .stack_size(32 * 1024 * 1024)
        .spawn(move || {
            let mut lines = Vec::new();
            let mut i: i64 = 100;
            while i <= 100_000 {
                lines.push(format!("Triples below {} : {}", i, f2(3, 4, 5, i) + (i / 12) as u64));
                i *= 10;
            }
            lines
        })
        .unwrap();

    let primitive_lines = new_th_1.join().unwrap();
    let triples_lines = new_th_2.join().unwrap();

    // Fixed print order: all primitive lines, then all triples lines.
    for line in &primitive_lines {
        println!("{}", line);
    }
    for line in &triples_lines {
        println!("{}", line);
    }
}
