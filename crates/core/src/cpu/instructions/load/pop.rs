use crate::cpu::{Cpu, RegisterType};

impl Cpu {
    #[inline(always)]
    pub fn fetch_execute_pop<const R1: u8>(&mut self) {
        let lo = self.pop() as u16;
        let hi = self.pop() as u16;
        let addr = (hi << 8) | lo;

        if RegisterType::from_u8(R1) == RegisterType::AF {
            self.registers.set_register::<R1>(addr & 0xFFF0);
        } else {
            self.registers.set_register::<R1>(addr);
        }
    }
}
