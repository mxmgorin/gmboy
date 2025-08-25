use crate::cpu::flags::{Flags, FlagsCtx};
use crate::cpu::{Cpu, RegisterType};
use serde::{Deserialize, Serialize};

impl Cpu {
    #[inline(always)]
    pub fn fetch_execute_add_sp_e8(&mut self) {
        self.fetch_r_d8::<{ RegisterType::SP as u8 }>();
        let lhs = self.registers.sp;
        let rhs = self.step_ctx.fetched_data.value;

        self.clock.tick_m_cycles(2);
        self.registers.sp = self.registers.sp.wrapping_add(rhs as i8 as u16);
        self.registers
            .flags
            .set(FlagsCtx::AddSpE8(AddSpE8FlagsCtx { lhs, rhs }));
    }

    #[inline(always)]
    pub fn fetch_execute_add_r_r<const R1: u8, const R2: u8>(&mut self) {
        self.fetch_r_r::<R1, R2>();
        self.execute_add::<R1>();
    }

    #[inline(always)]
    pub fn fetch_execute_add_r_mr<const R1: u8, const R2: u8>(&mut self) {
        self.fetch_r_mr::<R1, R2>();
        self.execute_add::<R1>();
    }

    #[inline(always)]
    pub fn fetch_execute_add_r_d8<const R1: u8>(&mut self) {
        self.fetch_r_d8::<R1>();
        self.execute_add::<R1>();
    }

    #[inline(always)]
    pub fn execute_add<const R1: u8>(&mut self) {
        let lhs = self.registers.get_register::<R1>();
        let rhs = self.step_ctx.fetched_data.value;

        if RegisterType::from_u8(R1).is_16bit() {
            // but not for SP
            let result = lhs.wrapping_add(rhs);
            self.registers.set_register::<R1>(result);
            self.clock.tick_m_cycles(1);
            self.registers
                .flags
                .set(FlagsCtx::Add16(Add16FlagsCtx { lhs, rhs }));
        } else {
            let result = lhs.wrapping_add(rhs);
            self.registers.set_register::<R1>(result);
            self.registers.flags.set(FlagsCtx::Add8(Add8FlagsCtx {
                lhs: lhs as u8,
                rhs: rhs as u8,
                carry_in: 0,
                result: result as u8,
            }));
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Add8FlagsCtx {
    pub lhs: u8,
    pub rhs: u8,
    pub carry_in: u8,
    pub result: u8,
}

impl Add8FlagsCtx {
    #[inline(always)]
    pub fn apply(&self, flags: &mut Flags) {
        flags.set_z_inner(self.result == 0);
        flags.set_n_inner(false);
        flags.set_h_inner((self.lhs & 0xF) + (self.rhs & 0xF) + self.carry_in > 0xF);
        flags.set_c_inner((self.lhs as u16 + self.rhs as u16 + self.carry_in as u16) > 0xFF);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Add16FlagsCtx {
    pub lhs: u16,
    pub rhs: u16,
}

impl Add16FlagsCtx {
    #[inline(always)]
    pub fn apply(&self, flags: &mut Flags) {
        flags.set_n_inner(false);
        flags.set_h_inner(((self.lhs & 0x0FFF) + (self.rhs & 0x0FFF)) > 0x0FFF);
        flags.set_c_inner((self.lhs as u32 + self.rhs as u32) > 0xFFFF);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddSpE8FlagsCtx {
    pub lhs: u16,
    pub rhs: u16,
}

impl AddSpE8FlagsCtx {
    #[inline(always)]
    pub fn apply(&self, flags: &mut Flags) {
        flags.set_z_inner(false);
        flags.set_n_inner(false);
        flags.set_h_inner((self.lhs & 0xF) + (self.rhs & 0xF) > 0xF);
        flags.set_c_inner(((self.lhs & 0xFF) + (self.rhs & 0xFF)) > 0xFF);
    }
}
