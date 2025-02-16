use crate::bus::{Bus, ECHO_MIRROR_OFFSET};
use crate::ppu::oam::OAM_ADDR_START;

#[derive(Debug, Clone, Default)]
pub struct Dma {
    pub is_active: bool,
    pub index: u16,
    pub src_addr: u16,
    pub start_delay: u8,
}

impl Dma {
    pub fn start(&mut self, address: u8) {
        self.is_active = true;
        self.start_delay = 2;
        self.index = 0;
        self.src_addr = (address as u16) << 8;
    }

    /// Executes a single OAM DMA write and auto-increments the internal index cursor.
    pub fn tick(bus: &mut Bus) {
        if !bus.dma.is_active {
            return;
        }

        if bus.dma.start_delay > 0 {
            bus.dma.start_delay -= 1;
            return;
        }

        let addr = bus.dma.src_addr + bus.dma.index;
        // DMA from high addresses doesn't read from HRAM or MMIO, it reads an extended echo ram instead
        let addr = match addr {
            0xFE00..=0xFFFF => addr - ECHO_MIRROR_OFFSET,
            _ => addr,
        };
        let byte = bus.read(addr);
        let dest_addr = OAM_ADDR_START + bus.dma.index;
        bus.oam_ram.write(dest_addr, byte);
        bus.dma.index = bus.dma.index.wrapping_add(1);
        bus.dma.is_active = bus.dma.index < 160;
    }
}
