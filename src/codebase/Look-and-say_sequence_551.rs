// Rosetta Code task: Look-and-say sequence
// Source: https://rosettacode.org/wiki/Look-and-say_sequence#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Sequence 0: [1]
// Sequence 1: [1, 1]
// Sequence 2: [2, 1]
// Sequence 3: [1, 2, 1, 1]
// Sequence 4: [1, 1, 1, 2, 2, 1]
// Sequence 5: [3, 1, 2, 2, 1, 1]
// Sequence 6: [1, 3, 1, 1, 2, 2, 2, 1]
// Sequence 7: [1, 1, 1, 3, 2, 1, 3, 2, 1, 1]
// Sequence 8: [3, 1, 1, 3, 1, 2, 1, 1, 1, 3, 1, 2, 2, 1]
// Sequence 9: [1, 3, 2, 1, 1, 3, 1, 1, 1, 2, 3, 1, 1, 3, 1, 1, 2, 2, 1, 1]
// =======================

fn next_sequence(in_seq: &[i8]) -> Vec<i8> {
    assert!(!in_seq.is_empty());

    let mut result = Vec::new();
    let mut current_number = in_seq[0];
    let mut current_runlength = 1;

    for i in &in_seq[1..] {
        if current_number == *i {
            current_runlength += 1;
        } else {
            result.push(current_runlength);
            result.push(current_number);
            current_runlength = 1;
            current_number = *i;
        }
    }
    result.push(current_runlength);
    result.push(current_number);
    result
}

fn main() {
    let mut seq = vec![1];

    for i in 0..10 {
        println!("Sequence {}: {:?}", i, seq);
        seq = next_sequence(&seq);
    }
}
