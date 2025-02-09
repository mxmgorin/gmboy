use crate::bus::Bus;

#[derive(Debug, Clone, Default)]
struct Dma {
    is_transferring: bool,
    current_byte: u8,
    src_addr: u8,
    start_delay: u8,
}

impl Dma {
    pub fn start(&mut self, addr: u8) {
        self.is_transferring = true;
        self.start_delay = 2;
        self.current_byte = 0x00;
        self.src_addr = addr;
    }

    pub fn tick(&mut self, bus: &mut Bus) {
        if !self.is_transferring {
            return;
        }

        if self.start_delay > 0 {
            self.start_delay -= 1;
            return;
        }

        let value = bus.read(self.src_addr as u16 * 0x100) + self.current_byte;
        bus.ppu.oam_write(self.current_byte as u16, value);
        self.current_byte += 1;
        self.is_transferring = self.current_byte < 0xA0; // DMA transfers 160 bytes at a time
    }
}