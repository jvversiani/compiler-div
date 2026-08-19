// Rosetta Code task: UTF-8 encode and decode
// Source: https://rosettacode.org/wiki/UTF-8_encode_and_decode#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Character: A, Unicode:\u{41}, UTF-8 encoded:41, Decoded: A
// Character: ö, Unicode:\u{f6}, UTF-8 encoded:C3B6, Decoded: ö
// Character: Ж, Unicode:\u{416}, UTF-8 encoded:D096, Decoded: Ж
// Character: €, Unicode:\u{20ac}, UTF-8 encoded:E282AC, Decoded: €
// Character: 𝄞, Unicode:\u{1d11e}, UTF-8 encoded:F09D849E, Decoded: 𝄞
// =======================

fn main() {
    let chars = vec!('A', 'ö', 'Ж', '€', '𝄞');
    chars.iter().for_each(|c| {
        let mut encoded = vec![0; c.len_utf8()];
        c.encode_utf8(&mut encoded);
        let decoded = String::from_utf8(encoded.to_vec()).unwrap();
        let encoded_string = encoded.iter().fold(String::new(), |acc, val| format!("{}{:X}", acc, val));
        println!("Character: {}, Unicode:{}, UTF-8 encoded:{}, Decoded: {}", c, c.escape_unicode(), encoded_string , decoded);
    });
}
