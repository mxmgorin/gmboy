use crate::core::cpu::Cpu;

pub enum Stack {}

impl Stack {
    pub fn push(cpu: &mut Cpu, val: u8) {
        cpu.registers.sp -= 1;
        cpu.bus.write(cpu.registers.sp, val);
    }

    fn pop(cpu: &mut Cpu) -> u8 {
        let val = cpu.bus.read(cpu.registers.sp);
        cpu.registers.sp += 1;

        val
    }

    pub fn push16(cpu: &mut Cpu, val: u16) {
        let shifted = (val >> 8) & 0xFF;
        Stack::push(cpu, shifted as u8);
        let shifted = val & 0xFF;
        Stack::push(cpu, shifted as u8);
    }

    pub fn pop16(cpu: &mut Cpu) -> u16 {
        let lo = Stack::pop(cpu) as u16;
        let hi = Stack::pop(cpu) as u16;

        (hi << 8) | lo
    }
}
