// Rosetta Code task: Duffinian numbers
// Source: https://rosettacode.org/wiki/Duffinian_numbers#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// First 50 Duffinian numbers:
//   4   8   9  16  21  25  27  32  35  36
//  39  49  50  55  57  63  64  65  75  77
//  81  85  93  98 100 111 115 119 121 125
// 128 129 133 143 144 155 161 169 171 175
// 183 185 187 189 201 203 205 209 215 217
//
// First 50 Duffinian triplets:
// (63, 64, 65)             (323, 324, 325)          (511, 512, 513)         
// (721, 722, 723)          (899, 900, 901)          (1443, 1444, 1445)      
// (2303, 2304, 2305)       (2449, 2450, 2451)       (3599, 3600, 3601)      
// (3871, 3872, 3873)       (5183, 5184, 5185)       (5617, 5618, 5619)      
// (6049, 6050, 6051)       (6399, 6400, 6401)       (8449, 8450, 8451)      
// (10081, 10082, 10083)    (10403, 10404, 10405)    (11663, 11664, 11665)   
// (12481, 12482, 12483)    (13447, 13448, 13449)    (13777, 13778, 13779)   
// (15841, 15842, 15843)    (17423, 17424, 17425)    (19043, 19044, 19045)   
// (26911, 26912, 26913)    (30275, 30276, 30277)    (36863, 36864, 36865)   
// (42631, 42632, 42633)    (46655, 46656, 46657)    (47523, 47524, 47525)   
// (53137, 53138, 53139)    (58563, 58564, 58565)    (72961, 72962, 72963)   
// (76175, 76176, 76177)    (79523, 79524, 79525)    (84099, 84100, 84101)   
// (86527, 86528, 86529)    (94177, 94178, 94179)    (108899, 108900, 108901)
// (121103, 121104, 121105) (125315, 125316, 125317) (128017, 128018, 128019)
// (129599, 129600, 129601) (137287, 137288, 137289) (144399, 144400, 144401)
// (144721, 144722, 144723) (154567, 154568, 154569) (158403, 158404, 158405)
// (166463, 166464, 166465) (167041, 167042, 167043)
// =======================

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn duffinian(n: i32) -> bool {
    if n == 2 {
        return false;
    }
    
    let mut total = 1;
    let mut power = 2;
    let m = n;
    let mut n = n;
    
    // Handle powers of 2
    while (n & 1) == 0 {
        total += power;
        power <<= 1;
        n >>= 1;
    }
    
    // Handle odd prime factors
    let mut p = 3;
    while p * p <= n {
        let mut sum = 1;
        power = p;
        while n % p == 0 {
            sum += power;
            power *= p;
            n /= p;
        }
        total *= sum;
        p += 2;
    }
    
    // Check if n is composite
    if m == n {
        return false;
    }
    
    // Handle remaining prime factor
    if n > 1 {
        total *= n + 1;
    }
    
    gcd(total, m) == 1
}

fn main() {
    println!("First 50 Duffinian numbers:");
    let mut count = 0;
    let mut n = 1;
    
    while count < 50 {
        if duffinian(n) {
            print!("{:3}", n);
            count += 1;
            if count % 10 == 0 {
                println!();
            } else {
                print!(" ");
            }
        }
        n += 1;
    }
    
    println!("\nFirst 50 Duffinian triplets:");
    let mut n = 1;
    let mut m = 0;
    let mut count = 0;
    
    while count < 50 {
        if duffinian(n) {
            m += 1;
        } else {
            m = 0;
        }
        
        if m == 3 {
            let triplet = format!("({}, {}, {})", n - 2, n - 1, n);
            print!("{:<24}", triplet);
            count += 1;
            if count % 3 == 0 {
                println!();
            } else {
                print!(" ");
            }
        }
        n += 1;
    }
    println!();
}
