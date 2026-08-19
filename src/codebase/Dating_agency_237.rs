// Rosetta Code task: Dating agency
// Source: https://rosettacode.org/wiki/Dating_agency#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Dating service should NOT offer a date with Ariel
// Dating service should offer a date with Bertha
// 	Sailor Dune should take an offer to date her
// 	Sailor Kai should take an offer to date her
// 	Sailor Ray should take an offer to date her
// 	Sailor Tao should take an offer to date her
// Dating service should offer a date with Blue
// 	Sailor Dune should take an offer to date her
// 	Sailor Kai should take an offer to date her
// 	Sailor Ray should take an offer to date her
// 	Sailor Tao should take an offer to date her
// Dating service should NOT offer a date with Cali
// Dating service should NOT offer a date with Catalina
// Dating service should NOT offer a date with Gale
// Dating service should offer a date with Hannah
// 	Sailor Adrian should take an offer to date her
// 	Sailor Caspian should take an offer to date her
// 	Sailor Finn should take an offer to date her
// 	Sailor Fisher should take an offer to date her
// 	Sailor Heron should take an offer to date her
// 	Sailor Sailor should take an offer to date her
// Dating service should NOT offer a date with Isla
// Dating service should NOT offer a date with Marina
// Dating service should NOT offer a date with Shelly
// =======================

const SAILORS: [&str; 10] = [
    "Adrian", "Caspian", "Dune", "Finn", "Fisher", "Heron", "Kai", "Ray", "Sailor", "Tao",
];

const LADIES: [&str; 10] = [
    "Ariel", "Bertha", "Blue", "Cali", "Catalina", "Gale", "Hannah", "Isla", "Marina", "Shelly",
];

fn is_nice_girl(lady: &str) -> bool {
    (lady.as_bytes().first().unwrap()).is_multiple_of(2)
}

fn is_lovable(lady: &str, sailor: &str) -> bool {
    lady.as_bytes().last().unwrap() % 2 == sailor.as_bytes().last().unwrap() % 2
}

fn main() {
    for lady in LADIES {
        if is_nice_girl(lady) {
            println!("Dating service should offer a date with {lady}");
            for sailor in SAILORS {
                if is_lovable(lady, sailor) {
                    println!("\tSailor {sailor} should take an offer to date her")
                }
            }
        } else {
            println!("Dating service should NOT offer a date with {lady}")
        }
    }
}
