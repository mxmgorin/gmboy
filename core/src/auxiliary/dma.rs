use crate::bus::{Bus, ECHO_MIRROR_OFFSET};
use crate::ppu::oam::OAM_ADDR_START;
use serde::{Deserialize, Serialize};

pub const VRAM_DMA_ADDR_START: u16 = 0xFF51;
pub const VRAM_DMA_ADDR_END: u16 = 0xFF55;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OamDma {
    pub is_active: bool,
    pub current_index: u16,
    pub src_addr: u16,
    pub start_delay: u8,
    pub queue_addr: Option<u16>,
}

impl OamDma {
    #[inline]
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

    #[inline]
    pub fn is_transferring(&self) -> bool {
        self.is_active && (self.start_delay == 0 || self.queue_addr.is_some())
    }

    /// Executes a single OAM DMA write and auto-increments the internal index cursor.
    #[inline]
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
        bus.io.ppu.oam_ram.write(dest_addr, byte);
        bus.dma.current_index = bus.dma.current_index.wrapping_add(1);
        bus.dma.is_active = bus.dma.current_index < 160;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VramDma {
    pub hblank_active: bool,
    pub src: u16,
    pub dst: u16,
    pub blocks_remaining: u8,

    // Registers
    pub hdma1: u8,
    pub hdma2: u8,
    pub hdma3: u8,
    pub hdma4: u8,
}

impl VramDma {
    #[inline]
    pub fn write(bus: &mut Bus, addr: u16, value: u8) {
        match addr {
            0xFF51 => bus.vram_dma.hdma1 = value,
            0xFF52 => bus.vram_dma.hdma2 = value & 0xF0,
            0xFF53 => bus.vram_dma.hdma3 = value & 0x1F,
            0xFF54 => bus.vram_dma.hdma4 = value & 0xF0,
            0xFF55 => VramDma::start(bus, value),
            _ => {}
        }
    }

    #[inline]
    pub fn read(bus: &Bus) -> u8 {
        if bus.vram_dma.hblank_active {
            0x80 | (bus.vram_dma.blocks_remaining - 1)
        } else {
            0xFF
        }
    }

    #[inline(always)]
    pub fn tick_hblank(bus: &mut Bus) {
        if !bus.vram_dma.hblank_active {
            return;
        }

        VramDma::copy_block(bus);
        bus.vram_dma.blocks_remaining -= 1;

        if bus.vram_dma.blocks_remaining == 0 {
            bus.vram_dma.hblank_active = false;
        }
    }

    #[inline]
    fn start(bus: &mut Bus, value: u8) {
        // Cancellation case
        if bus.vram_dma.hblank_active && (value & 0x80) == 0 {
            bus.vram_dma.hblank_active = false;
            return;
        }

        let blocks = (value & 0x7F) + 1;

        bus.vram_dma.src = VramDma::compute_src(&bus.vram_dma);
        bus.vram_dma.dst = VramDma::compute_dst(&bus.vram_dma);
        bus.vram_dma.blocks_remaining = blocks;

        if value & 0x80 == 0 {
            // GDMA (Immediate)
            VramDma::do_gdma(bus);
        } else {
            // HBlank DMA
            bus.vram_dma.hblank_active = true;
        }
    }

    #[inline]
    fn do_gdma(bus: &mut Bus) {
        let blocks = bus.vram_dma.blocks_remaining;

        for _ in 0..blocks {
            VramDma::copy_block(bus);
        }

        bus.vram_dma.blocks_remaining = 0;
        bus.vram_dma.hblank_active = false;
    }

    #[inline(always)]
    fn copy_block(bus: &mut Bus) {
        let mut src = bus.vram_dma.src;
        let mut dst = bus.vram_dma.dst;

        for i in 0..0x10 {
            src += i;
            dst += i;
            let byte = bus.read(src);
            bus.io.ppu.video_ram.write(dst, byte);
        }

        bus.vram_dma.src = src;
        bus.vram_dma.dst = dst;
    }

    #[inline(always)]
    fn compute_src(dma: &VramDma) -> u16 {
        ((dma.hdma1 as u16) << 8) | ((dma.hdma2 as u16) & 0xF0)
    }

    #[inline(always)]
    fn compute_dst(dma: &VramDma) -> u16 {
        0x8000 | (((dma.hdma3 as u16) & 0x1F) << 8) | ((dma.hdma4 as u16) & 0xF0)
    }
}
