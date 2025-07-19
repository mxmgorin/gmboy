use crate::bus::{Bus, ECHO_MIRROR_OFFSET};
use crate::ppu::oam::OAM_ADDR_START;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Dma {
    pub is_active: bool,
    pub current_index: u16,
    pub src_addr: u16,
    pub start_delay: u8,
    pub queue_addr: Option<u16>,
}

impl Dma {
    pub fn start(&mut self, address: u8) {
        if self.is_active {
            self.queue_addr = Some((address as u16) << 8);
        } else {
            self.src_addr = (address as u16) << 8;
            self.current_index = 0;
        }

        self.start_delay = 2;
        self.is_active = true;
    }

    pub fn is_transferring(&self) -> bool {
        self.is_active && (self.start_delay == 0 || self.queue_addr.is_some())
    }

    /// Executes a single OAM DMA write and auto-increments the internal index cursor.
    pub fn tick(bus: &mut Bus) {
        if !bus.dma.is_active {
            return;
        }

        if bus.dma.start_delay > 0 {
            bus.dma.start_delay -= 1;

            if bus.dma.queue_addr.is_none() {
                return;
            }
        } else if let Some(queue_addr) = bus.dma.queue_addr {
            bus.dma.queue_addr = None;
            bus.dma.src_addr = queue_addr;
            bus.dma.current_index = 0;
        }

        let addr = bus.dma.src_addr + bus.dma.current_index;
        // DMA from high addresses doesn't read from HRAM or MMIO, it reads an extended echo ram instead
        let addr = match addr {
            0xFE00..=0xFFFF => addr - ECHO_MIRROR_OFFSET,
            _ => addr,
        };
        let byte = bus.read(addr);
        let dest_addr = OAM_ADDR_START + bus.dma.current_index;
        bus.oam_ram.write(dest_addr, byte);
        bus.dma.current_index = bus.dma.current_index.wrapping_add(1);
        bus.dma.is_active = bus.dma.current_index < 160;
    }
}
