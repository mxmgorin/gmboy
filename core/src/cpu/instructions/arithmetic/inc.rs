use crate::cpu::{Cpu, RegisterType};

impl Cpu {
    #[inline(always)]
    pub fn fetch_execute_inc_r<const R1: u8>(&mut self) {
        self.fetch_r::<R1>();

        if RegisterType::from_u8(R1).is_16bit() {
            let lhs = self.step_ctx.fetched_data.value;
            let result = lhs.wrapping_add(1);
            self.registers.set_register::<R1>(result);
            self.clock.tick_m_cycles(1);
        } else {
            let lhs = self.step_ctx.fetched_data.value as u8;
            let result = lhs.wrapping_add(1);
            self.registers.set_register8::<R1>(result);
            self.registers.flags.op_inc8(lhs, result);
        }
    }

    #[inline]
    pub fn fetch_execute_inc_mr<const R1: u8>(&mut self) {
        self.fetch_mr::<R1>();
        let lhs = self.step_ctx.fetched_data.value as u8;
        let result = lhs.wrapping_add(1);

        self.write_to_memory(self.step_ctx.fetched_data.addr, result);
        self.registers.flags.op_inc8(lhs, result);
    }
}

