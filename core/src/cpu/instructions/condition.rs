use crate::cpu::Cpu;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum JumpCondition {
    None,
    /// Non-zero: Execute if Z is not set.
    NZ,
    /// Zero: Execute if Z is set.
    Z,
    /// Non-carry: Execute if C is not set.
    NC,
    /// Carry: Execute if C is set.
    C,
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
    pub fn check_cond(&mut self, cond: JumpCondition) -> bool {
        match cond {
            JumpCondition::C => self.registers.flags.get_c(),
            JumpCondition::NC => !self.registers.flags.get_c(),
            JumpCondition::Z => self.registers.flags.get_z(),
            JumpCondition::NZ => !self.registers.flags.get_z(),
            JumpCondition::None => true,
        }
    }
}
