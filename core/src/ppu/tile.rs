use crate::ppu::vram::{VRAM_ADDR_END, VRAM_ADDR_START};
use serde::{Deserialize, Serialize};

// Tile sets addresses
pub const TILE_SET_DATA_1_START: u16 = VRAM_ADDR_START;
pub const TILE_SET_1_END: u16 = 0x8FFF;
pub const TILE_SET_DATA_2_START: u16 = 0x8800;
pub const TILE_SET_2_END: u16 = 0x97FF;

// Tile maps addresses
pub const BG_TILE_MAP_1_ADDR_START: u16 = 0x9800;
pub const BG_TILE_MAP_1_ADDR_END: u16 = 0x9BFF;
pub const BG_TILE_MAP_2_ADDR_START: u16 = 0x9C00;
pub const BG_TILE_MAP_2_ADDR_END: u16 = VRAM_ADDR_END;

// Tile data info
pub const TILE_LINE_BYTES_COUNT: usize = 2;
pub const TILE_BIT_SIZE: u16 = 16;
pub const TILE_WIDTH: u16 = 8;
pub const TILE_HEIGHT: u16 = 8;
pub const TILE_BITS_COUNT: u8 = 8;
pub const TILES_COUNT: usize = 384;

/// Tile Data (Character Data). Tile data contains the actual visual representation of the tiles.
#[derive(Copy, Clone, Debug, Default)]
pub struct TileData {
    pub lines: [TileLineData; TILE_HEIGHT as usize],
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub struct TileLineData {
    pub byte1: u8,
    pub byte2: u8,
}

impl TileLineData {
    /// Mutable view as 2-byte array slice
    #[inline(always)]
    pub fn as_bytes_mut(&mut self) -> &mut [u8; 2] {
        // SAFETY: TileLineData is #[repr(C)] with exactly 2 u8 fields, no padding
        unsafe { &mut *(self as *mut TileLineData as *mut [u8; 2]) }
    }

    /// View entry as 4-byte array slice
    #[inline(always)]
    pub fn as_bytes(&self) -> &[u8; 2] {
        // SAFETY: TileLineData is #[repr(C)] with exactly 2 u8 fields, no padding
        unsafe { &*(self as *const TileLineData as *const [u8; 2]) }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct PixelData {
    pub byte1: u8,
    pub byte2: u8,
    pub bit: u8,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Pixel {
    pub color: PixelColor,
    pub color_id: ColorId,
}

impl Pixel {
    pub fn new(color: PixelColor, color_id: ColorId) -> Pixel {
        Pixel { color, color_id }
    }
}

impl PixelData {
    pub fn new(byte1: u8, byte2: u8, bit: u8) -> PixelData {
        Self { byte1, byte2, bit }
    }

    #[inline(always)]
    pub fn into_color_index(self) -> usize {
        get_color_index(self.byte1, self.byte2, self.bit)
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum ColorId {
    #[default]
    Lightest,
    Light,
    Dark,
    Darkest,
}

impl From<usize> for ColorId {
    fn from(value: usize) -> Self {
        match value {
            0 => ColorId::Lightest,
            1 => ColorId::Light,
            2 => ColorId::Dark,
            3 => ColorId::Darkest,
            _ => panic!("Invalid value for ColorId {value}"),
        }
    }
}

impl ColorId {
    pub fn new(byte1: u8, byte2: u8, bit: u8) -> ColorId {
        get_color_index(byte1, byte2, bit).into()
    }
}

pub fn get_color_index(byte1: u8, byte2: u8, bit: u8) -> usize {
    let bit1 = (byte1 >> (7 - bit)) & 0x01;
    let bit2 = (byte2 >> (7 - bit)) & 0x01;

    (bit2 << 1 | bit1) as usize
}

impl TileLineData {
    pub fn new(byte_one: u8, byte_two: u8) -> TileLineData {
        Self {
            byte1: byte_one,
            byte2: byte_two,
        }
    }

    pub fn get_color_id(&self, bit: u8) -> ColorId {
        ColorId::new(self.byte1, self.byte2, bit)
    }

    pub fn iter_color_ids(&self) -> impl Iterator<Item = ColorId> {
        TileLineIterator {
            bit: 0,
            line: *self,
        }
    }
}

pub struct TileLineIterator {
    pub bit: u8,
    pub line: TileLineData,
}

impl Iterator for TileLineIterator {
    type Item = ColorId;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bit < TILE_BITS_COUNT {
            let bit = self.bit;
            self.bit += 1;

            Some(self.line.get_color_id(bit))
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PixelColor {
    rgb565: u16,
}

impl PixelColor {
    pub fn new(r: u8, g: u8, b: u8) -> PixelColor {
        // Convert from 8-bit per channel to RGB565
        let r5 = (r as u16 >> 3) & 0x1F; // 5 bits
        let g6 = (g as u16 >> 2) & 0x3F; // 6 bits
        let b5 = (b as u16 >> 3) & 0x1F; // 5 bits
        let rgb565 = (r5 << 11) | (g6 << 5) | b5;

        PixelColor { rgb565 }
    }

    pub fn from_hex_rgba(hex: &str) -> PixelColor {
        assert!(hex.len() >= 6);

        let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
        //let a = u8::from_str_radix(&hex[6..8], 16).unwrap();

        Self::new(r, g, b)
    }

    /// Convert to RGB888 (full 8-bit channels)
    #[inline]
    pub fn as_rgb_bytes(&self) -> [u8; 3] {
        let r = ((self.rgb565 >> 11) & 0x1F) as u8;
        let g = ((self.rgb565 >> 5) & 0x3F) as u8;
        let b = (self.rgb565 & 0x1F) as u8;

        [
            r << 3 | (r >> 2), // 5-bit → 8-bit
            g << 2 | (g >> 4), // 6-bit → 8-bit
            b << 3 | (b >> 2), // 5-bit → 8-bit
        ]
    }

    #[inline]
    pub fn as_rgba_bytes(&self) -> [u8; 4] {
        let rgb = self.as_rgb_bytes();

        [rgb[0], rgb[1], rgb[2], 255]
    }

    /// Return raw RGB565 bytes in **little-endian** order
    #[inline]
    pub fn as_rgb565_bytes(&self) -> [u8; 2] {
        self.rgb565.to_le_bytes()
    }

    #[inline]
    pub fn zero() -> PixelColor {
        PixelColor::new(0, 0, 0)
    }
}

/// Each entry in the tile map is 1 byte and refers to a tile index in the tile data.
#[derive(Copy, Clone, Debug, Default)]
pub struct TileMapEntry {
    pub tile_idx: u8,
}
