use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptType {
    VBlank = 1,
    LCDStat = 2,
    Timer = 4,
    Serial = 8,
    Joypad = 16,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Interrupts {
    /// Interrupt flags
    pub int_flags: u8,
    /// Interrupt master enable
    pub ime: bool,
    /// Interrupt enable register
    pub ie_register: u8,
}

impl Interrupts {
    #[inline]
    pub fn request_interrupt(&mut self, it: InterruptType) {
        self.int_flags |= it as u8;
    }

    #[inline]
    pub fn any_is_pending(&self) -> bool {
        (self.int_flags & self.ie_register) != 0
    }

    #[inline]
    pub fn acknowledge_interrupt(&mut self, it: InterruptType) {
        let it = it as u8;

        self.int_flags &= !it;
        self.ime = false;
    }

    #[inline]
    pub fn is_pending(&self, it: InterruptType) -> bool {
        let it = it as u8;
        let is_requested = self.int_flags & it != 0;
        let is_enabled = self.ie_register & it != 0;

        is_requested && is_enabled
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::cpu::INTERRUPT_HANDLERS;

    #[test]
    pub fn test_check_interrupts_enabled() {
        let mut interrupts = Interrupts::default();
        interrupts.ie_register = 0xFF;

        for (interrupt_type, _) in INTERRUPT_HANDLERS {
            interrupts.request_interrupt(interrupt_type);
            assert!(interrupts.is_pending(interrupt_type));
        }
    }

    #[test]
    pub fn test_check_interrupts_disabled() {
        let interrupts = Interrupts::default();

        for (interrupt_type, _) in INTERRUPT_HANDLERS {
            assert!(!interrupts.is_pending(interrupt_type));
        }
    }

    #[test]
    pub fn test_interrupts() {
        let mut interrupts = Interrupts::default();

        for (interrupt_type, _) in INTERRUPT_HANDLERS {
            assert!(!interrupts.is_pending(interrupt_type));

            interrupts.ie_register |= interrupt_type as u8;
            interrupts.request_interrupt(interrupt_type);

            assert!(interrupts.is_pending(interrupt_type));
            interrupts.acknowledge_interrupt(interrupt_type);

            assert!(!interrupts.is_pending(interrupt_type));
        }
    }
}
