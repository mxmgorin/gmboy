use crate::cpu::{Cpu, CpuCallback};

pub enum Stack {}

impl Stack {
    /// Costs 1 M-Cycle.
    pub fn push(cpu: &mut Cpu, value: u8, callback: &mut impl CpuCallback) {
        cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);
        callback.get_bus_mut().write(cpu.registers.sp, value);
        callback.m_cycles(1);
    }

    /// Costs 1 M-Cycle.
    pub fn pop(cpu: &mut Cpu, callback: &mut impl CpuCallback) -> u8 {
        let value = callback.get_bus_mut().read(cpu.registers.sp);
        cpu.registers.sp = cpu.registers.sp.wrapping_add(1);
        callback.m_cycles(1);

        value
    }

    /// Costs 2 M-cycles.
    pub fn push16(cpu: &mut Cpu, val: u16, callback: &mut impl CpuCallback) {
        let high_byte = (val >> 8) & 0xFF;
        Stack::push(cpu, high_byte as u8, callback);

        let low_byte = val & 0xFF;
        Stack::push(cpu, low_byte as u8, callback);
    }

    /// Costs 2 M-cycles.
    pub fn _pop16(cpu: &mut Cpu, callback: &mut impl CpuCallback) -> u16 {
        let lo = Stack::pop(cpu, callback) as u16;
        let hi = Stack::pop(cpu, callback) as u16;

        (hi << 8) | lo
    }
}
