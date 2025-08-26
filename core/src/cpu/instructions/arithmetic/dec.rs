use crate::cpu::flags::{Flags, FlagsCtx, FlagsData, FlagsOp};
use crate::cpu::{Cpu, RegisterType};

impl Cpu {
    #[inline(always)]
    pub fn fetch_execute_dec_r<const R1: u8>(&mut self) {
        self.fetch_r::<R1>();

        if RegisterType::from_u8(R1).is_16bit() {
            let value = self.step_ctx.fetched_data.value.wrapping_sub(1);
            self.registers.set_register::<R1>(value);
            self.clock.tick_m_cycles(1);
        } else {
            let lhs = self.step_ctx.fetched_data.value as u8;
            let result = lhs.wrapping_sub(1);
            self.registers.set_register8::<R1>(result);
            self.registers.flags.set(FlagsCtx::new_dec8(lhs, result));
        }
    }

    #[inline(always)]
    pub fn fetch_execute_dec_mr<const R1: u8>(&mut self) {
        self.fetch_mr::<R1>();
        let lhs = self.step_ctx.fetched_data.value as u8;
        let result = lhs.wrapping_sub(1);
        self.write_to_memory(self.step_ctx.fetched_data.addr, result);
        self.registers.flags.set(FlagsCtx::new_dec8(lhs, result));
    }
}

impl FlagsOp {
    pub fn dec8(data: FlagsData, flags: &mut Flags) {
        flags.set_z_raw(data.result == 0);
        flags.set_n_raw(true);
        flags.set_h_raw((data.lhs & 0xF) == 0);
    }
}
