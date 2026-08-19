// Rosetta Code task: Strip block comments
// Source: https://rosettacode.org/wiki/Strip_block_comments#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// NON-NESTED BLOCK COMMENT EXAMPLE:
//
//
// function subroutine() {
// a =  b + c ;
// }
//
//
//
// function something() {
// }
//
//
// NESTED BLOCK COMMENT EXAMPLE:
//
//
//
// function something() {
// }
// =======================

// Strips first nest of block comments
fn _commentstripper(mut txt: String, deliml: &str, delimr: &str) -> String {
    let mut out = String::new();
    if txt.contains(deliml) {
        let mut indx = txt.find(deliml).unwrap();
        out += &txt[..indx];
        txt = txt[indx + deliml.len()..].to_string();
        txt = _commentstripper(txt, deliml, delimr);
        assert!(txt.contains(delimr), "Missing closing comment delimiter");
        indx = txt.find(delimr).unwrap();
        out += &txt[indx + delimr.len()..];
    } else {
        out = txt;
    }
    return out;
}

// Strips nests of block comments
fn commentstripper(mut txt: String, deliml: &str, delimr: &str) -> String {
    while txt.contains(deliml) {
        txt = _commentstripper(txt, deliml, delimr);
    }
    return txt.to_string();
}

fn main() {
    let deliml = "/*";
    let delimr = "*/";

    println!("NON-NESTED BLOCK COMMENT EXAMPLE:");
    let mut sample = r#"
/**
 * Some comments
 * longer comments here that we can parse.
 *
 * Rahoo 
 */
function subroutine() {
a = /* inline comment */ b + c ;
}
/*/ <-- tricky comments */

/**
* Another comment.
*/
function something() {
}
"#
    .to_string();

    println!("{}", commentstripper(sample, deliml, delimr));

    println!("\nNESTED BLOCK COMMENT EXAMPLE:");
    sample = r#"
/**
 * Some comments
 * longer comments here that we can parse.
 *
 * Rahoo 
 *//*
function subroutine() {
a = /* inline comment */ b + c ;
}
/*/ <-- tricky comments */
*/
/**
* Another comment.
*/
function something() {
}
"#
    .to_string();

    println!("{}", commentstripper(sample, deliml, delimr));
}
