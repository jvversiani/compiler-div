// Rosetta Code task: Angles (geometric), normalization and conversion
// Source: https://rosettacode.org/wiki/Angles_(geometric),_normalization_and_conversion#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Angle        Unit         Degrees      Gradians     Mils         Radians
// -2           Degrees      -2.0000      -2.2222      -35.5556     -0.0349
// -1           Degrees      -1.0000      -1.1111      -17.7778     -0.0175
// 0            Degrees      0.0000       0.0000       0.0000       0.0000
// 1            Degrees      1.0000       1.1111       17.7778      0.0175
// 2            Degrees      2.0000       2.2222       35.5556      0.0349
// 6.2831853    Degrees      6.2832       6.9813       111.7011     0.1097
// 16           Degrees      16.0000      17.7778      284.4444     0.2793
// 57.2957795   Degrees      57.2958      63.6620      1018.5916    1.0000
// 359          Degrees      359.0000     398.8889     6382.2222    6.2657
// 399          Degrees      39.0000      43.3333      693.3333     0.6807
// 6399         Degrees      279.0000     310.0000     4960.0000    4.8695
// 1000000      Degrees      280.0000     311.1111     4977.7778    4.8869
//
// Angle        Unit         Degrees      Gradians     Mils         Radians
// -2           Gradians     -1.8000      -2.0000      -32.0000     -0.0314
// -1           Gradians     -0.9000      -1.0000      -16.0000     -0.0157
// 0            Gradians     0.0000       0.0000       0.0000       0.0000
// 1            Gradians     0.9000       1.0000       16.0000      0.0157
// 2            Gradians     1.8000       2.0000       32.0000      0.0314
// 6.2831853    Gradians     5.6549       6.2832       100.5310     0.0987
// 16           Gradians     14.4000      16.0000      256.0000     0.2513
// 57.2957795   Gradians     51.5662      57.2958      916.7325     0.9000
// 359          Gradians     323.1000     359.0000     5744.0000    5.6392
// 399          Gradians     359.1000     399.0000     6384.0000    6.2675
// 6399         Gradians     359.1000     399.0000     6384.0000    6.2675
// 1000000      Gradians     0.0000       0.0000       0.0000       0.0000
//
// Angle        Unit         Degrees      Gradians     Mils         Radians
// -2           Mils         -0.1125      -0.1250      -2.0000      -0.0020
// -1           Mils         -0.0563      -0.0625      -1.0000      -0.0010
// 0            Mils         0.0000       0.0000       0.0000       0.0000
// 1            Mils         0.0563       0.0625       1.0000       0.0010
// 2            Mils         0.1125       0.1250       2.0000       0.0020
// 6.2831853    Mils         0.3534       0.3927       6.2832       0.0062
// 16           Mils         0.9000       1.0000       16.0000      0.0157
// 57.2957795   Mils         3.2229       3.5810       57.2958      0.0562
// 359          Mils         20.1938      22.4375      359.0000     0.3524
// 399          Mils         22.4438      24.9375      399.0000     0.3917
// 6399         Mils         359.9438     399.9375     6399.0000    6.2822
// 1000000      Mils         90.0000      100.0000     1600.0000    1.5708
//
// Angle        Unit         Degrees      Gradians     Mils         Radians
// -2           Radians      -114.5916    -127.3240    -2037.1833   -2.0000
// -1           Radians      -57.2958     -63.6620     -1018.5916   -1.0000
// 0            Radians      0.0000       0.0000       0.0000       0.0000
// 1            Radians      57.2958      63.6620      1018.5916    1.0000
// 2            Radians      114.5916     127.3240     2037.1833    2.0000
// 6.2831853    Radians      360.0000     400.0000     6400.0000    6.2832
// 16           Radians      196.7325     218.5916     3497.4662    3.4336
// 57.2957795   Radians      42.8063      47.5626      761.0018     0.7471
// 359          Radians      49.1848      54.6498      874.3972     0.8584
// 399          Radians      181.0160     201.1289     3218.0627    3.1593
// 6399         Radians      155.6931     172.9923     2767.8774    2.7174
// 1000000      Radians      339.5131     377.2368     6035.7881    5.9256
// =======================

use std::{
    marker::PhantomData,
    f64::consts::PI,
};

pub trait AngleUnit: Copy {
    const TURN: f64;
    const NAME: &'static str;
}

macro_rules! unit {
    ($name:ident, $value:expr, $string:expr) => (
        #[derive(Debug, Copy, Clone)]
        struct $name;
        impl AngleUnit for $name {
            const TURN: f64 = $value;
            const NAME: &'static str = $string;
        }
    );
}

unit!(Degrees,  360.0,      "Degrees");
unit!(Radians,  PI * 2.0,   "Radians");
unit!(Gradians, 400.0,      "Gradians");
unit!(Mils,     6400.0,     "Mils");

#[derive(Copy, Clone, PartialEq, PartialOrd)]
struct Angle<T: AngleUnit>(f64, PhantomData<T>);

impl<T: AngleUnit> Angle<T> {
    pub fn new(val: f64) -> Self {
        Self(val, PhantomData)
    }

    pub fn normalize(self) -> Self {
        Self(self.0 % T::TURN, PhantomData)
    }

    pub fn val(self) -> f64 {
        self.0
    }

    pub fn convert<U: AngleUnit>(self) -> Angle<U> {
        Angle::new(self.0 * U::TURN / T::TURN)
    }

    pub fn name(self) -> &'static str {
        T::NAME
    }
}

fn print_angles<T: AngleUnit>() {
    let angles = [-2.0, -1.0, 0.0, 1.0, 2.0, 6.2831853, 16.0, 57.2957795, 359.0, 399.0, 6399.0, 1000000.0];
    println!("{:<12} {:<12} {:<12} {:<12} {:<12} {:<12}", "Angle", "Unit", "Degrees", "Gradians", "Mils", "Radians");

    for &angle in &angles {
        let deg = Angle::<T>::new(angle).normalize();
        println!("{:<12} {:<12} {:<12.4} {:<12.4} {:<12.4} {:<12.4}",
                 angle,
                 deg.name(),
                 deg.convert::<Degrees>().val(),
                 deg.convert::<Gradians>().val(),
                 deg.convert::<Mils>().val(),
                 deg.convert::<Radians>().val(),
        );
    }

    println!();
}

fn main() {
    print_angles::<Degrees>();
    print_angles::<Gradians>();
    print_angles::<Mils>();
    print_angles::<Radians>();
}
