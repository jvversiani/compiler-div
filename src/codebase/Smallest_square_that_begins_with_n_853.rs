// Rosetta Code task: Smallest square that begins with n
// Source: https://rosettacode.org/wiki/Smallest_square_that_begins_with_n#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
//     1    25    36     4   529    64   729    81     9   100
//  1156   121  1369   144  1521    16  1764  1849   196  2025
//  2116   225  2304  2401    25  2601  2704   289  2916  3025
//  3136   324  3364  3481 35344    36  3721  3844  3969   400
// 41209  4225  4356   441 45369  4624  4761   484    49  5041
// =======================

// © 2025
fn squares() -> impl Iterator <Item = u32> {
    (1..).map(|num| num * num)
}

fn start_squares() -> impl Iterator <Item = u32> {
    (1..).map(|num| squares()
        .skip_while(|&sqr| !select_eq(sqr, num))
        .next().unwrap()
    )
}

fn select_eq(sqr: u32, num: u32) -> bool { 
    if sqr > num {select_eq(sqr / 10, num)}
    else {sqr == num}
}

fn main() {
    let limit = 50;
    let list: Vec<u32> = start_squares().take(limit).collect();
    list.chunks(10)
        .for_each(|grp| {
            let str: Vec<String> = grp.iter()
                .map(|&s| format!("{s:5}"))
                .collect();
            println!("{}", str.join(" "))
        });
 }
