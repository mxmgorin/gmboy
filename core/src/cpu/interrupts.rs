use crate::cpu::Cpu;
use serde::{Deserialize, Serialize};

const INTERRUPTS: [(u16, InterruptType); 5] = [
    (0x40, InterruptType::VBlank),
    (0x48, InterruptType::LCDStat),
    (0x50, InterruptType::Timer),
    (0x58, InterruptType::Serial),
    (0x60, InterruptType::Joypad),
];

impl Cpu {
    /// Costs 5 M-cycles when an interrupt is executed
    #[inline(always)]
    pub fn handle_interrupts(&mut self) {
        if self.clock.bus.io.interrupts.ime {
            if self.clock.bus.io.interrupts.has_pending() {
                self.handle_interrupt();
            }

            self.enabling_ime = false;
        }
    }

    #[inline(always)]
    fn handle_interrupt(&mut self) {
        self.clock.tick_m_cycles(2);

        self.is_halted = false;
        let [lo, hi] = u16::to_le_bytes(self.registers.pc);
        self.push(hi);
        let interrupt = self.clock.bus.io.interrupts.get_pending();
        self.push(lo);

        self.registers.pc = match interrupt {
            Some((addr, it)) => {
                self.clock.bus.io.interrupts.ack_interrupt(it);
                addr
            }
            None => {
                self.clock.bus.io.interrupts.ime = false;
                0x0000
            },
        };
        self.clock.tick_m_cycles(1);
    }
}

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
    pub ie: u8,
}

impl Interrupts {
    #[inline(always)]
    pub fn get_pending(&mut self) -> Option<(u16, InterruptType)> {
        for (address, interrupt_type) in INTERRUPTS {
            if self.is_pending(interrupt_type) {
                return Some((address, interrupt_type));
            }
        }

        None
    }
    #[inline(always)]
    pub const fn request_interrupt(&mut self, it: InterruptType) {
        self.int_flags |= it as u8;
    }

    #[inline(always)]
    pub fn has_pending(&self) -> bool {
        (self.int_flags & self.ie) != 0
    }

    #[inline(always)]
    pub fn ack_interrupt(&mut self, it: InterruptType) {
        let it = it as u8;

        self.int_flags &= !it;
        self.ime = false;
    }

    #[inline(always)]
    pub fn is_pending(&self, it: InterruptType) -> bool {
        let it = it as u8;
        let is_requested = self.int_flags & it != 0;
        let is_enabled = self.ie & it != 0;

        is_requested && is_enabled
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_check_interrupts_enabled() {
        let mut interrupts = Interrupts::default();
        interrupts.ie = 0xFF;

        for (_, interrupt_type) in INTERRUPTS {
            interrupts.request_interrupt(interrupt_type);
            assert!(interrupts.is_pending(interrupt_type));
        }
    }

    #[test]
    pub fn test_check_interrupts_disabled() {
        let interrupts = Interrupts::default();

        for (_, interrupt_type) in INTERRUPTS {
            assert!(!interrupts.is_pending(interrupt_type));
        }
    }

    #[test]
    pub fn test_interrupts() {
        let mut interrupts = Interrupts::default();

        for (_, interrupt_type) in INTERRUPTS {
            assert!(!interrupts.is_pending(interrupt_type));

            interrupts.ie |= interrupt_type as u8;
            interrupts.request_interrupt(interrupt_type);

            assert!(interrupts.is_pending(interrupt_type));
            interrupts.ack_interrupt(interrupt_type);

            assert!(!interrupts.is_pending(interrupt_type));
        }
    }
}
