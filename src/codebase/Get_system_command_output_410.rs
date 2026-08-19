// Rosetta Code task: Get system command output
// Source: https://rosettacode.org/wiki/Get_system_command_output#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// # /etc/fstab: static file system information.
// =======================

use std::process::Command;
use std::io::{Write, self};

fn main() {
    let output = Command::new("/bin/cat")
                            .arg("/etc/fstab")
                            .output()
                            .expect("failed to execute process");

    io::stdout().write(&output.stdout);
}
