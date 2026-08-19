// Rosetta Code task: Search a list
// Source: https://rosettacode.org/wiki/Search_a_list#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// First occurence of 'Bush' at Some(4)
// Last occurence of 'Bush' at Some(7)
// First occurence of 'Rob' at None
// =======================

fn main() {
    let haystack=vec!["Zig", "Zag", "Wally", "Ronald", "Bush", "Krusty", "Charlie", 
                        "Bush", "Boz", "Zag"];

    println!("First occurence of 'Bush' at {:?}",haystack.iter().position(|s| *s=="Bush"));
    println!("Last occurence of 'Bush' at {:?}",haystack.iter().rposition(|s| *s=="Bush"));
    println!("First occurence of 'Rob' at {:?}",haystack.iter().position(|s| *s=="Rob"));
}
