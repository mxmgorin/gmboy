use crate::core::ppu::oam::OamRam;
use crate::ppu::vram::VideoRam;

#[derive(Debug, Clone)]
pub struct Ppu {
    video_ram: VideoRam,
    oam_ram: OamRam,
}

impl Default for Ppu {
    fn default() -> Self {
        Self::new()
    }
}

impl Ppu {
    pub fn new() -> Ppu {
        Self {
            video_ram: VideoRam::new(),
            oam_ram: OamRam::new(),
        }
    }

    pub fn tick(&self) {
        // todo: Impl
    }

    pub fn vram_read(&self, addr: u16) -> u8 {
        self.video_ram.read(addr)
    }

    pub fn vram_write(&mut self, addr: u16, value: u8) {
        self.video_ram.write(addr, value);
    }

    pub fn oam_read(&self, addr: u16) -> u8 {
        self.oam_ram.read_byte(addr)
    }

    pub fn oam_write(&mut self, addr: u16, value: u8) {
        self.oam_ram.write_byte(addr, value);
    }
}
