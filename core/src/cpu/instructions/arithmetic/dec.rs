use crate::cpu::{Cpu, RegisterType};

impl Cpu {
    #[inline(always)]
    pub fn fetch_execute_dec_r<const R1: u8>(&mut self) {
        if RegisterType::from_u8(R1).is_16bit() {
            let lhs = self.registers.get_register::<R1>();
            let result = lhs.wrapping_sub(1);
            self.registers.set_register::<R1>(result);
            self.clock.tick_m_cycles(1);
        } else {
            let lhs = self.registers.get_register8::<R1>();
            let result = lhs.wrapping_sub(1);
            self.registers.set_register8::<R1>(result);
            self.registers.flags.op_dec8(lhs, result);
        }
    }

    #[inline(always)]
    pub fn fetch_execute_dec_mr<const R1: u8>(&mut self) {
        let (addr, lhs) = self.read_mr_addr_val::<R1>();
        let result = lhs.wrapping_sub(1);
        self.write_to_memory(addr, result);
        self.registers.flags.op_dec8(lhs, result);
    }
}
