// Rosetta Code task: Events
// Source: https://rosettacode.org/wiki/Events#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// [1] Starting
// [2] Waiting for event
// [1] Sending event
// [2] Received event
// =======================

use std::{sync::mpsc, thread, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("[1] Starting");
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        println!("[2] Waiting for event");
        rx.recv();
        println!("[2] Received event");
    });
    thread::sleep(Duration::from_secs(1));
    println!("[1] Sending event");
    tx.send(())?;
    thread::sleep(Duration::from_secs(1));

    Ok(())
}
