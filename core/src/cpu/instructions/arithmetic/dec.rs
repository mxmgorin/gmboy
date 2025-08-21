use crate::cpu::{Cpu, RegisterType};

impl Cpu {
    #[inline(always)]
    pub fn fetch_execute_dec_r<const R1: u8>(&mut self) {
        self.fetch_r::<R1>();
        let mut value = self.step_ctx.fetched_data.value.wrapping_sub(1);
        let r1 = RegisterType::from_u8(R1);

        if r1.is_16bit() {
            self.clock.m_cycles(1);
        }

        self.registers.set_register(r1, value);
        value = self.registers.read_register(r1);

        self.set_flags_dec(value);
    }

    #[inline(always)]
    pub fn fetch_execute_dec_mr_hl(&mut self) {
        self.fetch_mr_hl();
        let value = self.step_ctx.fetched_data.value.wrapping_sub(1);
        self.write_to_memory(self.step_ctx.fetched_data.addr, value as u8);
        self.set_flags_dec(value);
    }

    fn set_flags_dec(&mut self, value: u16) {
        if (self.step_ctx.opcode & 0x0B) == 0x0B {
            return;
        }

        self.registers.flags.set_z(value == 0);
        self.registers.flags.set_n(true);
        self.registers.flags.set_h((value & 0x0F) == 0x0F);
    }
}
