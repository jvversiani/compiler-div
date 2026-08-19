// Rosetta Code task: Floyd's triangle
// Source: https://rosettacode.org/wiki/Floyd's_triangle#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
//   1
//   2  3
//   4  5  6
//   7  8  9 10
//  11 12 13 14 15
//   1
//   2  3
//   4  5  6
//   7  8  9 10
//  11 12 13 14 15
//  16 17 18 19 20 21
//  22 23 24 25 26 27 28
//  29 30 31 32 33 34 35 36
//  37 38 39 40 41 42 43 44  45
//  46 47 48 49 50 51 52 53  54  55
//  56 57 58 59 60 61 62 63  64  65  66
//  67 68 69 70 71 72 73 74  75  76  77  78
//  79 80 81 82 83 84 85 86  87  88  89  90  91
//  92 93 94 95 96 97 98 99 100 101 102 103 104 105
// =======================

fn main() {
    floyds_triangle(5);
    floyds_triangle(14);
}

fn floyds_triangle(n: u32) {
    let mut triangle: Vec<Vec<String>> = Vec::new();
    let mut current = 0;
    for i in 1..=n {
        let mut v = Vec::new();
        for _ in 0..i {
            current += 1;
            v.push(current);
        }
        let row = v.iter().map(|x| x.to_string()).collect::<Vec<_>>();
        triangle.push(row);
    }

    for row in &triangle {
        let arranged_row: Vec<_> = row
            .iter()
            .enumerate()
            .map(|(i, number)| {
                let space_len = triangle.last().unwrap()[i].len() - number.len() + 1;
                let spaces = " ".repeat(space_len);
                let mut padded_number = spaces;
                padded_number.push_str(&number);
                padded_number
            })
            .collect();
        println!("{}", arranged_row.join(""))
    }
}
