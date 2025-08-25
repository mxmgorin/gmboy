use crate::cpu::Cpu;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum JumpCondition {
    None = 0,
    /// Non-zero: Execute if Z is not set.
    NZ = 1,
    /// Zero: Execute if Z is set.
    Z = 2,
    /// Non-carry: Execute if C is not set.
    NC = 3,
    /// Carry: Execute if C is set.
    C = 4,
}

impl JumpCondition {
    pub fn from_u8(n: u8) -> Self {
        match n {
            0 => JumpCondition::None,
            1 => JumpCondition::NZ,
            2 => JumpCondition::Z,
            3 => JumpCondition::NC,
            4 => JumpCondition::C,
            _ => panic!("invalid 8-bit JumpCondition"),
        }
    }
}

impl Cpu {
    #[inline(always)]
    pub fn check_cond<const C: u8>(&mut self) -> bool {
        match JumpCondition::from_u8(C) {
            JumpCondition::C => self.registers.flags.get_c(),
            JumpCondition::NC => !self.registers.flags.get_c(),
            JumpCondition::Z => self.registers.flags.get_z(),
            JumpCondition::NZ => !self.registers.flags.get_z(),
            JumpCondition::None => true,
        }
    }
}
