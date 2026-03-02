use crate::auxiliary::dma::Dma;
use crate::auxiliary::io::Io;
use crate::auxiliary::ram::{WRAM_CGB_BANK_END_ADDR, WRAM_START_ADDR};
use crate::cart::header::CgbFlag;
use crate::cart::Cart;
use crate::emu::config::GbModel;
use crate::ppu::lcd::LCD_DMA_ADDRESS;
use crate::ppu::vram::{VRAM_ADDR_END, VRAM_ADDR_START};
use serde::{Deserialize, Serialize};

pub const ECHO_MIRROR_OFFSET: u16 = 0x2000;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Bus {
    pub cart: Cart,
    pub io: Io,
    pub dma: Dma,
    flat_mem: Option<Vec<u8>>,
}

impl Bus {
    pub fn clone_empty_cart(&self) -> Self {
        Self {
            cart: Cart::empty(),
            io: self.io.clone(),
            flat_mem: self.flat_mem.clone(),
            dma: self.dma.clone(),
        }
    }

    pub fn new(cart: Cart, io: Io) -> Self {
        Self {
            cart,
            io,
            flat_mem: None,
            dma: Default::default(),
        }
    }

    pub fn load_cart(&mut self, cart: Cart, model: GbModel) {
        self.cart = cart;
        self.flat_mem = None;
        self.io.ppu.lcd.cgb_flag = self.detect_cgb_flag(model);
    }

    /// Creates with just array as memory. Use only for tests.
    pub fn with_bytes(bytes: Vec<u8>, io: Io) -> Self {
        let cart = Cart::empty();
        let mut obj = Self::new(cart, io);
        obj.flat_mem = Some(bytes);

        obj
    }

    pub fn read(&self, address: u16) -> u8 {
        #[cfg(debug_assertions)]
        if let Some(test_bytes) = self.flat_mem.as_ref() {
            return test_bytes[address as usize];
        }

        match address {
            0x0000..=0x7FFF | 0xA000..=0xBFFF => self.cart.read(address),
            VRAM_ADDR_START..=VRAM_ADDR_END => self.io.ppu.video_ram.read(address),
            WRAM_START_ADDR..=WRAM_CGB_BANK_END_ADDR => self.io.ram.read_wram(address),
            0xE000..=0xFDFF => {
                let mirrored_addr = address - ECHO_MIRROR_OFFSET; // Redirect to WRAM (0xC000 - 0xDDFF)

                self.io.ram.read_wram(mirrored_addr)
            }
            0xFE00..=0xFE9F => {
                if self.dma.is_transferring() {
                    return 0xFF;
                }

                self.io.ppu.oam_ram.read(address)
            }
            0xFEA0..=0xFEFF => 0xFF,
            0xFF00..=0xFF7F => self.io.read(address),
            0xFF80..=0xFFFE => self.io.ram.read_hram(address),
            0xFFFF => self.io.interrupts.ie,
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        #[cfg(debug_assertions)]
        if let Some(test_bytes) = self.flat_mem.as_mut() {
            test_bytes[address as usize] = value;
            return;
        }

        if address == LCD_DMA_ADDRESS {
            self.dma.start(value);
        }

        match address {
            0x0000..=0x7FFF | 0xA000..=0xBFFF => self.cart.write(address, value),
            VRAM_ADDR_START..=VRAM_ADDR_END => self.io.ppu.video_ram.write(address, value),
            0xC000..=0xDFFF => self.io.ram.write_wram(address, value),
            0xE000..=0xFDFF => {
                let mirrored_addr = address - ECHO_MIRROR_OFFSET; // Redirect to WRAM (0xC000 - 0xDDFF)

                self.io.ram.write_wram(mirrored_addr, value);
            }
            0xFE00..=0xFE9F => {
                if self.dma.is_active {
                    return;
                }

                self.io.ppu.oam_ram.write(address, value)
            }
            0xFEA0..=0xFEFF => {}
            0xFF00..=0xFF7F => self.io.write(address, value),
            0xFF80..=0xFFFE => self.io.ram.write_hram(address, value),
            0xFFFF => self.io.interrupts.ie = value,
        }
    }

    #[inline(always)]
    pub fn detect_cgb_flag(&self, model: GbModel) -> CgbFlag {
        match model {
            GbModel::Auto => self.cart.data.cgb_flag,
            GbModel::Dmg => CgbFlag::DmgOnly,
            GbModel::Cgb => CgbFlag::CgbOnly,
        }
    }
}
