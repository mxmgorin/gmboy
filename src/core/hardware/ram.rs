const W_RAM_SIZE: usize = 0x2000;
const H_RAM_SIZE: usize = 0x80;
const W_RAM_ADDR_OFFSET: usize = 0xC000;
const H_RAM_ADDR_OFFSET: usize = 0xFF80;

#[derive(Debug, Clone)]
pub struct Ram {
    w_ram: [u8; W_RAM_SIZE],
    h_ram: [u8; H_RAM_SIZE],
}

impl Ram {
    pub fn new() -> Self {
        Self {
            w_ram: [0; W_RAM_SIZE],
            h_ram: [0; H_RAM_SIZE],
        }
    }

    pub fn w_ram_read(&self, addr: u16) -> u8 {
        self.w_ram[normalize_w_addr(addr)]
    }

    pub fn w_ram_write(&mut self, addr: u16, val: u8) {
        self.w_ram[normalize_w_addr(addr)] = val;
    }

    pub fn h_ram_read(&self, addr: u16) -> u8 {
        self.h_ram[normalize_h_addr(addr)]
    }

    pub fn h_ram_write(&mut self, addr: u16, val: u8) {
        self.h_ram[normalize_h_addr(addr)] = val;
    }
}

fn normalize_w_addr(addr: u16) -> usize {
    addr as usize - W_RAM_ADDR_OFFSET
}

fn normalize_h_addr(addr: u16) -> usize {
    addr as usize - H_RAM_ADDR_OFFSET
}
