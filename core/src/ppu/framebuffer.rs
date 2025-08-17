use crate::ppu::{PPU_BYTES_PER_PIXEL, PPU_PITCH};

pub struct FrameBuffer<'a> {
    pub buffer: &'a mut [u8],
    pub pitch: usize,
    pub bytes_per_pixel: usize,
}

impl<'a> FrameBuffer<'a> {
    pub fn from_ppu(buffer: &'a mut [u8]) -> Self {
        FrameBuffer {
            buffer,
            pitch: PPU_PITCH,
            bytes_per_pixel: PPU_BYTES_PER_PIXEL,
        }
    }
}