use crate::cpu::{Cpu, RegisterType};

impl Cpu {
    #[inline(always)]
    pub fn fetch_execute_inc_r<const R1: u8>(&mut self) {
        let r1 = RegisterType::from_u8(R1);
        self.fetch_r::<R1>();
        let value = self.step_ctx.fetched_data.value.wrapping_add(1);

        if r1.is_16bit() {
            self.clock.tick_m_cycles(1);
        }

        self.registers.set_register::<R1>(value);
        let value = self.registers.read_register::<R1>();
        self.set_flags(value);
    }

    #[inline]
    pub fn fetch_execute_inc_mr<const R1: u8>(&mut self) {
        self.fetch_mr::<R1>();
        let mut value = self.step_ctx.fetched_data.value.wrapping_add(1);
        value &= 0xFF; // Ensure it fits into 8 bits
        self.write_to_memory(self.step_ctx.fetched_data.addr, value as u8);
        self.set_flags(value);
    }

    #[inline(always)]
    fn set_flags(&mut self, value: u16) {
        if (self.step_ctx.opcode & 0x03) == 0x03 {
            return;
        }

        self.registers.flags.set_z(value == 0);
        self.registers.flags.set_n(false);
        self.registers.flags.set_h((value & 0x0F) == 0);
    }
}
