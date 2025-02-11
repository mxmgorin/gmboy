use crate::core::cart::Cart;
use crate::auxiliary::dma::{Dma};
use crate::auxiliary::io::Io;
use crate::auxiliary::ram::Ram;
use crate::ppu::lcd::LCD_DMA_ADDRESS;
use crate::ppu::ppu::Ppu;
use crate::ppu::vram::{VRAM_ADDR_END, VRAM_ADDR_START};

#[derive(Debug, PartialEq, Eq)]
pub enum BusAddrLocation {
    /// 0x0000 - 0x3FFF: 16 KiB ROM bank 00.From cartridge, usually a fixed bank.
    RomBank0,
    /// 0x4000 - 0x7FFF: 16 KiB ROM Bank 01–NN. From cartridge, switchable bank via mapper (if any).
    RomBank1,
    /// 8000 - 9FFF
    VRAM,
    /// 0xA000 - 0xBFFF: 8 KiB External RAM. From cartridge, switchable bank if any
    CartRam,
    /// 0xC000 - 0xCFFF
    WRamBank0,
    /// 0xD000 - 0xDFFF: 4 KiB Work RAM (WRAM). In CGB mode, switchable bank 1–7.
    WRamBank1To7,
    /// 0xE000 - 0xFDFF: Echo RAM (mirror of C000–DDFF). Nintendo says use of this area is prohibited.
    EchoRam,
    /// 0xFE00 - 0xFE9F: Object attribute memory (OAM)
    Oam,
    /// 0xFEA0 - 0xFEFF: Nintendo says use of this area is prohibited.
    Unusable,
    /// 0xFF00 - 0xFF7F
    IoRegisters,
    /// 0xFF80 - 0xFFFE: High RAM (HRAM). Aka ZeroPage
    HRam,
    /// 0xFFFF: Interrupt enable register.
    IeRegister,
}

impl From<u16> for BusAddrLocation {
    fn from(address: u16) -> Self {
        match address {
            0x0000..=0x3FFF => BusAddrLocation::RomBank0,
            0x4000..=0x7FFF => BusAddrLocation::RomBank1,
            VRAM_ADDR_START..=VRAM_ADDR_END => BusAddrLocation::VRAM,
            0xA000..=0xBFFF => BusAddrLocation::CartRam,
            0xC000..=0xCFFF => BusAddrLocation::WRamBank0,
            0xD000..=0xDFFF => BusAddrLocation::WRamBank1To7,
            0xE000..=0xFDFF => BusAddrLocation::EchoRam,
            0xFE00..=0xFE9F => BusAddrLocation::Oam,
            0xFEA0..=0xFEFF => BusAddrLocation::Unusable,
            0xFF00..=0xFF7F => BusAddrLocation::IoRegisters,
            0xFF80..=0xFFFE => BusAddrLocation::HRam,
            0xFFFF => BusAddrLocation::IeRegister,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Bus {
    cart: Cart,
    ram: Ram,
    pub io: Io,
    pub ppu: Ppu,
    flat_mem: Option<Vec<u8>>,
    dma: Dma,
}

impl Bus {
    pub fn new(cart: Cart) -> Self {
        Self {
            cart,
            ram: Ram::new(),
            io: Io::new(),
            ppu: Ppu::new(),
            flat_mem: None,
            dma: Default::default(),
        }
    }

    /// Creates with just array as memory. Use only for tests.
    pub fn flat_mem(bytes: Vec<u8>) -> Self {
        let cart = Cart::new(vec![0; 0x2000]).unwrap();
        let mut obj = Self::new(cart);
        obj.flat_mem = Some(bytes);

        obj
    }

    pub fn read(&self, address: u16) -> u8 {
        #[cfg(debug_assertions)]
        if let Some(test_bytes) = self.flat_mem.as_ref() {
            return test_bytes[address as usize];
        }

        let location = BusAddrLocation::from(address);

        match location {
            BusAddrLocation::Oam => {
                if self.dma.is_active {
                    return 0xFF;
                }

                self.ppu.oam_read(address)
            }
            BusAddrLocation::VRAM => self.ppu.vram_read(address),
            BusAddrLocation::RomBank0 | BusAddrLocation::RomBank1 | BusAddrLocation::CartRam => {
                self.cart.read(address)
            }
            BusAddrLocation::WRamBank0 | BusAddrLocation::WRamBank1To7 => {
                self.ram.w_ram_read(address)
            }
            BusAddrLocation::EchoRam | BusAddrLocation::Unusable => 0,
            BusAddrLocation::IoRegisters => self.io.read(address),
            BusAddrLocation::HRam => self.ram.h_ram_read(address),
            BusAddrLocation::IeRegister => self.io.interrupts.ie_register,
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

        let location = BusAddrLocation::from(address);

        match location {
            BusAddrLocation::VRAM => self.ppu.vram_write(address, value),
            BusAddrLocation::EchoRam | BusAddrLocation::Unusable => {}
            BusAddrLocation::Oam => {
                if self.dma.is_active {
                    return;
                }

                self.ppu.oam_write(address, value)
            }
            BusAddrLocation::RomBank0 | BusAddrLocation::RomBank1 | BusAddrLocation::CartRam => {
                self.cart.write(address, value)
            }
            BusAddrLocation::WRamBank0 | BusAddrLocation::WRamBank1To7 => {
                self.ram.w_ram_write(address, value)
            }
            BusAddrLocation::IoRegisters => self.io.write(address, value),
            BusAddrLocation::HRam => self.ram.h_ram_write(address, value),
            BusAddrLocation::IeRegister => self.io.interrupts.ie_register = value,
        }
    }

    pub fn dma_tick(&mut self) {
        if !self.dma.is_active {
            return;
        }

        if self.dma.start_delay > 0 {
            self.dma.start_delay -= 1;
            return;
        }

        let value = self.read(self.dma.address as u16 * 0x100) + self.dma.current_byte;
        self.ppu.oam_write(self.dma.current_byte as u16, value);
        self.dma.current_byte += self.dma.current_byte.wrapping_add(1);
        self.dma.is_active = self.dma.current_byte < 0xA0; // 160
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ie_register() {
        let location: BusAddrLocation = 0xFFFF.into();

        assert_eq!(location, BusAddrLocation::IeRegister);
    }
}
