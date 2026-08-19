// Rosetta Code task: Fusc sequence
// Source: https://rosettacode.org/wiki/Fusc_sequence#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// First 61 fusc numbers:
// 0 1 1 2 1 3 2 3 1 4 3 5 2 5 3 4 1 5 4 7 3 8 5 7 2 7 5 8 3 7 4 5 1 6 5 9 4 11 7 10 3 11 8 13 5 12 7 9 2 9 7 12 5 13 8 11 3 10 7 11 4 
// Fusc numbers up to 10000 that are longer than any previous one:
// index = 0, fusc number = 0
// index = 37, fusc number = 11
// index = 1173, fusc number = 108
// =======================

fn fusc_sequence() -> impl std::iter::Iterator<Item = u32> {
    let mut sequence = vec![0, 1];
    let mut n = 0;
    std::iter::from_fn(move || {
        if n > 1 {
            sequence.push(match n % 2 {
                0 => sequence[n / 2],
                _ => sequence[(n - 1) / 2] + sequence[(n + 1) / 2],
            });
        }
        let result = sequence[n];
        n += 1;
        Some(result)
    })
}

fn main() {
    println!("First 61 fusc numbers:");
    for n in fusc_sequence().take(61) {
        print!("{} ", n)
    }
    println!();

    let limit = 10000;
    println!(
        "Fusc numbers up to {} that are longer than any previous one:",
        limit
    );
    let mut max = 0;
    for (index, n) in fusc_sequence().take(limit).enumerate() {
        if n >= max {
            max = std::cmp::max(10, max * 10);
            println!("index = {}, fusc number = {}", index, n);
        }
    }
}
