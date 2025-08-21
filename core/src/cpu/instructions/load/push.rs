use crate::cpu::{Cpu, RegisterType};

impl Cpu {
    #[inline]
    pub fn fetch_execute_push<const R1: u8>(&mut self) {
        let r1 = RegisterType::from_u8(R1);
        self.fetch_r::<R1>();
        self.clock.m_cycles(1);

        let hi = (self.registers.read_register(r1) >> 8) & 0xFF;
        self.push(hi as u8);

        let lo = self.registers.read_register(r1) & 0xFF;
        self.push(lo as u8);
    }
}
