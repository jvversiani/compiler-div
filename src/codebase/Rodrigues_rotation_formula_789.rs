// Rosetta Code task: Rodrigues’ rotation formula
// Source: https://rosettacode.org/wiki/Rodrigues’_rotation_formula#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
//  Angle         Rotated vector
// -----------------------------------
// 0.0000    (2.5000, -1.5000, 3.0000)
// 0.6283    (3.9018, -0.0529, 1.5076)
// 1.2566    (3.9725, 1.0961, -0.7198)
// 1.8850    (2.6849, 1.5081, -2.8314)
// 2.5133    (0.5309, 1.0258, -4.0207)
// 3.1416    (-1.6667, -0.1667, -3.8333)
// 3.7699    (-3.0685, -1.6138, -2.3409)
// 4.3982    (-3.1391, -2.7628, -0.1135)
// 5.0265    (-1.8516, -3.1748, 1.9981)
// 5.6549    (0.3024, -2.6925, 3.1873)
// 6.2832    (2.5000, -1.5000, 3.0000)
// =======================

use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vector { x, y, z }
    }

    fn unit_vector(&self) -> Self {
        self.scalar_multiply(1.0 / self.dot_product(*self).sqrt())
    }

    fn add(&self, other: &Vector) -> Self {
        Vector::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    fn scalar_multiply(&self, value: f64) -> Self {
        Vector::new(self.x * value, self.y * value, self.z * value)
    }

    fn dot_product(&self, other: Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn cross_product(&self, other: &Vector) -> Self {
        Vector::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    fn rodrigues_rotation(&self, vector: &Vector, angle: f64) -> Self {
        let axis = self.unit_vector();
        vector.scalar_multiply(angle.cos())
            .add(&axis.cross_product(vector).scalar_multiply(angle.sin()))
            .add(&axis.scalar_multiply(axis.dot_product(*vector) * (1.0 - angle.cos())))
    }

    fn display(&self) {
        print!("({:.4}, {:.4}, {:.4})", self.x, self.y, self.z);
    }
}

fn main() {
    let axis = Vector::new(-1.0, 2.0, 1.0);
    let vector = Vector::new(2.5, -1.5, 3.0);

    println!(" Angle         Rotated vector");
    println!("-----------------------------------");
    let mut theta = 0.0;
    while theta <= 2.0 * PI {
        let result = axis.rodrigues_rotation(&vector, theta);
        print!("{:.4}    ", theta);
        result.display();
        println!();
        theta += PI / 5.0;
    }
}
