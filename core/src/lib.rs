use crate::ppu::tile::PixelColor;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::{env, fs, io};

pub mod apu;
pub mod auxiliary;
pub mod bus;
pub mod cart;
pub mod cpu;
pub mod debugger;
pub mod emu;
pub mod ppu;

pub struct LittleEndianBytes {
    pub low_byte: u8,
    pub high_byte: u8,
}

impl Into<u16> for LittleEndianBytes {
    fn into(self) -> u16 {
        let low_byte = self.low_byte as u16;
        let high_byte = self.high_byte as u16;

        low_byte | (high_byte << 8)
    }
}

/// Returns true if the n-th bit of byte is set, false otherwise.
pub fn get_bit_flag(byte: u8, pos: u8) -> bool {
    byte & (1 << pos) != 0
}

pub fn get_bit_flag16(val: u16, pos: u8) -> bool {
    get_bit16(val, pos) != 0
}

pub fn get_bit16(val: u16, pos: u8) -> u16 {
    val & (1 << pos)
}

/// Sets or clears the n-th bit of `a` based on the value of `on`.
pub fn set_bit(a: &mut u8, n: u8, on: bool) {
    if on {
        *a |= 1 << n; // Set the n-th bit to 1
    } else {
        *a &= !(1 << n); // Set the n-th bit to 0
    }
}

pub fn struct_to_bytes_mut<T>(s: &mut T) -> &mut [u8] {
    // Convert the mutable reference to a mutable raw pointer
    let ptr = s as *mut T as *mut u8;
    let size = size_of::<T>();

    // Convert the raw pointer to a mutable byte slice
    unsafe { std::slice::from_raw_parts_mut(ptr, size) }
}

pub fn hex_to_rgba(argb: u32) -> (u8, u8, u8, u8) {
    let alpha = ((argb >> 24) & 0xFF) as u8; // Extract alpha
    let red = ((argb >> 16) & 0xFF) as u8; // Extract red
    let green = ((argb >> 8) & 0xFF) as u8; // Extract green
    let blue = (argb & 0xFF) as u8; // Extract blue

    (red, green, blue, alpha)
}

pub fn into_pixel_colors(hex_colors: &[String]) -> [PixelColor; 4] {
    let colors: Vec<PixelColor> = hex_colors
        .iter()
        .map(|hex| PixelColor::from_u32(u32::from_str_radix(hex, 16).unwrap()))
        .collect();

    colors[..4].try_into().unwrap()
}

pub fn read_json_file<P, T>(path: P) -> io::Result<T>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    let path_ref = path.as_ref();
    let file = File::open(path_ref).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("Failed to open {}: {}", path_ref.display(), e),
        )
    })?;

    let reader = BufReader::new(file);
    serde_json::from_reader(reader).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Invalid JSON in {}: {}", path_ref.display(), e),
        )
    })
}

pub fn save_json_file<P, T>(path: P, data: &T) -> io::Result<()>
where
    P: AsRef<Path>,
    T: Serialize,
{
    let file = File::create(path.as_ref())?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, data).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

pub fn get_exe_dir() -> PathBuf {
    let exe_path = env::current_exe().expect("Failed to get executable path");

    exe_path
        .parent()
        .expect("Failed to get executable directory")
        .to_path_buf()
}

pub fn read_bytes(file_path: &Path) -> Result<Box<[u8]>, String> {
    if !file_path.exists() {
        return Err(format!("File not found: {file_path:?}"));
    }

    fs::read(file_path)
        .map(|x| x.into_boxed_slice())
        .map_err(|e| format!("Failed to read file: {e}"))
}

pub fn move_next_wrapped(curr_idx: usize, max_idx: usize) -> usize {
    if curr_idx < max_idx {
        curr_idx + 1
    } else {
        0
    }
}

pub fn move_prev_wrapped(curr_idx: usize, max_idx: usize) -> usize {
    if curr_idx > 0 {
        curr_idx - 1
    } else {
        max_idx
    }
}

pub fn change_f32_rounded(value: f32, delta: f32) -> f32 {
    ((value + delta) * 100.0).round() / 100.0
}

pub fn change_f64_rounded(value: f64, delta: f64) -> f64 {
    ((value + delta) * 100.0).round() / 100.0
}

pub fn change_duration(value: Duration, micros: i32) -> Duration {
    if micros < 0 {
        value.saturating_sub(Duration::from_micros(micros.unsigned_abs() as u64))
    } else {
        value.saturating_add(Duration::from_micros(micros as u64))
    }
}

pub fn change_usize(value: usize, delta: i32) -> usize {
    if delta < 0 {
        value - delta.unsigned_abs() as usize
    } else {
        value + delta as usize
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_get_next_wrapped() {
        assert_eq!(move_next_wrapped(0, 2), 1);
        assert_eq!(move_next_wrapped(1, 2), 2);
        assert_eq!(move_next_wrapped(2, 2), 0);
    }

    #[test]
    fn test_get_prev_wrapped() {
        assert_eq!(move_prev_wrapped(0, 2), 2);
        assert_eq!(move_prev_wrapped(1, 2), 0);
        assert_eq!(move_prev_wrapped(2, 2), 1);
    }

    #[test]
    fn test_get_bit_flag_1() {
        assert!(get_bit_flag(0b0001, 0));
        assert!(!get_bit_flag(0b0010, 0));

        assert!(get_bit_flag(0b0010, 1));
        assert!(!get_bit_flag(0b0001, 1));

        assert!(get_bit_flag(0b10000000, 7));
        assert!(!get_bit_flag(0b01000000, 7));

        assert!(get_bit_flag(0b10101010, 1));
        assert!(!get_bit_flag(0b10101010, 2));
    }

    #[test]
    fn test_set_bit_1() {
        let mut a = 0b1010; // 10 in decimal

        set_bit(&mut a, 2, true);
        assert_eq!(a, 0b1110);

        set_bit(&mut a, 2, false);
        assert_eq!(a, 0b1010);
    }
}
