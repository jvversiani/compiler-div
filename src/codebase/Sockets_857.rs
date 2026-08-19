// Rosetta Code task: Sockets
// Source: https://rosettacode.org/wiki/Sockets#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Server received: "hello socket world"
// =======================

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    // --- Start a server on an OS-assigned free port (127.0.0.1:0) ---
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    // Handle one connection in a background thread
    let server = thread::spawn(move || {
        let (mut socket, _) = listener.accept().unwrap();

        let mut buf = Vec::new();
        socket.read_to_end(&mut buf).unwrap();
        println!("Server received: {:?}", String::from_utf8_lossy(&buf));
    });

    // --- Client: connect to the server's actual address ---
    {
        let mut my_stream = TcpStream::connect(addr).unwrap();
        let _ = my_stream.write(b"hello socket world");
    } // <- my_stream dropped here, closing the socket so read_to_end finishes

    // Wait for the server thread to finish
    server.join().unwrap();
}
