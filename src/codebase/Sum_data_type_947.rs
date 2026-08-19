// Rosetta Code task: Sum data type
// Source: https://rosettacode.org/wiki/Sum_data_type#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// home = V4(127, 0, 0, 1), loopback = V6("::1")
// =======================

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main(){
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    println!("home = {:?}, loopback = {:?}", home, loopback);
}
