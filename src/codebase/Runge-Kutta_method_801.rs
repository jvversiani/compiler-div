// Rosetta Code task: Runge-Kutta method
// Source: https://rosettacode.org/wiki/Runge-Kutta_method#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// y(0):	1.0000000000		 0E0
// y(1):	1.5624998543		 1.4572189210859676E-7
// y(2):	3.9999990805		 9.194792007782837E-7
// y(3):	10.5624970904		 2.9095624487496252E-6
// y(4):	24.9999937651		 6.234909363911356E-6
// y(5):	52.5624891803		 1.0819697415342944E-5
// y(6):	99.9999834054		 1.659459641700778E-5
// y(7):	175.5624764823		 2.3517728749311573E-5
// y(8):	288.9999684348		 3.156520142510999E-5
// y(9):	451.5624592768		 4.07231603389846E-5
// y(10):	675.9999490167		 5.098329029351589E-5
// =======================

fn runge_kutta4(fx: &dyn Fn(f64, f64) -> f64, x: f64, y: f64, dx: f64) -> f64 {
    let k1 = dx * fx(x, y);
    let k2 = dx * fx(x + dx / 2.0, y + k1 / 2.0);
    let k3 = dx * fx(x + dx / 2.0, y + k2 / 2.0);
    let k4 = dx * fx(x + dx, y + k3);

    y + (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0
}

fn f(x: f64, y: f64) -> f64 {
    x * y.sqrt()
}

fn actual(x: f64) -> f64 {
    (1.0 / 16.0) * (x * x + 4.0).powi(2)
}

fn main() {
    let mut y = 1.0;
    let mut x = 0.0;
    let step = 0.1;
    let max_steps = 101;
    let sample_every_n = 10;

    for steps in 0..max_steps {
        if steps % sample_every_n == 0 {
            println!("y({}):\t{:.10}\t\t {:E}", x, y, actual(x) - y)
        }

        y = runge_kutta4(&f, x, y, step);

        x = ((x * 10.0) + (step * 10.0)) / 10.0;
    }
}
