// Rosetta Code task: Forward difference
// Source: https://rosettacode.org/wiki/Forward_difference#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// [90, 47, 58, 29, 22, 32, 55, 5, 55, 73]
// [-43, 11, -29, -7, 10, 23, -50, 50, 18]
// [54, -40, 22, 17, 13, -73, 100, -32]
// [-94, 62, -5, -4, -86, 173, -132]
// [156, -67, 1, -82, 259, -305]
// [-223, 68, -83, 341, -564]
// [291, -151, 424, -905]
// [-442, 575, -1329]
// [1017, -1904]
// [-2921]
// =======================

fn forward_difference(input_seq: Vec<i32>, order: u32) -> Vec<i32> {
    match order {
        0 => input_seq,
        1 => {
            let input_seq_iter = input_seq.into_iter();
            let clone_of_input_seq_iter = input_seq_iter.clone();
            input_seq_iter.zip(clone_of_input_seq_iter.skip(1)).map(|(current, next)| next - current).collect()
        },
        _ => forward_difference(forward_difference(input_seq, order - 1), 1),
    }
}

fn main() {
    let mut sequence = vec![90, 47, 58, 29, 22, 32, 55, 5, 55, 73];
    loop {
        println!("{:?}", sequence);
        sequence = forward_difference(sequence, 1);
        if sequence.is_empty() {
            break;
        }
    }
}
