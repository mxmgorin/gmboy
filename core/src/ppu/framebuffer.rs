use std::ops::{Deref, DerefMut};
use serde::{Deserialize, Serialize};
use crate::ppu::{PPU_BYTES_PER_PIXEL, PPU_PITCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameBuffer {
    buffer: Box<[u8]>,
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

impl FrameBuffer {
    pub const PITCH: usize = PPU_PITCH;
    pub const BYTES_PER_PIXEL: usize = PPU_BYTES_PER_PIXEL;

    pub fn new(buffer: Box<[u8]>) -> Self {
        FrameBuffer {
            buffer,
        }
    }
}