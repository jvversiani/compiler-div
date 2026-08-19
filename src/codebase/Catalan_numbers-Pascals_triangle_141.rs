// Rosetta Code task: Catalan numbers/Pascal's triangle
// Source: https://rosettacode.org/wiki/Catalan_numbers/Pascal's_triangle#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 1 2 5 14 42 132 429 1430 4862 16796 58786 208012 742900 2674440 9694845
// =======================

fn main()
{let n=15usize;
 let mut t= [0; 17];
 t[1]=1;
 let mut j:usize;
 for i in 1..n+1
 {
  j=i;
  loop{
      if j==1{
          break; 
    }
    t[j]=t[j] + t[j-1];
    j=j-1;
  }
  t[i+1]= t[i];
  j=i+1;
  loop{
    if j==1{
    break;
    }
    t[j]=t[j] + t[j-1];
    j=j-1;
  }
  print!("{} ", t[i+1]-t[i]);
 }
}
