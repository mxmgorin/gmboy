use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn fetch_execute_ldh_a8_r<const R2: u8>(&mut self) {
        let (addr, val) = self.read_a8_r8::<R2>();
        self.write_to_memory(addr as u16 | 0xFF00, val);
    }

    #[inline(always)]
    pub fn fetch_execute_ldh_mr_r<const R1: u8, const R2: u8>(&mut self) {
        let (addr, val) = self.read_mr_r::<R1, R2>();
        self.write_to_memory(addr | 0xFF00, val as u8);
    }

    #[inline(always)]
    pub fn fetch_execute_ldh_r_ha8<const R1: u8>(&mut self) {
        let data = self.read_ha8();
        self.registers.set_register8::<R1>(data);
    }

    #[inline(always)]
    pub fn fetch_execute_ldh_r_hmr<const R1: u8, const R2: u8>(&mut self) {
        let value = self.read_hmr::<R2>();
        self.registers.set_register8::<R1>(value);
    }
}
