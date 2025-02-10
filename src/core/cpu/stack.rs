use crate::cpu::Cpu;

pub enum Stack {}

impl Stack {
    /// Costs 1 M-Cycle.
    pub fn push(cpu: &mut Cpu, value: u8) {
        cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);
        cpu.bus.write(cpu.registers.sp, value);
        cpu.clock.m_cycles(1, &mut cpu.bus);
    }

    /// Costs 1 M-Cycle.
    pub fn pop(cpu: &mut Cpu) -> u8 {
        let value = cpu.bus.read(cpu.registers.sp);
        cpu.registers.sp = cpu.registers.sp.wrapping_add(1);
        cpu.clock.m_cycles(1, &mut cpu.bus);

        value
    }

    /// Costs 2 M-cycles.
    pub fn push16(cpu: &mut Cpu, val: u16) {
        let high_byte = (val >> 8) & 0xFF;
        Stack::push(cpu, high_byte as u8);

        let low_byte = val & 0xFF;
        Stack::push(cpu, low_byte as u8);
    }

    /// Costs 2 M-cycles.
    pub fn _pop16(cpu: &mut Cpu) -> u16 {
        let lo = Stack::pop(cpu) as u16;
        let hi = Stack::pop(cpu) as u16;

        (hi << 8) | lo
    }
}
