use crate::ppu::lcd::PixelColor;
use crate::ppu::{LCD_X_RES, LCD_Y_RES, PPU_BUFFER_LEN, PPU_BYTES_PER_PIXEL, PPU_PITCH};
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};
use std::ptr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameBuffer {
    bytes: Box<[u8]>,
    pushed_x: usize,
}

impl Deref for FrameBuffer {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

impl DerefMut for FrameBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bytes
    }
}

impl Default for FrameBuffer {
    fn default() -> Self {
        FrameBuffer::new(vec![0; PPU_BUFFER_LEN].into_boxed_slice())
    }
}

impl FrameBuffer {
    pub const PITCH: usize = PPU_PITCH;
    pub const BYTES_PER_PIXEL: usize = PPU_BYTES_PER_PIXEL;

    pub fn new(buffer: Box<[u8]>) -> Self {
        FrameBuffer {
            bytes: buffer,
            pushed_x: 0,
        }
    }

    pub fn rgb888(&self) -> Vec<u8> {
        let size = LCD_X_RES as usize * LCD_Y_RES as usize * 3;
        let mut rgb888 = Vec::with_capacity(size);

        for chunk in self.bytes.chunks_exact(2) {
            let pixel = u16::from_le_bytes([chunk[0], chunk[1]]);

            let r = ((pixel >> 11) & 0x1F) as u16 * 255 / 31;
            let g = ((pixel >> 5) & 0x3F) as u16 * 255 / 63;
            let b = (pixel & 0x1F) as u16 * 255 / 31;

            rgb888.push(r as u8);
            rgb888.push(g as u8);
            rgb888.push(b as u8);
        }

        rgb888
    }

    #[inline(always)]
    pub fn push(&mut self, ly: usize, pixel: PixelColor) {
        let pixel_index = self.pushed_x.wrapping_add(ly * LCD_X_RES as usize);
        let bytes_index = pixel_index * PPU_BYTES_PER_PIXEL;
        let bytes = pixel.as_rgb565_bytes();

        unsafe {
            let dst = self.bytes.as_mut_ptr().add(bytes_index);
            ptr::copy_nonoverlapping(bytes.as_ptr(), dst, PPU_BYTES_PER_PIXEL);
        }

        self.pushed_x += 1;
    }

    /// Resets position of X position
    #[inline(always)]
    pub const fn reset_x(&mut self) {
        self.pushed_x = 0;
    }

    /// Returns count of pushed pixels on X position
    #[inline(always)]
    pub const fn count_x(&self) -> usize {
        self.pushed_x
    }

    /// Sets the buffer to zero
    #[inline]
    pub fn clear(&mut self) {
        for byte in self.bytes.iter_mut() {
            *byte = 0;
        }
    }
}
