use crate::cpu::flags::{Flags, FlagsCtx, FlagsCtxData, FlagsOp};
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
            self.registers.flags.set(FlagsCtx::inc8(lhs, result));
        }
    }

    #[inline]
    pub fn fetch_execute_inc_mr<const R1: u8>(&mut self) {
        self.fetch_mr::<R1>();
        let lhs = self.step_ctx.fetched_data.value as u8;
        let result = lhs.wrapping_add(1);

        self.write_to_memory(self.step_ctx.fetched_data.addr, result);
        self.registers.flags.set(FlagsCtx::inc8(lhs, result));
    }
}

impl FlagsOp {
    #[inline(always)]
    pub fn inc8(data: FlagsCtxData, flags: &mut Flags) {
        flags.set_z_inner(data.result == 0);
        flags.set_n_inner(false);
        flags.set_h_inner((data.lhs & 0xF) + 1 > 0xF);
    }
}
