// Rosetta Code task: Idoneal numbers
// Source: https://rosettacode.org/wiki/Idoneal_numbers#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
//    1    2    3    4    5    6    7    8    9   10   12   13   15   16   18
//   21   22   24   25   28   30   33   37   40   42   45   48   57   58   60
//   70   72   78   85   88   93  102  105  112  120  130  133  165  168  177
//  190  210  232  240  253  273  280  312  330  345  357  385  408  462  520
//  760  840 1320 1365 1848
// =======================

fn idoneals(limit: u32) -> impl Iterator<Item = u32> {
    (1 ..= limit).into_iter()
        .filter(|&n| (1 ..= n).into_iter()
            .all(|a| (a + 1 ..= n).into_iter()
                .take_while(|b| a * b + a + b < n)
                .all(|b| (a * b + (b + 1) * (a + b) ..)
                    .step_by((a + b) as usize)
                    .skip_while(|&s| s < n)
                    .next().unwrap() > n
                )
            )
        )
}

fn main() {
    idoneals(2_000).collect::<Vec<u32>>()
        .chunks(15).for_each(|grp|
            println!("{}", grp.iter().map(|n|format!("{:>4}", n))
                .collect::<Vec<String>>()
                .join(" ")
            )
        )
} // © 2026
