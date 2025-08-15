use crate::cpu::{Cpu};

/// Methods to stack operations
impl Cpu {
    /// Costs 1 M-Cycle.
    pub fn push(&mut self, value: u8) {
        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.clock.bus.write(self.registers.sp, value);
        self.clock.m_cycles(1);
    }

    /// Costs 1 M-Cycle.
    pub fn pop(&mut self) -> u8 {
        let value = self.clock.bus.read(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(1);
        self.clock.m_cycles(1);

        value
    }

    /// Costs 2 M-cycles.
    pub fn push16(&mut self, val: u16) {
        let high_byte = (val >> 8) & 0xFF;
        self.push(high_byte as u8);

        let low_byte = val & 0xFF;
        self.push(low_byte as u8);
    }
}
