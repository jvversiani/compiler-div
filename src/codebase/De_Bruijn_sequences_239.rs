// Rosetta Code task: De Bruijn sequences
// Source: https://rosettacode.org/wiki/De_Bruijn_sequences#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// The length (with wrap) of the sequence is 10003. The first 130 digits are:
// 0000100020003000400050006000700080009001100120013001400150016001700180019002100220023002400250026002700280029003100320033003400350
// and the last 130 digits are:
// 6898689969697769786979698769886989699769986999777787779778877897798779978787978887889789878997979887989799879998888988998989999000
// Testing sequence: The sequence does contain all PINs.
// Testing the reversed sequence: The sequence does contain all PINs.
//
// After replacing 4444th digit with '.':
// PIN 1459 does not occur in the sequence.
// PIN 4591 does not occur in the sequence.
// PIN 5914 does not occur in the sequence.
// PIN 8145 does not occur in the sequence.
// The sequence does not contain all PINs.
// =======================

fn db(seq: &mut Vec<u8>, a: &mut Vec<u8>, k: usize, n: usize, t: usize, p: usize) {
    if t > n {
        if n % p == 0 {
            seq.append(&mut a[1..p+1].to_vec().clone());
        }
    } else {
            a[t] = a[t - p];
            db(seq, a, k, n, t + 1, p);
            for j in (a[t - p] as usize + 1)..k {
                a[t] = j as u8;
                db(seq, a, k, n, t + 1, t);
            }
    }
}

fn debruijn(k: usize, n: usize) -> String {
    const ALPHABET: &[u8; 36] = b"0123456789abcdefghijklmnopqrstuvwxyz";
    let mut a = vec![0_u8; k * n];
    let mut seq: Vec<u8> = vec![];
    db(&mut seq, &mut a, k, n, 1, 1);
    let mut s = seq.iter().map(|i| ((ALPHABET[*i as usize] as char).to_string())).collect::<Vec<String>>().join("");
    s += &(s.clone()[..3]); // add the wrap here, so len 1000 -> len 1003
    return s;
}

fn verify_all_pin(mut s: String, k: usize, n: usize, deltaposition: usize) {
    if deltaposition != 0 {
        s = s[..deltaposition].to_string() + "." + &s[deltaposition+1..];
    }
    let mut result = true;
    for i in 0..k.pow(n as u32) - 1 {
        let pin = format!("{:.*}", n, i);
        if !s.contains(&pin) {
            println!("PIN {pin} does not occur in the sequence.");
            result = false;
        }
    }
    println!(
        "The sequence does{}contain all PINs.",
        if result { " " } else { " not " }
    );
}

fn main() {
    let s = debruijn(10, 4);
    println!(
        r#"The length (with wrap) of the sequence is {}. The first 130 digits are:
{}
and the last 130 digits are:
{}"#,
        s.len(),
        &s[..130],
        &s[s.len() - 130..]
    );
    print!("Testing sequence: ");
    verify_all_pin(s.clone(), 10, 4, 0);
    print!("Testing the reversed sequence: ");
    verify_all_pin(s.clone().chars().rev().collect(), 10, 4, 0);
    println!("\nAfter replacing 4444th digit with \'.\':");
    verify_all_pin(s.clone(), 10, 4, 4444);
}
