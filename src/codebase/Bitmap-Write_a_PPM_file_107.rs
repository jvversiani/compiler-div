// Rosetta Code task: Bitmap/Write a PPM file
// Source: https://rosettacode.org/wiki/Bitmap/Write_a_PPM_file#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// all good!
// =======================

use std::path::Path;
use std::io::Write;
use std::fs::File;

pub struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

pub struct PPM {
    height: u32,
    width: u32,
    data: Vec<u8>,
}

impl PPM {
    pub fn new(height: u32, width: u32) -> PPM {
        let size = 3 * height * width;
        let buffer = vec![0; size as usize];
        PPM { height: height, width: width, data: buffer }
    }

    fn buffer_size(&self) -> u32 {
        3 * self.height * self.width
    }

    fn get_offset(&self, x: u32, y: u32) -> Option<usize> {
        let offset = (y * self.width * 3) + (x * 3);
        if offset < self.buffer_size() {
            Some(offset as usize)
        } else {
            None
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Option<RGB> {
        match self.get_offset(x, y) {
            Some(offset) => {
                let r = self.data[offset];
                let g = self.data[offset + 1];
                let b = self.data[offset + 2];
                Some(RGB {r: r, g: g, b: b})
            },
            None => None
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: RGB) -> bool {
        match self.get_offset(x, y) {
            Some(offset) => {
                self.data[offset] = color.r;
                self.data[offset + 1] = color.g;
                self.data[offset + 2] = color.b;
                true
            },
            None => false
        }
    }

    pub fn write_file(&self, filename: &str) -> std::io::Result<()> {
        let path = Path::new(filename);
        let mut file = File::create(&path)?;
        let header = format!("P6 {} {} 255\n", self.width, self.height);
        file.write(header.as_bytes())?;
        file.write(&self.data)?;
        Ok(())
    }
}

fn main () {
    // A 4-wide, 3-tall image: 3 bytes per pixel, all zeroed (black) to start.
    let mut ppm = PPM::new(3, 4);
    assert_eq!(36, ppm.buffer_size());

    // Paint one pixel of each primary and read them back.
    assert!(ppm.set_pixel(0, 0, RGB { r: 255, g: 0, b: 0 }));
    assert!(ppm.set_pixel(3, 1, RGB { r: 0, g: 255, b: 0 }));
    assert!(ppm.set_pixel(2, 2, RGB { r: 0, g: 0, b: 255 }));

    let red = ppm.get_pixel(0, 0).unwrap();
    assert_eq!((255, 0, 0), (red.r, red.g, red.b));
    let green = ppm.get_pixel(3, 1).unwrap();
    assert_eq!((0, 255, 0), (green.r, green.g, green.b));
    let blue = ppm.get_pixel(2, 2).unwrap();
    assert_eq!((0, 0, 255), (blue.r, blue.g, blue.b));

    // An untouched pixel is still black.
    let black = ppm.get_pixel(1, 0).unwrap();
    assert_eq!((0, 0, 0), (black.r, black.g, black.b));

    // Out of bounds is rejected rather than clobbering the buffer.
    assert!(!ppm.set_pixel(4, 2, RGB { r: 1, g: 2, b: 3 }));
    assert!(ppm.get_pixel(4, 2).is_none());

    // Write the P6 file the task asks for, then check what landed on disk.
    let out = std::env::temp_dir().join("rosetta_bitmap_write_ppm.ppm");
    let filename = out.to_str().unwrap();
    ppm.write_file(filename).expect("failed to write PPM file");

    let written = std::fs::read(filename).expect("failed to read PPM file back");
    let magic = b"P6 4 3 255\n";
    assert_eq!(magic, &written[..magic.len()]);
    assert_eq!(magic.len() + 36, written.len());
    // Pixel (0, 0) is the first RGB triple after the header.
    assert_eq!(&[255u8, 0, 0], &written[magic.len()..magic.len() + 3]);

    std::fs::remove_file(filename).expect("failed to remove PPM file");

    println!("all good!");
}
