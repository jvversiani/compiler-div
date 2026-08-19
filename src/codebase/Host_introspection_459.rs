// Rosetta Code task: Host introspection
// Source: https://rosettacode.org/wiki/Host_introspection#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Word size: 8 bytes
// Endianness: Little
// =======================

#[derive(Copy, Clone, Debug)]
enum Endianness {
    Big, Little,
}

impl Endianness {
    fn target() -> Self {
        #[cfg(target_endian = "big")]
        {
            Endianness::Big
        }
        #[cfg(not(target_endian = "big"))]
        {
            Endianness::Little
        }
    }
}

fn main() {
    println!("Word size: {} bytes", std::mem::size_of::<usize>());
    println!("Endianness: {:?}", Endianness::target());
}
