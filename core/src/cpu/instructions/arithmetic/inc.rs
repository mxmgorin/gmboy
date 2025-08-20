use crate::cpu::instructions::DataDestination;
use crate::cpu::{Cpu, RegisterType};

impl Cpu {
    #[inline(always)]
    pub fn fetch_execute_inc_r<const R1: u8>(&mut self) {
        let r1 = RegisterType::from_u8(R1);
        self.fetch_r::<R1>();
        let value = self.step_ctx.fetched_data.value.wrapping_add(1);

        if r1.is_16bit() {
            self.clock.m_cycles(1);
        }

        self.registers.set_register(r1, value);
        let value = self.registers.read_register(r1);
        self.set_flags_inc(value);
    }

    #[inline]
    pub fn fetch_execute_inc_mr_hl(&mut self) {
        self.fetch_mr_hl();
        let mut value = self.step_ctx.fetched_data.value.wrapping_add(1);
        let DataDestination::Memory(addr) = self.step_ctx.fetched_data.dest else {
            unreachable!()
        };

        // uses only HL
        value &= 0xFF; // Ensure it fits into 8 bits
        self.write_to_memory(addr, value as u8);
        self.set_flags_inc(value);
    }

    #[inline(always)]
    fn set_flags_inc(&mut self, value: u16) {
        if (self.step_ctx.opcode & 0x03) == 0x03 {
            return;
        }

        self.registers.flags.set_z(value == 0);
        self.registers.flags.set_n(false);
        self.registers.flags.set_h((value & 0x0F) == 0);
    }
}
