use crate::ppu::tile::PixelColor;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
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

pub fn read_json_file<T: DeserializeOwned>(path: &Path) -> io::Result<T> {
    let data = fs::read_to_string(path)?;
    let config =
        serde_json::from_str(&data).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(config)
}

pub fn save_json_file<T: Serialize>(path: &Path, data: &T) -> io::Result<()> {
    let mut file = File::create(path)?;
    let json = serde_json::to_string_pretty(data)?;

    file.write_all(json.as_bytes())
}

pub fn get_exe_path() -> PathBuf {
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

pub fn get_next_wrapped(curr_idx: usize, max_idx: usize) -> usize {
    if curr_idx < max_idx {
        curr_idx + 1
    } else {
        0
    }
}
#[cfg(test)]
pub mod tests {
    use super::*;

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
