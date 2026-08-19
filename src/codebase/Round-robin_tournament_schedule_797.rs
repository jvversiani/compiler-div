// Rosetta Code task: Round-robin tournament schedule
// Source: https://rosettacode.org/wiki/Round-robin_tournament_schedule#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Round robin for 12 players:
//
// Round  1: ( 1 vs 12) ( 2 vs 11) ( 3 vs 10) ( 4 vs 9 ) ( 5 vs 8 ) ( 6 vs 7 )
// Round  2: ( 1 vs 11) (12 vs 10) ( 2 vs 9 ) ( 3 vs 8 ) ( 4 vs 7 ) ( 5 vs 6 )
// Round  3: ( 1 vs 10) (11 vs 9 ) (12 vs 8 ) ( 2 vs 7 ) ( 3 vs 6 ) ( 4 vs 5 )
// Round  4: ( 1 vs 9 ) (10 vs 8 ) (11 vs 7 ) (12 vs 6 ) ( 2 vs 5 ) ( 3 vs 4 )
// Round  5: ( 1 vs 8 ) ( 9 vs 7 ) (10 vs 6 ) (11 vs 5 ) (12 vs 4 ) ( 2 vs 3 )
// Round  6: ( 1 vs 7 ) ( 8 vs 6 ) ( 9 vs 5 ) (10 vs 4 ) (11 vs 3 ) (12 vs 2 )
// Round  7: ( 1 vs 6 ) ( 7 vs 5 ) ( 8 vs 4 ) ( 9 vs 3 ) (10 vs 2 ) (11 vs 12)
// Round  8: ( 1 vs 5 ) ( 6 vs 4 ) ( 7 vs 3 ) ( 8 vs 2 ) ( 9 vs 12) (10 vs 11)
// Round  9: ( 1 vs 4 ) ( 5 vs 3 ) ( 6 vs 2 ) ( 7 vs 12) ( 8 vs 11) ( 9 vs 10)
// Round 10: ( 1 vs 3 ) ( 4 vs 2 ) ( 5 vs 12) ( 6 vs 11) ( 7 vs 10) ( 8 vs 9 )
// Round 11: ( 1 vs 2 ) ( 3 vs 12) ( 4 vs 11) ( 5 vs 10) ( 6 vs 9 ) ( 7 vs 8 )
//
//
// Round robin for 5 players (0 denotes a bye):
//
// Round  1: ( 1 vs 0 ) ( 2 vs 5 ) ( 3 vs 4 )
// Round  2: ( 1 vs 5 ) ( 0 vs 4 ) ( 2 vs 3 )
// Round  3: ( 1 vs 4 ) ( 5 vs 3 ) ( 0 vs 2 )
// Round  4: ( 1 vs 3 ) ( 4 vs 2 ) ( 5 vs 0 )
// Round  5: ( 1 vs 2 ) ( 3 vs 0 ) ( 4 vs 5 )
// =======================

fn round_robin(n: usize) {
    assert!(n >= 2);
    let mut n = n;
    let mut list1: Vec<usize> = (2..=n).collect();
    
    if n % 2 == 1 {
        list1.push(0); // 0 denotes a "bye".
        n += 1;
    }
    
    for r in 1..n {
        print!("Round {:2}:", r);
        let list2 = vec![1].into_iter().chain(list1.iter().cloned()).collect::<Vec<_>>();
        
        for i in 0..(n / 2) {
            print!(" ({:>2} vs {:<2})", list2[i], list2[n - i - 1]);
        }
        
        println!();
        list1.rotate_right(1);
    }
}

fn main() {
    println!("Round robin for 12 players:\n");
    round_robin(12);

    println!("\n\nRound robin for 5 players (0 denotes a bye):\n");
    round_robin(5);
}
