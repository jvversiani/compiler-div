// Rosetta Code task: Sine wave
// Source: https://rosettacode.org/wiki/Sine_wave#Rust
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// Wrote sine.wav (220500 samples)
// =======================

use std::fs::{File, remove_file};
use std::io::{self, BufWriter, Write};

fn main() -> io::Result<()> {
    let duration = 5u32;        // seconds
    let frequency = 220.0f64;   // Hz
    let sample_rate = 44_100u32;
    let amplitude = 0.5;        // 0.0..1.0

    let path = "sine.wav";
    let num_samples = duration * sample_rate;

    // Generate 16-bit signed samples
    let mut samples: Vec<i16> = Vec::with_capacity(num_samples as usize);
    for n in 0..num_samples {
        let t = n as f64 / sample_rate as f64;
        let s = (2.0 * std::f64::consts::PI * frequency * t).sin() * amplitude;
        samples.push((s * i16::MAX as f64) as i16);
    }

    write_wav(path, &samples, sample_rate)?;
    println!("Wrote {} ({} samples)", path, num_samples);

    remove_file(path).unwrap();

    Ok(())
}

/// Write a minimal 16-bit mono PCM WAV file.
fn write_wav(path: &str, samples: &[i16], sample_rate: u32) -> io::Result<()> {
    let mut w = BufWriter::new(File::create(path)?);

    let num_channels: u16 = 1;
    let bits_per_sample: u16 = 16;
    let byte_rate = sample_rate * num_channels as u32 * (bits_per_sample / 8) as u32;
    let block_align = num_channels * (bits_per_sample / 8);
    let data_bytes = (samples.len() * 2) as u32;

    // RIFF header
    w.write_all(b"RIFF")?;
    w.write_all(&(36 + data_bytes).to_le_bytes())?; // chunk size
    w.write_all(b"WAVE")?;

    // fmt subchunk
    w.write_all(b"fmt ")?;
    w.write_all(&16u32.to_le_bytes())?;          // subchunk1 size (PCM)
    w.write_all(&1u16.to_le_bytes())?;           // audio format = PCM
    w.write_all(&num_channels.to_le_bytes())?;
    w.write_all(&sample_rate.to_le_bytes())?;
    w.write_all(&byte_rate.to_le_bytes())?;
    w.write_all(&block_align.to_le_bytes())?;
    w.write_all(&bits_per_sample.to_le_bytes())?;

    // data subchunk
    w.write_all(b"data")?;
    w.write_all(&data_bytes.to_le_bytes())?;
    for &s in samples {
        w.write_all(&s.to_le_bytes())?;
    }

    w.flush()?;

    Ok(())
}