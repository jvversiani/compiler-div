// Rosetta Code task: Fibonacci word
// Source: https://rosettacode.org/wiki/Fibonacci_word#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// N          Length     Entropy    Word                                                        
//          1          1 0.0000000000 1                                                           
//          2          1 0.0000000000 0                                                           
//          3          2 1.0000000000 10                                                          
//          4          3 0.9182958341 010                                                         
//          5          5 0.9709505945 10010                                                       
//          6          8 0.9544340029 01010010                                                    
//          7         13 0.9612366047 1001001010010                                               
//          8         21 0.9587118830 010100101001001010010                                       
//          9         34 0.9596868938 1001001010010010100101001001010010                          
//         10         55 0.9593160321 0101001010010010100101001001010010010100101001001010010     
//         11         89 0.9594579158 Too long                                                    
//         12        144 0.9594037542 Too long                                                    
//         13        233 0.9594244470 Too long                                                    
//         14        377 0.9594165437 Too long                                                    
//         15        610 0.9594195626 Too long                                                    
//         16        987 0.9594184095 Too long                                                    
//         17       1597 0.9594188500 Too long                                                    
//         18       2584 0.9594186817 Too long                                                    
//         19       4181 0.9594187460 Too long                                                    
//         20       6765 0.9594187214 Too long                                                    
//         21      10946 0.9594187308 Too long                                                    
//         22      17711 0.9594187272 Too long                                                    
//         23      28657 0.9594187286 Too long                                                    
//         24      46368 0.9594187281 Too long                                                    
//         25      75025 0.9594187283 Too long                                                    
//         26     121393 0.9594187282 Too long                                                    
//         27     196418 0.9594187282 Too long                                                    
//         28     317811 0.9594187282 Too long                                                    
//         29     514229 0.9594187282 Too long                                                    
//         30     832040 0.9594187282 Too long                                                    
//         31    1346269 0.9594187282 Too long                                                    
//         32    2178309 0.9594187282 Too long                                                    
//         33    3524578 0.9594187282 Too long                                                    
//         34    5702887 0.9594187282 Too long                                                    
//         35    9227465 0.9594187282 Too long                                                    
//         36   14930352 0.9594187282 Too long                                                    
//         37   24157817 0.9594187282 Too long                                                    
// =======================

struct Fib<T> {
    curr: T,
    next: T,
}

impl<T> Fib<T> {
    fn new(curr: T, next: T) -> Self {
        Fib { curr: curr, next: next, }
    }
}

impl Iterator for Fib<String>  {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.curr.clone();
        self.curr = self.next.clone();
        self.next = format!("{}{}", ret, self.next);
        Some(ret)
    }
}

fn get_entropy(s: &[u8]) -> f64 {
    let mut entropy = 0.0;
    let mut histogram = [0.0; 256];
 
    for i in 0..s.len() {
        histogram.get_mut(s[i] as usize).map(|v| *v += 1.0);
    }

    for i in 0..256 {
        if histogram[i] > 0.0 {
            let ratio = histogram[i] / s.len() as f64;
            entropy -= ratio * ratio.log2();
        }
    }
    entropy
}

fn main() {
    let f = Fib::new("1".to_string(), "0".to_string());
        println!("{:10} {:10} {:10} {:60}", "N", "Length", "Entropy", "Word");
    for (i, s) in f.take(37).enumerate() {
        let word = if s.len() > 60 {"Too long"} else {&*s};
        println!("{:10} {:10} {:.10} {:60}", i + 1, s.len(), get_entropy(&s.bytes().collect::<Vec<_>>()), word);
    }
}
