const W_RAM_SIZE: usize = 0x2000;
const H_RAM_SIZE: usize = 0x80;
const W_RAM_ADDR_START: usize = 0xC000;
const H_RAM_ADDR_START: usize = 0xFF80;

#[derive(Debug, Clone)]
pub struct Ram {
    working_ram: [u8; W_RAM_SIZE],
    high_ram: [u8; H_RAM_SIZE],
}

impl Default for Ram {
    fn default() -> Self {
        Self {
            working_ram: [0; W_RAM_SIZE],
            high_ram: [0; H_RAM_SIZE],
        }
    }
}

impl Ram {
    pub fn working_ram_read(&self, addr: u16) -> u8 {
        self.working_ram[normalize_w_addr(addr)]
    }

    pub fn working_ram_write(&mut self, addr: u16, val: u8) {
        self.working_ram[normalize_w_addr(addr)] = val;
    }

    pub fn high_ram_read(&self, addr: u16) -> u8 {
        self.high_ram[normalize_h_addr(addr)]
    }

    pub fn high_ram_write(&mut self, addr: u16, val: u8) {
        self.high_ram[normalize_h_addr(addr)] = val;
    }
}

fn normalize_w_addr(addr: u16) -> usize {
    addr as usize - W_RAM_ADDR_START
}

fn normalize_h_addr(addr: u16) -> usize {
    addr as usize - H_RAM_ADDR_START
}
