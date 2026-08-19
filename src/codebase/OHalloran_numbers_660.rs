// Rosetta Code task: O'Halloran numbers
// Source: https://rosettacode.org/wiki/O'Halloran_numbers#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Values larger than 6 and less than 1_000 which cannot be the surface area of a cuboid:
// 8 12 20 36 44 60 84 116 140 156 204 260 380 420 660 924
// =======================

fn main() {
    const MAXIMUM_AREA: u32 = 1_000;
    const HALF_MAXIMUM_AREA: u32 = MAXIMUM_AREA / 2;

    let mut ohalloran_numbers = vec![true; HALF_MAXIMUM_AREA as usize];

    for length in 1..MAXIMUM_AREA {
        for width in 1..HALF_MAXIMUM_AREA {
            for height in 1..HALF_MAXIMUM_AREA {
                let half_area = length * width + length * height + width * height;
                if half_area < HALF_MAXIMUM_AREA {
                    ohalloran_numbers[half_area as usize] = false;
                }
            }
        }
    }

    println!("Values larger than 6 and less than 1_000 which cannot be the surface area of a cuboid:");
    for i in 3..HALF_MAXIMUM_AREA {
        if ohalloran_numbers[i as usize] {
            print!("{} ", i * 2);
        }
    }
    println!();
}
