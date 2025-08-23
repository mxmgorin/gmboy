use crate::cpu::{Cpu, RegisterType};

impl Cpu {
    #[inline(always)]
    pub fn fetch_execute_dec_r<const R1: u8>(&mut self) {
        self.fetch_r::<R1>();
        let mut value = self.step_ctx.fetched_data.value.wrapping_sub(1);
        self.registers.set_register::<R1>(value);
        value = self.registers.read_register::<R1>();

        if RegisterType::from_u8(R1).is_16bit() {
            self.clock.tick_m_cycles(1);
        } else {
            self.set_flags_dec(value);
        }
    }

    #[inline(always)]
    pub fn fetch_execute_dec_mr<const R1: u8>(&mut self) {
        self.fetch_mr::<R1>();
        let value = self.step_ctx.fetched_data.value.wrapping_sub(1);
        self.write_to_memory(self.step_ctx.fetched_data.addr, value as u8);
        self.set_flags_dec(value);
    }

    #[inline(always)]
    fn set_flags_dec(&mut self, value: u16) {
        self.registers.flags.set_z(value == 0);
        self.registers.flags.set_n(true);
        self.registers.flags.set_h((value & 0x0F) == 0x0F);
    }
}
