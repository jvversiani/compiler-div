// Rosetta Code task: Sort an array of composite structures
// Source: https://rosettacode.org/wiki/Sort_an_array_of_composite_structures#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Alice  : Sales
// David  : Manager
// Henry  : Admin
// Joanna : Director
// Juan   : Admin
// Tim    : Sales
// =======================

use std::cmp::Ordering;

#[derive(Debug)]
struct Employee {
    name: String,
    category: String,
}

impl Employee {
    fn new(name: &str, category: &str) -> Self {
        Employee {
            name: name.into(),
            category: category.into(),
        }
    }
}

impl PartialEq for Employee {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Employee {}

impl PartialOrd for Employee {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Employee {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

fn main() {
    let mut employees = vec![
        Employee::new("David", "Manager"),
        Employee::new("Alice", "Sales"),
        Employee::new("Joanna", "Director"),
        Employee::new("Henry", "Admin"),
        Employee::new("Tim", "Sales"),
        Employee::new("Juan", "Admin"),
    ];
    employees.sort();
    for e in employees {
        println!("{:<6} : {}", e.name, e.category);
    }
}
