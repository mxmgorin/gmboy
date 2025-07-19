use crate::auxiliary::dma::Dma;
use crate::auxiliary::io::Io;
use crate::auxiliary::ram::Ram;
use crate::cart::Cart;
use crate::ppu::lcd::LCD_DMA_ADDRESS;
use crate::ppu::oam::OamRam;
use crate::ppu::vram::{VideoRam, VRAM_ADDR_END, VRAM_ADDR_START};
use serde::{Deserialize, Serialize};

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

pub const ECHO_MIRROR_OFFSET: u16 = 0x2000;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bus {
    pub cart: Cart,
    pub ram: Ram,
    pub io: Io,
    flat_mem: Option<Vec<u8>>,
    pub dma: Dma,
    pub video_ram: VideoRam,
    pub oam_ram: OamRam,
}

impl Bus {
    pub fn clone_without_cart(&self) -> Self {
        Self {
            cart: Cart::default(),
            ram: self.ram.clone(),
            io: self.io.clone(),
            flat_mem: self.flat_mem.clone(),
            dma: self.dma.clone(),
            video_ram: self.video_ram.clone(),
            oam_ram: self.oam_ram.clone(),
        }
    }

    pub fn new(cart: Cart, io: Io) -> Self {
        Self {
            cart,
            ram: Ram::default(),
            io,
            flat_mem: None,
            dma: Default::default(),
            video_ram: Default::default(),
            oam_ram: Default::default(),
        }
    }

    /// Creates with just array as memory. Use only for tests.
    pub fn with_bytes(bytes: Vec<u8>, io: Io) -> Self {
        let cart = Cart::new(vec![0; 0x2000].into_boxed_slice()).unwrap();
        let mut obj = Self::new(cart, io);
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
                if self.dma.is_transferring() {
                    return 0xFF;
                }

                self.oam_ram.read(address)
            }
            BusAddrLocation::VRAM => self.video_ram.read(address),
            BusAddrLocation::RomBank0 | BusAddrLocation::RomBank1 | BusAddrLocation::CartRam => {
                self.cart.read(address)
            }
            BusAddrLocation::WRamBank0 | BusAddrLocation::WRamBank1To7 => {
                self.ram.working_ram_read(address)
            }
            BusAddrLocation::EchoRam => {
                let mirrored_addr = address - ECHO_MIRROR_OFFSET; // Redirect to WRAM (0xC000 - 0xDDFF)
                self.ram.working_ram_read(mirrored_addr)
            }
            BusAddrLocation::Unusable => 0xFF,
            BusAddrLocation::IoRegisters => self.io.read(address),
            BusAddrLocation::HRam => self.ram.high_ram_read(address),
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
            BusAddrLocation::VRAM => self.video_ram.write(address, value),
            BusAddrLocation::EchoRam => {
                let mirrored_addr = address - ECHO_MIRROR_OFFSET; // Redirect to WRAM (0xC000 - 0xDDFF)
                self.ram.working_ram_write(mirrored_addr, value);
            }
            BusAddrLocation::Unusable => {}
            BusAddrLocation::Oam => {
                if self.dma.is_active {
                    return;
                }

                self.oam_ram.write(address, value)
            }
            BusAddrLocation::RomBank0 | BusAddrLocation::RomBank1 | BusAddrLocation::CartRam => {
                self.cart.write(address, value)
            }
            BusAddrLocation::WRamBank0 | BusAddrLocation::WRamBank1To7 => {
                self.ram.working_ram_write(address, value)
            }
            BusAddrLocation::IoRegisters => self.io.write(address, value),
            BusAddrLocation::HRam => self.ram.high_ram_write(address, value),
            BusAddrLocation::IeRegister => self.io.interrupts.ie_register = value,
        }
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
