// Rosetta Code task: Determine if a string is squeezable
// Source: https://rosettacode.org/wiki/Determine_if_a_string_is_squeezable#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Squeeze: ' '
// Input (0 chars): 	
// Output (0 chars): 	
//
// Squeeze: '-'
// Input (72 chars): 	"If I were two-faced, would I be wearing this one?" --- Abraham Lincoln 
// Output (70 chars): 	"If I were two-faced, would I be wearing this one?" - Abraham Lincoln 
//
// Squeeze: '7'
// Input (72 chars): 	..1111111111111111111111111111111111111111111111111111111111111117777888
// Output (69 chars): 	..1111111111111111111111111111111111111111111111111111111111111117888
//
// Squeeze: ' '
// Input (95 chars): 	I never give 'em hell, I just tell the truth, and they think it's hell.
//     ---  Harry S Truman
// Output (91 chars): 	I never give 'em hell, I just tell the truth, and they think it's hell.
//  --- Harry S Truman
//
// Squeeze: '-'
// Input (95 chars): 	I never give 'em hell, I just tell the truth, and they think it's hell.
//     ---  Harry S Truman
// Output (93 chars): 	I never give 'em hell, I just tell the truth, and they think it's hell.
//     -  Harry S Truman
//
// Squeeze: 'r'
// Input (95 chars): 	I never give 'em hell, I just tell the truth, and they think it's hell.
//     ---  Harry S Truman
// Output (94 chars): 	I never give 'em hell, I just tell the truth, and they think it's hell.
//     ---  Hary S Truman
//
// Squeeze: 'e'
// Input (80 chars): 	The better the 4-wheel drive, the further you'll be from help when ya get stuck!
// Output (79 chars): 	The better the 4-whel drive, the further you'll be from help when ya get stuck!
//
// Squeeze: 's'
// Input (16 chars): 	headmistressship
// Output (14 chars): 	headmistreship
//
// =======================

fn squeezable_string<'a>(s: &'a str, squeezable: char) -> impl Iterator<Item = char> + 'a {
    let mut previous = None;

    s.chars().filter(move |c| match previous {
        Some(p) if p == squeezable && p == *c => false,
        _ => {
            previous = Some(*c);
            true
        }
    })
}

fn main() {
    fn show(input: &str, c: char) {
        println!("Squeeze: '{}'", c);
        println!("Input ({} chars): \t{}", input.chars().count(), input);
        let output: String = squeezable_string(input, c).collect();
        println!("Output ({} chars): \t{}", output.chars().count(), output);
        println!();
    }

    let harry = r#"I never give 'em hell, I just tell the truth, and they think it's hell.
    ---  Harry S Truman"#;

    #[rustfmt::skip]
    let inputs = [
        ("", ' '),
        (r#""If I were two-faced, would I be wearing this one?" --- Abraham Lincoln "#, '-'),
        ("..1111111111111111111111111111111111111111111111111111111111111117777888", '7'),
        (harry, ' '),
        (harry, '-'),
        (harry, 'r'),
        ("The better the 4-wheel drive, the further you'll be from help when ya get stuck!", 'e'),
        ("headmistressship", 's'),
    ];

    inputs.iter().for_each(|(input, c)| show(input, *c));
}
