use serde::{Deserialize, Serialize};
use crate::cpu::{Cpu, RegisterType};
use crate::cpu::flags::{Flags, FlagsCtx};

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
            self.registers
                .flags
                .set(FlagsCtx::Inc8(Inc8FlagsCtx { lhs, result }));
        }
    }

    #[inline]
    pub fn fetch_execute_inc_mr<const R1: u8>(&mut self) {
        self.fetch_mr::<R1>();
        let lhs = self.step_ctx.fetched_data.value as u8;
        let result = lhs.wrapping_add(1);

        self.write_to_memory(self.step_ctx.fetched_data.addr, result);
        self.registers
            .flags
            .set(FlagsCtx::Inc8(Inc8FlagsCtx{ lhs, result }));
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inc8FlagsCtx {
    pub lhs: u8,
    pub result: u8,
}

impl Inc8FlagsCtx {
    #[inline(always)]
    pub fn apply(&self, flags: &mut Flags) {
        flags.set_z_inner(self.result == 0);
        flags.set_n_inner(false);
        flags.set_h_inner((self.lhs & 0xF) + 1 > 0xF);
    }
}
