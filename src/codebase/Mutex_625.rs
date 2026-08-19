// Rosetta Code task: Mutex
// Source: https://rosettacode.org/wiki/Mutex#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// A
// AA
// AAA
// AAAA
// AAAAA
// AAAAAA
// AAAAAAA
// AAAAAAAA
// AAAAAAAAA
// AAAAAAAAAA
// AAAAAAAAAAA
// AAAAAAAAAAAA
// AAAAAAAAAAAAA
// AAAAAAAAAAAAAA
// AAAAAAAAAAAAAAA
// AAAAAAAAAAAAAAAA
// AAAAAAAAAAAAAAAAA
// AAAAAAAAAAAAAAAAAA
// AAAAAAAAAAAAAAAAAAA
// AAAAAAAAAAAAAAAAAAAA
// AAAAAAAAAAAAAAAAAAAAB
// AAAAAAAAAAAAAAAAAAAABB
// AAAAAAAAAAAAAAAAAAAABBB
// AAAAAAAAAAAAAAAAAAAABBBB
// AAAAAAAAAAAAAAAAAAAABBBBB
// AAAAAAAAAAAAAAAAAAAABBBBBB
// AAAAAAAAAAAAAAAAAAAABBBBBBB
// AAAAAAAAAAAAAAAAAAAABBBBBBBB
// AAAAAAAAAAAAAAAAAAAABBBBBBBBB
// AAAAAAAAAAAAAAAAAAAABBBBBBBBBB
// AAAAAAAAAAAAAAAAAAAABBBBBBBBBBB
// AAAAAAAAAAAAAAAAAAAABBBBBBBBBBBB
// AAAAAAAAAAAAAAAAAAAABBBBBBBBBBBBB
// AAAAAAAAAAAAAAAAAAAABBBBBBBBBBBBBB
// AAAAAAAAAAAAAAAAAAAABBBBBBBBBBBBBBB
// AAAAAAAAAAAAAAAAAAAABBBBBBBBBBBBBBBB
// AAAAAAAAAAAAAAAAAAAABBBBBBBBBBBBBBBBB
// AAAAAAAAAAAAAAAAAAAABBBBBBBBBBBBBBBBBB
// AAAAAAAAAAAAAAAAAAAABBBBBBBBBBBBBBBBBBB
// AAAAAAAAAAAAAAAAAAAABBBBBBBBBBBBBBBBBBBB
// Done: AAAAAAAAAAAAAAAAAAAABBBBBBBBBBBBBBBBBBBB
// =======================

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

fn main() {
    let shared = Arc::new(Mutex::new(String::new()));

    let handle1 = {
        let value = shared.clone();
        thread::spawn(move || {
            for _ in 0..20 {
                thread::sleep(Duration::from_millis(200));
                // The guard is valid until the end of the block
                let mut guard = value.lock().unwrap();
                guard.push_str("A");
                println!("{}", guard);
            }
        })
    };
    // Join the first worker before starting the second so the interleaving
    // (and therefore the printed output) is deterministic.
    handle1.join().ok();

    let handle2 = {
        let value = shared.clone();
        thread::spawn(move || {
            for _ in 0..20 {
                thread::sleep(Duration::from_millis(300));

                {
                    // Making the guard scope explicit here
                    let mut guard = value.lock().unwrap();
                    guard.push_str("B");
                    println!("{}", guard);
                }
            }
        })
    };

    handle2.join().ok();
    shared.lock().ok().map_or((), |it| println!("Done: {}", it));
}
