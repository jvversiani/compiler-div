// Rosetta Code task: The Name Game
// Source: https://rosettacode.org/wiki/The_Name_Game#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
//     
//     Gary, Gary, bo-bary
//     Banana-fana fo-fary
//     Fee-fi-mo-mary
//     Gary!
//
//     Earl, Earl, bo-bearl
//     Banana-fana fo-fearl
//     Fee-fi-mo-mearl
//     Earl!
//
//     Billy, Billy, bo-illy
//     Banana-fana fo-filly
//     Fee-fi-mo-milly
//     Billy!
//
//     Felix, Felix, bo-belix
//     Banana-fana fo-elix
//     Fee-fi-mo-melix
//     Felix!
//
//     Mary, Mary, bo-bary
//     Banana-fana fo-fary
//     Fee-fi-mo-ary
//     Mary!
//
//     Shirley, Shirley, bo-bhirley
//     Banana-fana fo-fhirley
//     Fee-fi-mo-mhirley
//     Shirley!
// =======================

fn verse(name: &str) -> String {
    let lower_name = name.to_lowercase();
    let mut x = lower_name.clone();
    x.replace_range(0..1, x[0..1].to_uppercase().as_str());
    let y = if "AEIOU".contains(&x[0..=0]) {lower_name.as_str()} else {&x[1..]};
    let b = if &x[0..1] == "B" {""} else {"b"};
    let f = if &x[0..1] == "F" {""} else {"f"};
    let m = if &x[0..1] == "M"{""} else {"m"};
    return format!(r#"
    {x}, {x}, bo-{b}{y}
    Banana-fana fo-{f}{y}
    Fee-fi-mo-{m}{y}
    {x}!"#);
}

fn main() {
    for name in ["gARY", "Earl", "Billy", "Felix", "Mary", "sHIRley"] {
        println!("{}", verse(name));
    }
}
