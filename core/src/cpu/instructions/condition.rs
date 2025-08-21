use crate::cpu::Cpu;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ConditionType {
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

impl ConditionType {
    pub fn from_u8(n: u8) -> Self {
        match n {
            0 => ConditionType::None,
            1 => ConditionType::NZ,
            2 => ConditionType::Z,
            3 => ConditionType::NC,
            4 => ConditionType::C,
            _ => panic!("invalid 8-bit ConditionType"),
        }
    }
}

impl Cpu {
    #[inline(always)]
    pub fn check_cond(&mut self, cond: ConditionType) -> bool {
        match cond {
            ConditionType::C => self.registers.flags.get_c(),
            ConditionType::NC => !self.registers.flags.get_c(),
            ConditionType::Z => self.registers.flags.get_z(),
            ConditionType::NZ => !self.registers.flags.get_z(),
            ConditionType::None => true,
        }
    }
}
