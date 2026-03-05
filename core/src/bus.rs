use crate::auxiliary::dma::{OamDma, VramDma, VRAM_DMA_ADDR_END, VRAM_DMA_ADDR_START};
use crate::auxiliary::io::Io;
use crate::auxiliary::ram::{WRAM_CGB_BANK_END_ADDR, WRAM_START_ADDR};
use crate::cart::header::CgbFlag;
use crate::cart::Cart;
use crate::emu::config::GbModel;
use crate::ppu::lcd::{LCD_DMA_ADDRESS};
use crate::ppu::vram::{VRAM_ADDR_END, VRAM_ADDR_START};
use serde::{Deserialize, Serialize};

pub const ECHO_MIRROR_OFFSET: u16 = 0x2000;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Bus {
    pub cart: Cart,
    pub io: Io,
    pub oam_dma: OamDma,
    pub vram_dma: VramDma,
    flat_mem: Option<Vec<u8>>,
}

impl Bus {
    pub fn clone_empty_cart(&self) -> Self {
        Self {
            cart: Cart::empty(),
            io: self.io.clone(),
            flat_mem: self.flat_mem.clone(),
            oam_dma: self.oam_dma.clone(),
            vram_dma: self.vram_dma.clone(),
        }
    }

    pub fn new(cart: Cart, io: Io, model: Option<GbModel>) -> Self {
        let mut obj = Self {
            cart,
            io,
            flat_mem: None,
            oam_dma: Default::default(),
            vram_dma: Default::default(),
        };
        obj.update_model(model);

        obj
    }

    pub fn load_cart(&mut self, cart: Cart, model: Option<GbModel>) {
        self.cart = cart;
        self.flat_mem = None;
        self.update_model(model);
    }

    pub fn update_model(&mut self, model: Option<GbModel>) {
        if let Some(model) = model {
            self.set_model(model);
        } else {
            self.set_model(self.detect_gb_model());
        }
    }

    fn set_model(&mut self, model: GbModel) {
        self.io.ppu.lcd.set_model(model);
    }

    /// Creates with just array as memory. Use only for tests.
    pub fn with_bytes(bytes: Vec<u8>, io: Io) -> Self {
        let cart = Cart::empty();
        let mut obj = Self::new(cart, io, None);
        obj.flat_mem = Some(bytes);

        obj
    }

    pub fn read(&self, addr: u16) -> u8 {
        #[cfg(debug_assertions)]
        if let Some(test_bytes) = self.flat_mem.as_ref() {
            return test_bytes[addr as usize];
        }

        match addr {
            0x0000..=0x7FFF | 0xA000..=0xBFFF => self.cart.read(addr),
            VRAM_ADDR_START..=VRAM_ADDR_END => {
                if self.io.ppu.vram_blocked() {
                    return 0xFF;
                }

                self.io.ppu.video_ram.read(addr)
            }
            WRAM_START_ADDR..=WRAM_CGB_BANK_END_ADDR => self.io.ram.read_wram(addr),
            0xE000..=0xFDFF => {
                let mirrored_addr = addr - ECHO_MIRROR_OFFSET; // Redirect to WRAM (0xC000 - 0xDDFF)

                self.io.ram.read_wram(mirrored_addr)
            }
            0xFE00..=0xFE9F => {
                if self.oam_dma.is_transferring() {
                    return 0xFF;
                }

                self.io.ppu.oam_ram.read(addr)
            }
            0xFEA0..=0xFEFF => 0xFF,
            0xFF00..=0xFF7F => {
                if addr >= VRAM_DMA_ADDR_START
                    && addr <= VRAM_DMA_ADDR_END
                    && self.io.ppu.lcd.model == GbModel::Cgb
                {
                    return VramDma::read(self);
                }

                self.io.read(addr)
            }
            0xFF80..=0xFFFE => self.io.ram.read_hram(addr),
            0xFFFF => self.io.interrupts.ie,
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        #[cfg(debug_assertions)]
        if let Some(test_bytes) = self.flat_mem.as_mut() {
            test_bytes[addr as usize] = value;
            return;
        }

        if addr == LCD_DMA_ADDRESS {
            self.oam_dma.start(value);
        }

        match addr {
            0x0000..=0x7FFF | 0xA000..=0xBFFF => self.cart.write(addr, value),
            VRAM_ADDR_START..=VRAM_ADDR_END => {
                if self.io.ppu.vram_blocked() {
                    return;
                }

                self.io.ppu.video_ram.write(addr, value);
            }
            0xC000..=0xDFFF => self.io.ram.write_wram(addr, value),
            0xE000..=0xFDFF => {
                let mirrored_addr = addr - ECHO_MIRROR_OFFSET; // Redirect to WRAM (0xC000 - 0xDDFF)

                self.io.ram.write_wram(mirrored_addr, value);
            }
            0xFE00..=0xFE9F => {
                if self.oam_dma.is_active {
                    return;
                }

                self.io.ppu.oam_ram.write(addr, value)
            }
            0xFEA0..=0xFEFF => {}
            0xFF00..=0xFF7F => {
                if addr >= VRAM_DMA_ADDR_START
                    && addr <= VRAM_DMA_ADDR_END
                    && self.io.ppu.lcd.model == GbModel::Cgb
                {
                    VramDma::write(self, addr, value);
                    return;
                }

                self.io.write(addr, value);
            }
            0xFF80..=0xFFFE => self.io.ram.write_hram(addr, value),
            0xFFFF => self.io.interrupts.ie = value,
        }
    }

    #[inline(always)]
    pub fn detect_gb_model(&self) -> GbModel {
        match self.cart.data.cgb_flag {
            CgbFlag::CgbEnhanced => GbModel::Cgb,
            CgbFlag::DmgOnly => GbModel::Dmg,
            CgbFlag::CgbOnly => GbModel::Cgb,
        }
    }
}
