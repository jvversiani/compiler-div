// Rosetta Code task: Elementary cellular automaton/Random number generator
// Source: https://rosettacode.org/wiki/Elementary_cellular_automaton/Random_number_generator#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 157 209 228 58 87 195 212 106 147 244
// =======================

//Assuming the code from the Elementary cellular automaton task is in the namespace.
fn main() {
    struct ElementaryCA {
        rule: u8,
        state: u64,
    }
    impl ElementaryCA {
        fn new(rule: u8) -> (u64, ElementaryCA) {
            let out = ElementaryCA {
                rule,
                state: 1,
            };
            (out.state, out)
        }
        fn next(&mut self) -> u64 {
            let mut next_state = 0u64;
            let state = self.state;
            for i in 0..64 {
                next_state |= (((self.rule as u64)>>(7 & (state.rotate_left(1).rotate_right(i as u32)))) & 1)<<i;
            }
            self.state = next_state;
            self.state
        }
    };
    struct WolfGen(ElementaryCA);
    impl WolfGen {
        fn new() -> WolfGen {
            let (_, ca) = ElementaryCA::new(30);
            WolfGen(ca)
        }
        fn next(&mut self) -> u8 {
            let mut out = 0;
            for i in 0..8 {
                out |= ((1 & self.0.next())<<i)as u8;
            }
            out
        }
    }
    let mut gen = WolfGen::new();
    for _ in 0..10 {
        print!("{} ", gen.next());
    }
}
