// Rosetta Code task: Boyer-Moore string search
// Source: https://rosettacode.org/wiki/Boyer-Moore_string_search#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// text1 = GCTAGCTCTACGAGTCTA
// text2 = GGCTATAATGCGTA
// text3 = there would have been a time for such a word
// text4 = needle need noodle needle
// text5 = DKnuthusesandprogramsanimaginarycomputertheMIXanditsassociatedmachinecodeandassemblylanguages
// text6 = Nearby farms grew an acre of alfalfa on the dairy's behalf, with bales of that alfalfa exchanged for milk.
//
// Found "TCTA" in 'text1' at indexes [6, 14]
// Found "TAATAAA" in 'text2' at indexes []
// Found "word" in 'text3' at indexes [40]
// Found "needle" in 'text4' at indexes [0, 19]
// Found "and" in 'text5' at indexes [10, 46, 73]
// Found "alfalfa" in 'text6' at indexes [29, 79]
// =======================

fn display(numbers: &Vec<i32>) {
    print!("[");
    for (i, num) in numbers.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("{}", num);
    }
    println!("]");
}

fn string_search_single(haystack: &str, needle: &str) -> i32 {
    // Rust's standard library doesn't have Boyer-Moore searcher directly,
    // but we can use the built-in find method which is efficient
    match haystack.find(needle) {
        Some(index) => index as i32,
        None => -1,
    }
}

fn string_search(haystack: &str, needle: &str) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut start: usize = 0;
    
    while start < haystack.len() {
        let haystack_reduced = &haystack[start..];
        let index = string_search_single(haystack_reduced, needle);
        
        if index >= 0 {
            result.push((start as i32) + index);
            start += index as usize + needle.len();
        } else {
            break;
        }
    }
    
    result
}

fn main() {
    let texts = vec![
        "GCTAGCTCTACGAGTCTA",
        "GGCTATAATGCGTA",
        "there would have been a time for such a word",
        "needle need noodle needle",
        "DKnuthusesandprogramsanimaginarycomputertheMIXanditsassociatedmachinecodeandassemblylanguages",
        "Nearby farms grew an acre of alfalfa on the dairy's behalf, with bales of that alfalfa exchanged for milk."
    ];

    let patterns = vec!["TCTA", "TAATAAA", "word", "needle", "and", "alfalfa"];

    for i in 0..texts.len() {
        println!("text{} = {}", i + 1, texts[i]);
    }
    println!();

    for i in 0..texts.len() {
        let indexes = string_search(texts[i], patterns[i]);
        print!("Found \"{}\" in 'text{}' at indexes ", patterns[i], i + 1);
        display(&indexes);
    }
}
