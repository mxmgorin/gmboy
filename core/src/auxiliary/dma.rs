use crate::bus::{Bus, ECHO_MIRROR_OFFSET};
use crate::ppu::lcd::PpuMode;
use crate::ppu::oam::OAM_ADDR_START;
use crate::ppu::vram::{VRAM_ADDR_END, VRAM_ADDR_START};
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
        if !bus.oam_dma.is_active {
            return;
        }

        if bus.oam_dma.start_delay > 0 {
            bus.oam_dma.start_delay -= 1;

            if bus.oam_dma.queue_addr.is_none() {
                return;
            }
        } else if let Some(queue_addr) = bus.oam_dma.queue_addr {
            bus.oam_dma.queue_addr = None;
            bus.oam_dma.src_addr = queue_addr;
            bus.oam_dma.current_index = 0;
        }

        let addr = bus.oam_dma.src_addr + bus.oam_dma.current_index;
        // DMA from high addresses doesn't read from HRAM or MMIO, it reads an extended echo ram instead
        let addr = match addr {
            0xFE00..=0xFFFF => addr - ECHO_MIRROR_OFFSET,
            _ => addr,
        };
        let byte = bus.read(addr);
        let dest_addr = OAM_ADDR_START + bus.oam_dma.current_index;
        bus.io.ppu.oam_ram.write(dest_addr, byte);
        bus.oam_dma.current_index = bus.oam_dma.current_index.wrapping_add(1);
        bus.oam_dma.is_active = bus.oam_dma.current_index < 160;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VramDma {
    pub src_addr: u16,
    pub dst_addr: u16,
    pub hdma_active: bool,
    pub blocks: u8,

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
            0xFF51 => {
                bus.vram_dma.hdma1 = value;
                bus.vram_dma.src_addr = VramDma::compute_src(&bus.vram_dma);
            }
            0xFF52 => {
                bus.vram_dma.hdma2 = value & 0xF0;
                bus.vram_dma.src_addr = VramDma::compute_src(&bus.vram_dma);
            }
            0xFF53 => {
                bus.vram_dma.hdma3 = value & 0x1F;
                bus.vram_dma.dst_addr = VramDma::compute_dst(&bus.vram_dma);
            }
            0xFF54 => {
                bus.vram_dma.hdma4 = value & 0xF0;
                bus.vram_dma.dst_addr = VramDma::compute_dst(&bus.vram_dma);
            }
            0xFF55 => VramDma::write_hdma5(bus, value),
            _ => {}
        }
    }

    #[inline]
    pub fn read_hdma5(bus: &Bus) -> u8 {
        if bus.vram_dma.hdma_active {
            // active: bit7 reads as 0
            (bus.vram_dma.blocks - 1) & 0x7F
        } else if bus.vram_dma.blocks == 0 {
            // completed
            0xFF
        } else {
            // aborted: bit7 reads as 1
            0x80 | ((bus.vram_dma.blocks - 1) & 0x7F)
        }
    }

    #[inline(always)]
    pub fn tick_hdma(bus: &mut Bus) {
        if !bus.vram_dma.hdma_active {
            return;
        }

        VramDma::copy_block(bus);
        bus.vram_dma.blocks -= 1;

        if bus.vram_dma.blocks == 0 {
            bus.vram_dma.hdma_active = false;
        }
    }

    #[inline]
    fn write_hdma5(bus: &mut Bus, value: u8) {
        bus.vram_dma.blocks = (value & 0x7F) + 1;
        let bit_7_zero = (value & 0x80) == 0;

        // Cancellation case
        if bus.vram_dma.hdma_active && bit_7_zero {
            bus.vram_dma.hdma_active = false;
            return;
        }

        if bit_7_zero {
            VramDma::tick_gdma(bus);
        } else {
            bus.vram_dma.hdma_active = true;

            if bus.io.ppu.lcd.status.get_ppu_mode() == PpuMode::HBlank {
                // Transfer immediately when HDMA is started on HBlank
                VramDma::tick_hdma(bus);
            }
        }
    }

    /// General-Purpose DMA (GPDMA / GDMA): Copies all at once
    #[inline]
    fn tick_gdma(bus: &mut Bus) {
        if bus.io.ppu.lcd.is_vram_blocked() {
            return;
        }

        let blocks = bus.vram_dma.blocks;

        for _ in 0..blocks {
            VramDma::copy_block(bus);
        }
    }

    #[inline(always)]
    fn copy_block(bus: &mut Bus) {
        let mut src = bus.vram_dma.src_addr;
        let mut dst = bus.vram_dma.dst_addr;

        for _ in 0..0x10 {
            src = src.wrapping_add(1);
            let (new_dst, dst_overflowed) = dst.overflowing_add(1);
            dst = new_dst;

            if dst_overflowed {
                bus.vram_dma.hdma_active = false;
                return;
            }

            let byte = match src {
                0x0000..=0x7FFF | 0xA000..=0xBFFF => bus.cart.read(src),
                0xC000..=0xDFFF => bus.io.ram.read_wram(src),
                // VRAM, OAM, I/O registers, and HRAM are not accessible from VRAM DMA
                VRAM_ADDR_START..=VRAM_ADDR_END | 0xE000..=0xFFFF => 0xFF,
            };

            if dst > VRAM_ADDR_END {
                dst = VRAM_ADDR_START
            }

            bus.io.ppu.video_ram.write(dst, byte);
        }

        bus.vram_dma.src_addr = src;
        bus.vram_dma.dst_addr = dst;
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
