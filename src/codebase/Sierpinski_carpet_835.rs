// Rosetta Code task: Sierpinski carpet
// Source: https://rosettacode.org/wiki/Sierpinski_carpet#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
//
// N=0
// #
//
// N=1
// ###
// # #
// ###
//
// N=2
// #########
// # ## ## #
// #########
// ###   ###
// # #   # #
// ###   ###
// #########
// # ## ## #
// #########
//
// N=3
// ###########################
// # ## ## ## ## ## ## ## ## #
// ###########################
// ###   ######   ######   ###
// # #   # ## #   # ## #   # #
// ###   ######   ######   ###
// ###########################
// # ## ## ## ## ## ## ## ## #
// ###########################
// #########         #########
// # ## ## #         # ## ## #
// #########         #########
// ###   ###         ###   ###
// # #   # #         # #   # #
// ###   ###         ###   ###
// #########         #########
// # ## ## #         # ## ## #
// #########         #########
// ###########################
// # ## ## ## ## ## ## ## ## #
// ###########################
// ###   ######   ######   ###
// # #   # ## #   # ## #   # #
// ###   ######   ######   ###
// ###########################
// # ## ## ## ## ## ## ## ## #
// ###########################
// =======================

fn main() {
    for i in 0..4 {
        println!("\nN={}", i);
        println!("{}", sierpinski_carpet(i));
    }
}

fn sierpinski_carpet(n: u32) -> String {
    let mut carpet = vec!["#".to_string()];
    for _ in 0..n {
        let mut top: Vec<_> = carpet.iter().map(|x| x.repeat(3)).collect();
        let middle: Vec<_> = carpet
            .iter()
            .map(|x| x.to_string() + &x.replace("#", " ") + x)
            .collect();
        let bottom = top.clone();

        top.extend(middle);
        top.extend(bottom);
        carpet = top;
    }
    carpet.join("\n")
}
