// Rosetta Code task: CSV to HTML translation
// Source: https://rosettacode.org/wiki/CSV_to_HTML_translation#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// <table>
// <tr><td>Character</td><td>Speech</td></tr>
// <tr><td>The multitude</td><td>The messiah! Show us the messiah!</td></tr>
// <tr><td>Brians mother</td><td><angry>Now you listen here! He's not the messiah; he's a very naughty boy! Now go away!</angry></td></tr>
// <tr><td>The multitude</td><td>Who are you?</td></tr>
// <tr><td>Brians mother</td><td>I'm his mother; that's who!</td></tr>
// <tr><td>The multitude</td><td>Behold his mother! Behold his mother!</td></tr>
// </table>
// =======================

static INPUT : &'static str  =
"Character,Speech
The multitude,The messiah! Show us the messiah!
Brians mother,<angry>Now you listen here! He's not the messiah; he's a very naughty boy! Now go away!</angry>
The multitude,Who are you?
Brians mother,I'm his mother; that's who!
The multitude,Behold his mother! Behold his mother!";

fn main() {
    print!("<table>\n<tr><td>");
    for c in INPUT.chars() {
        match c {
            '\n' => print!("</td></tr>\n<tr><td>"),
            ','  => print!("</td><td>"),
            '<'  => print!("<"),
            '>'  => print!(">"),
            '&'  => print!("&"),
            _    => print!("{}", c)
        }
    }
    println!("</td></tr>\n</table>");
}
