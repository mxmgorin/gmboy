use crate::core::bus::Bus;
use crate::core::cpu::Registers;

pub enum Stack {}

impl Stack {
    pub fn push(registers: &mut Registers, bus: &mut Bus, val: u8) {
        registers.sp = registers.sp.wrapping_sub(1);
        bus.write(registers.sp, val);
    }

    pub fn pop(registers: &mut Registers, bus: &mut Bus) -> u8 {
        let val = bus.read(registers.sp);
        registers.sp = registers.sp.wrapping_add(1);

        val
    }

    pub fn push16(registers: &mut Registers, bus: &mut Bus, val: u16) {
        let shifted = (val >> 8) & 0xFF;
        Stack::push(registers, bus, shifted as u8);
        
        let shifted = val & 0xFF;
        Stack::push(registers, bus, shifted as u8);
    }

    pub fn _pop16(registers: &mut Registers, bus: &mut Bus) -> u16 {
        let lo = Stack::pop(registers, bus) as u16;
        let hi = Stack::pop(registers, bus) as u16;

        (hi << 8) | lo
    }
}
