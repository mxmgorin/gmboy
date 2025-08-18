use crate::cpu::Registers;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ConditionType {
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
    #[inline(always)]
    pub fn check_cond(registers: &Registers, cond: Option<ConditionType>) -> bool {
        let Some(cond) = cond else {
            return true;
        };

        match cond {
            ConditionType::C => registers.flags.get_c(),
            ConditionType::NC => !registers.flags.get_c(),
            ConditionType::Z => registers.flags.get_z(),
            ConditionType::NZ => !registers.flags.get_z(),
        }
    }
}
