// Rosetta Code task: Function composition
// Source: https://rosettacode.org/wiki/Function_composition#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 12
// =======================

fn compose<'a,F,G,T,U,V>(f: F, g: G) -> Box<dyn Fn(T) -> V + 'a>
    where F: Fn(U) -> V + 'a,
          G: Fn(T) -> U + 'a,
{
   Box::new(move |x| f(g(x)))
}

fn main() {
    // g: i32 -> i32 (add 1), then f: i32 -> i32 (double).
    let add_one = |x: i32| x + 1;
    let double = |x: i32| x * 2;

    // compose(f, g) produces x -> f(g(x)), i.e. double(add_one(x)).
    let h = compose(double, add_one);

    println!("{}", h(5)); // add_one(5) = 6, double(6) = 12
}
