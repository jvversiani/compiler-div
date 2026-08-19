// Rosetta Code task: Memory allocation
// Source: https://rosettacode.org/wiki/Memory_allocation#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Value at allocated memory: 123
// =======================

use std::alloc::{Layout, alloc, dealloc};

fn main() {
    // We use `unsafe` here because we dereference a raw pointer.
    unsafe {
        // Define a layout describing a block of memory for an i32.
        let int_layout = Layout::new::<i32>();

        // Allocate the memory. `alloc` returns a *mut u8.
        let ptr = alloc(int_layout) as *mut i32;

        // Guard against allocation failure before writing.
        assert!(!ptr.is_null());

        // Write to the allocated memory and read it back.
        *ptr = 123;
        assert_eq!(*ptr, 123);
        println!("Value at allocated memory: {}", *ptr);

        // Deallocate using the same layout it was allocated with.
        // Cast back to *mut u8, which is what `dealloc` expects.
        dealloc(ptr as *mut u8, int_layout);
    }
}