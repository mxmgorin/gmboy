use crate::bus::Bus;
use crate::ppu::oam::OAM_ADDR_START;

#[derive(Debug, Clone, Default)]
pub struct Dma {
    pub is_active: bool,
    pub dest_addr: u16,
    pub src_addr: u16,
    pub start_delay: u8,
}

impl Dma {
    pub fn start(&mut self, address: u8) {
        self.is_active = true;
        self.start_delay = 2;
        self.dest_addr = 0;
        self.src_addr = (address as u16) << 8;
    }

    pub fn tick(bus: &mut Bus) {
        if !bus.dma.is_active {
            return;
        }

        if bus.dma.start_delay > 0 {
            bus.dma.start_delay -= 1;
            return;
        }

        let addr = bus.dma.src_addr | bus.dma.dest_addr;
        let value = bus.read(addr);
        bus.oam_ram.write(OAM_ADDR_START + bus.dma.dest_addr, value);
        bus.dma.dest_addr = bus.dma.dest_addr.wrapping_add(1);
        bus.dma.is_active = bus.dma.dest_addr < 160;
    }
}
