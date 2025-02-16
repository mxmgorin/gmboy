use crate::bus::Bus;

#[derive(Debug, Clone, Default)]
pub struct Dma {
    pub is_active: bool,
    pub dest_addr: u8,
    pub src_addr: u8,
    pub start_delay: u8,
}

impl Dma {
    pub fn start(&mut self, address: u8) {
        self.is_active = true;
        self.start_delay = 2;
        self.dest_addr = 0x00;
        self.src_addr = address;
    }

    pub fn tick(bus: &mut Bus) {
        if !bus.dma.is_active {
            return;
        }

        if bus.dma.start_delay > 0 {
            bus.dma.start_delay -= 1;
            return;
        }

        let addr = (bus.dma.src_addr as u16 * 0x100).wrapping_add(bus.dma.dest_addr as u16);
        let value = bus.read(addr);
        bus.oam_ram.write(bus.dma.dest_addr as u16, value);
        bus.dma.dest_addr += bus.dma.dest_addr.wrapping_add(1);
        bus.dma.is_active = bus.dma.dest_addr < 0xA0; // 160
    }
}
