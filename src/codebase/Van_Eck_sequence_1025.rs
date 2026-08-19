// Rosetta Code task: Van Eck sequence
// Source: https://rosettacode.org/wiki/Van_Eck_sequence#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// First 10 terms of the Van Eck sequence:
// 0 0 1 0 2 0 2 2 1 6 
// Terms 991 to 1000 of the Van Eck sequence:
// 4 7 30 25 67 225 488 0 10 136
// =======================

fn van_eck_sequence() -> impl std::iter::Iterator<Item = i32> {
    let mut index = 0;
    let mut last_term = 0;
    let mut last_pos = std::collections::HashMap::new();
    std::iter::from_fn(move || {
        let result = last_term;
        let mut next_term = 0;
        if let Some(v) = last_pos.get_mut(&last_term) {
            next_term = index - *v;
            *v = index;
        } else {
            last_pos.insert(last_term, index);
        }
        last_term = next_term;
        index += 1;
        Some(result)
    })
}

fn main() {
    let mut v = van_eck_sequence().take(1000);
    println!("First 10 terms of the Van Eck sequence:");
    for n in v.by_ref().take(10) {
        print!("{} ", n);
    }
    println!("\nTerms 991 to 1000 of the Van Eck sequence:");
    for n in v.skip(980) {
        print!("{} ", n);
    }
    println!();
}
