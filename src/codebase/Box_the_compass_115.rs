// Rosetta Code task: Box the compass
// Source: https://rosettacode.org/wiki/Box_the_compass#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Index  Degrees  Compass point
// -----  -------  -------------
//  1       0.00   north
//  2      16.87   north by east
//  3      16.88   north-northeast
//  4      33.75   northeast by north
//  5      50.62   northeast
//  6      50.63   northeast by east
//  7      67.50   east-northeast
//  8      84.37   east by north
//  9      84.38   east
// 10     101.25   east by south
// 11     118.12   east-southeast
// 12     118.13   southeast by east
// 13     135.00   southeast
// 14     151.87   southeast by south
// 15     151.88   south-southeast
// 16     168.75   south by east
// 17     185.62   south
// 18     185.63   south by west
// 19     202.50   south-southwest
// 20     219.37   southwest by south
// 21     219.38   southwest
// 22     236.25   southwest by west
// 23     253.12   west-southwest
// 24     253.13   west by south
// 25     270.00   west
// 26     286.87   west by north
// 27     286.88   west-northwest
// 28     303.75   northwest by west
// 29     320.62   northwest
// 30     320.63   northwest by north
// 31     337.50   north-northwest
// 32     354.37   north by west
//  1     354.38   north
// =======================

fn expand(cp: &str) -> String {
    let mut out = String::new();
    for c in cp.chars() {
        out.push_str(match c {
            'N' => "north",
            'E' => "east",
            'S' => "south",
            'W' => "west",
            'b' => " by ",
            _ => "-",
        });
    }
    out
}

fn main() {
    let cp = [
        "N", "NbE", "N-NE", "NEbN", "NE", "NEbE", "E-NE", "EbN",
        "E", "EbS", "E-SE", "SEbE", "SE", "SEbS", "S-SE", "SbE",
        "S", "SbW", "S-SW", "SWbS", "SW", "SWbW", "W-SW", "WbS",
        "W", "WbN", "W-NW", "NWbW", "NW", "NWbN", "N-NW", "NbW"
    ];
    println!("Index  Degrees  Compass point");
    println!("-----  -------  -------------");
    for i in 0..=32 {
        let index = i % 32;
        let heading = i as f32 * 11.25
            + match i % 3 {
                1 => 5.62,
                2 => -5.62,
                _ => 0.0,
            };
        println!(
            "{:2}     {:6.2}   {}",
            index + 1,
            heading,
            expand(cp[index])
        );
    }
}
