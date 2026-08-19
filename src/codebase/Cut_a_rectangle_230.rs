// Rosetta Code task: Cut a rectangle
// Source: https://rosettacode.org/wiki/Cut_a_rectangle#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 2 x 1: 1
// 2 x 2: 2
// 3 x 2: 3
// 4 x 1: 1
// 4 x 2: 4
// 4 x 3: 9
// 4 x 4: 22
// 5 x 2: 5
// 5 x 4: 39
// 6 x 1: 1
// 6 x 2: 6
// 6 x 3: 23
// 6 x 4: 90
// 6 x 5: 263
// 6 x 6: 1018
// 7 x 2: 7
// 7 x 4: 151
// 7 x 6: 2947
// 8 x 1: 1
// 8 x 2: 8
// 8 x 3: 53
// 8 x 4: 340
// 8 x 5: 1675
// 8 x 6: 11174
// 8 x 7: 55939
// 8 x 8: 369050
// 9 x 2: 9
// 9 x 4: 553
// 9 x 6: 31721
// 9 x 8: 1812667
// =======================

fn cwalk(mut vis: &mut Vec<Vec<bool>>, count: &mut isize, w: usize, h: usize, y: usize, x: usize, d: usize) {
    if x == 0 || y == 0 || x == w || y == h {
        *count += 1;
        return;
    }
 
    vis[y][x] = true;
    vis[h - y][w - x] = true;

    if x != 0 && ! vis[y][x - 1] {
        cwalk(&mut vis, count, w, h, y, x - 1, d | 1);
    }
    if d & 1 != 0 && x < w && ! vis[y][x+1] {
        cwalk(&mut vis, count, w, h, y, x + 1, d | 1);
    }
    if y != 0 && ! vis[y - 1][x] {
        cwalk(&mut vis, count, w, h, y - 1, x, d | 2);
    }
    if d & 2 != 0 && y < h && ! vis[y + 1][x] {
        cwalk(&mut vis, count, w, h, y + 1, x, d | 2);
    }

    vis[y][x] = false;
    vis[h - y][w - x] = false;
}

fn count_only(x: usize, y: usize) -> isize {
    let mut count = 0;
    let mut w = x;
    let mut h = y;
 
    if (h * w) & 1 != 0 {
        return count;
    }
    if h & 1 != 0 {
        std::mem::swap(&mut w, &mut h);
    }
 
    let mut vis = vec![vec![false; w + 1]; h + 1];
    vis[h / 2][w / 2] = true;
 
    if w & 1 != 0 {
        vis[h / 2][w / 2 + 1] = true;
    }
    let mut res;
    if w > 1 {
        cwalk(&mut vis, &mut count, w, h, h / 2, w / 2 - 1, 1);
        res = 2 * count - 1;
        count = 0;
        if w != h {
            cwalk(&mut vis, &mut count, w, h, h / 2 + 1, w / 2, if w & 1 != 0 { 3 } else { 2 });
        } 
        res += 2 * count - if w & 1 == 0 { 1 } else { 0 };
    }
    else {
        res = 1;
    }
 
    if w == h {
        res = 2 * res + 2;
    }
    res
}

fn main() {
    for y in 1..10 {
        for x in 1..y + 1 {
            if x & 1 == 0 || y & 1 == 0 {
                println!("{} x {}: {}", y, x, count_only(x, y));
            }
        }
    }
}
