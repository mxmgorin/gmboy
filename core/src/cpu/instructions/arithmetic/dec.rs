use serde::{Deserialize, Serialize};
use crate::cpu::flags::{Flags, FlagsCtx};
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
            self.registers
                .flags
                .set(FlagsCtx::Dec8(Dec8FlagsCtx { lhs, result }));
        }
    }

    #[inline(always)]
    pub fn fetch_execute_dec_mr<const R1: u8>(&mut self) {
        self.fetch_mr::<R1>();
        let lhs = self.step_ctx.fetched_data.value as u8;
        let result = lhs.wrapping_sub(1);
        self.write_to_memory(self.step_ctx.fetched_data.addr, result);
        self.registers
            .flags
            .set(FlagsCtx::Dec8(Dec8FlagsCtx { lhs, result }));
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dec8FlagsCtx {
    pub lhs: u8,
    pub result: u8,
}

impl Dec8FlagsCtx {
    #[inline(always)]
    pub fn apply(&self, flags: &mut Flags) {
        flags.set_z_inner(self.result == 0);
        flags.set_n_inner(true);
        flags.set_h_inner((self.lhs & 0xF) == 0);
    }
}