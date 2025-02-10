use crate::hardware::ram::Ram;
use crate::ppu::Ppu;

pub const DMA_ADDRESS: u16 = 0xFF46;

#[derive(Debug, Clone, Default)]
pub struct Dma {
    pub is_started: bool,
    byte: u8,
    value: u8,
    start_delay: u8,
}

impl Dma {
    pub fn start(&mut self, value: u8) {
        self.is_started = true;
        self.start_delay = 2;
        self.byte = 0x00;
        self.value = value;
    }

    pub fn tick(&mut self, ram: &Ram, ppu: &mut Ppu) {
        if !self.is_started {
            return;
        }

        if self.start_delay > 0 {
            self.start_delay -= 1;
            return;
        }

        let value = ram.h_ram_read(self.value as u16 * 0x100) + self.byte;
        ppu.oam_write(self.byte as u16, value);
        self.byte += 1;
        self.is_started = self.byte < 0xA0;
    }
}