// Rosetta Code task: Show ASCII table
// Source: https://rosettacode.org/wiki/Show_ASCII_table#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
//  32 : Spc    48 : 0      64 : @      80 : P      96 : `     112 : p     
//  33 : !      49 : 1      65 : A      81 : Q      97 : a     113 : q     
//  34 : "      50 : 2      66 : B      82 : R      98 : b     114 : r     
//  35 : #      51 : 3      67 : C      83 : S      99 : c     115 : s     
//  36 : $      52 : 4      68 : D      84 : T     100 : d     116 : t     
//  37 : %      53 : 5      69 : E      85 : U     101 : e     117 : u     
//  38 : &      54 : 6      70 : F      86 : V     102 : f     118 : v     
//  39 : '      55 : 7      71 : G      87 : W     103 : g     119 : w     
//  40 : (      56 : 8      72 : H      88 : X     104 : h     120 : x     
//  41 : )      57 : 9      73 : I      89 : Y     105 : i     121 : y     
//  42 : *      58 : :      74 : J      90 : Z     106 : j     122 : z     
//  43 : +      59 : ;      75 : K      91 : [     107 : k     123 : {     
//  44 : ,      60 : <      76 : L      92 : \     108 : l     124 : |     
//  45 : -      61 : =      77 : M      93 : ]     109 : m     125 : }     
//  46 : .      62 : >      78 : N      94 : ^     110 : n     126 : ~     
//  47 : /      63 : ?      79 : O      95 : _     111 : o     127 : Del
// =======================

fn main() {
    for i in 0u8..16 {
        for j in ((32+i)..128).step_by(16) {
            let k = (j as char).to_string();
            print!("{:3} : {:<3}   ", j, match j {
                32 => "Spc",
                127 => "Del",
                _ => &k,
            });
        }
        println!();
    }
}
