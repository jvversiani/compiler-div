// Rosetta Code task: Cholesky decomposition
// Source: https://rosettacode.org/wiki/Cholesky_decomposition#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// 5.0000	0.0000	0.0000	
// 3.0000	3.0000	0.0000	
// -1.0000	1.0000	3.0000	
//
// 4.2426	0.0000	0.0000	0.0000	
// 5.1854	6.5659	0.0000	0.0000	
// 12.7279	3.0460	1.6497	0.0000	
// 9.8995	1.6246	1.8497	1.3926
// =======================

fn cholesky(mat: Vec<f64>, n: usize) -> Vec<f64> {
    let mut res = vec![0.0; mat.len()];
    for i in 0..n {
        for j in 0..(i+1){
            let mut s = 0.0;
            for k in 0..j {
                s += res[i * n + k] * res[j * n + k];
            }
            res[i * n + j] = if i == j { (mat[i * n + i] - s).sqrt() } else { (1.0 / res[j * n + j] * (mat[i * n + j] - s)) };
        }
    }
    res
}

fn show_matrix(matrix: Vec<f64>, n: usize){
    for i in 0..n {
        for j in 0..n {
            print!("{:.4}\t", matrix[i * n + j]);
        }
        println!("");
    }
    println!("");
}

fn main(){
    let dimension = 3 as usize;
    let m1 = vec![25.0, 15.0, -5.0,
                  15.0, 18.0,  0.0,
                  -5.0,  0.0, 11.0];
    let res1 = cholesky(m1, dimension);
    show_matrix(res1, dimension);

    let dimension = 4 as usize;
    let m2 = vec![18.0, 22.0,  54.0,  42.0,
                  22.0, 70.0,  86.0,  62.0,
                  54.0, 86.0, 174.0, 134.0,
                  42.0, 62.0, 134.0, 106.0];
    let res2 = cholesky(m2, dimension);
    show_matrix(res2, dimension);
}
