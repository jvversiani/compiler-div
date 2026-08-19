// Rosetta Code task: Inheritance/Multiple
// Source: https://rosettacode.org/wiki/Inheritance/Multiple#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Using a device that is both a camera and a mobile phone.
// =======================

trait Camera {}
trait MobilePhone {}
trait CameraPhone: Camera + MobilePhone {}

struct Smartphone;

// To implement CameraPhone, the type must also implement its supertraits.
impl Camera for Smartphone {}
impl MobilePhone for Smartphone {}
impl CameraPhone for Smartphone {}

// A function that accepts anything that is a CameraPhone. Because CameraPhone
// requires Camera + MobilePhone, the bound gives us all three capabilities.
fn use_device<T: CameraPhone>(_device: &T) {
    println!("Using a device that is both a camera and a mobile phone.");
}

fn main() {
    let phone = Smartphone;
    use_device(&phone);
}
