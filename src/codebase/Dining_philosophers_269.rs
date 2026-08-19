// Rosetta Code task: Dining philosophers
// Source: https://rosettacode.org/wiki/Dining_philosophers#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Baruch Spinoza is eating.
// Baruch Spinoza is done eating.
// Gilles Deleuze is eating.
// Gilles Deleuze is done eating.
// Karl Marx is eating.
// Karl Marx is done eating.
// Friedrich Nietzsche is eating.
// Friedrich Nietzsche is done eating.
// Michel Foucault is eating.
// Michel Foucault is done eating.
// =======================

use std::thread;
use std::sync::{Mutex, Arc};

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating.", self.name);

        thread::sleep_ms(1000);

        println!("{} is done eating.", self.name);
    }
}

struct Table {
    forks: Vec<Mutex<()>>,
}

fn main() {
    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    let philosophers = vec![
        Philosopher::new("Baruch Spinoza", 0, 1),
        Philosopher::new("Gilles Deleuze", 1, 2),
        Philosopher::new("Karl Marx", 2, 3),
        Philosopher::new("Friedrich Nietzsche", 3, 4),
        Philosopher::new("Michel Foucault", 0, 4),
    ];

    // Spawn and join each philosopher in turn so the output order is
    // deterministic (and the naive fork ordering cannot deadlock).
    for p in philosophers.into_iter() {
        let table = table.clone();

        thread::spawn(move || {
            p.eat(&table);
        })
        .join()
        .unwrap();
    }
}
