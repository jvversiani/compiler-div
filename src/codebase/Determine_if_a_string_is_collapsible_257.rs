// Rosetta Code task: Determine if a string is collapsible
// Source: https://rosettacode.org/wiki/Determine_if_a_string_is_collapsible#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Old:  45 <<<122333444455555666666777777788888888999999999>>>
// New:   9 <<<123456789>>>
//
// Old:   0 <<<>>>
// New:   0 <<<>>>
//
// Old:  72 <<<"If I were two-faced, would I be wearing this one?" --- Abraham Lincoln >>>
// New:  70 <<<"If I were two-faced, would I be wearing this one?" - Abraham Lincoln >>>
//
// Old:  72 <<<..1111111111111111111111111111111111111111111111111111111111111117777888>>>
// New:   4 <<<.178>>>
//
// Old:  72 <<<I never give 'em hell, I just tell the truth, and they think it's hell. >>>
// New:  69 <<<I never give 'em hel, I just tel the truth, and they think it's hel. >>>
//
// Old:  72 <<<                                                    --- Harry S Truman  >>>
// New:  17 <<< - Hary S Truman >>>
//
// Old:  80 <<<The better the 4-wheel drive, the further you'll be from help when ya get stuck!>>>
// New:  77 <<<The beter the 4-whel drive, the further you'l be from help when ya get stuck!>>>
//
// Old:  16 <<<headmistressship>>>
// New:  14 <<<headmistreship>>>
// =======================

fn collapse_string(val: &str) -> String {
    let mut output = String::new();
    let mut chars = val.chars().peekable();

    while let Some(c) = chars.next() {
        while let Some(&b) = chars.peek() {
            if b == c {
                chars.next();
            } else {
                break;
            }
        }

        output.push(c);
    }

    output
}

fn main() {
    let tests = [
        "122333444455555666666777777788888888999999999",
        "",
        "\"If I were two-faced, would I be wearing this one?\" --- Abraham Lincoln ",
        "..1111111111111111111111111111111111111111111111111111111111111117777888",
        "I never give 'em hell, I just tell the truth, and they think it's hell. ",
        "                                                    --- Harry S Truman  ",
        "The better the 4-wheel drive, the further you'll be from help when ya get stuck!",
        "headmistressship",
    ];

    for s in &tests {
        println!("Old: {:>3} <<<{}>>>", s.len(), s);
        let collapsed = collapse_string(s);
        println!("New: {:>3} <<<{}>>>", collapsed.len(), collapsed);

        println!();
    }
}
