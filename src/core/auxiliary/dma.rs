pub const DMA_ADDRESS: u16 = 0xFF46;

#[derive(Debug, Clone, Default)]
pub struct Dma {
    pub is_active: bool,
    pub current_byte: u8,
    pub address: u8,
    pub start_delay: u8,
}

impl Dma {
    pub fn start(&mut self, address: u8) {
        self.is_active = true;
        self.start_delay = 2;
        self.current_byte = 0x00;
        self.address = address;
    }
}