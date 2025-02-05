const VRAM_SIZE: usize = 0x2000;
const VRAM_ADDR_OFFSET: usize = 0xFF80;

#[derive(Debug, Clone)]
pub struct VRam {
    pub bytes: [u8; VRAM_SIZE],
}

impl VRam {
    pub fn new() -> Self {
        Self {
            bytes: [0; VRAM_SIZE],
        }
    }
    
    pub fn read(&self, addr: u16) -> u8 {
        self.bytes[addr as usize - VRAM_ADDR_OFFSET]
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        self.bytes[addr as usize - VRAM_ADDR_OFFSET] = val;
    }
}