// Rosetta Code task: Inheritance/Single
// Source: https://rosettacode.org/wiki/Inheritance/Single#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// This is a dog, and therefore also an animal.
// This is a dog, and therefore also an animal.
// =======================

trait Animal {}
trait Cat: Animal {}
trait Dog: Animal {}
trait Lab: Dog {}
trait Collie: Dog {}

struct LabradorRetriever;

// To implement Lab, you must implement its whole supertrait chain:
// Lab requires Dog, and Dog requires Animal.
impl Animal for LabradorRetriever {}
impl Dog for LabradorRetriever {}
impl Lab for LabradorRetriever {}

struct BorderCollie;

impl Animal for BorderCollie {}
impl Dog for BorderCollie {}
impl Collie for BorderCollie {}

// Accepts any Dog (which, by inheritance, is also an Animal).
fn describe_dog<T: Dog>(_dog: &T) {
    println!("This is a dog, and therefore also an animal.");
}

fn main() {
    let lab = LabradorRetriever;
    let collie = BorderCollie;

    describe_dog(&lab);
    describe_dog(&collie);
}
