use crate::ppu::lcd::PixelColor;
use crate::ppu::{LCD_X_RES, PPU_BUFFER_LEN, PPU_BYTES_PER_PIXEL, PPU_PITCH};
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};
use std::ptr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameBuffer {
    buffer: Box<[u8]>,
    pushed_count: usize,
}

impl Deref for FrameBuffer {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

impl DerefMut for FrameBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer
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
            buffer,
            pushed_count: 0,
        }
    }

    #[inline(always)]
    pub fn push(&mut self, ly: usize, pixel: PixelColor) {
        let index = self.pushed_count.wrapping_add(ly * LCD_X_RES as usize);
        let base = index * PPU_BYTES_PER_PIXEL;
        let bytes = pixel.as_rgb565_bytes();

        unsafe {
            let dst = self.buffer.as_mut_ptr().add(base);
            ptr::copy_nonoverlapping(bytes.as_ptr(), dst, PPU_BYTES_PER_PIXEL);
        }

        self.pushed_count += 1;
    }

    #[inline(always)]
    pub const fn reset(&mut self) {
        self.pushed_count = 0;
    }

    #[inline(always)]
    pub const fn count(&self) -> usize {
        self.pushed_count
    }
}
