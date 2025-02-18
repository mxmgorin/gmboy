use crate::auxiliary::clock::T_CYCLES_PER_M_CYCLE;
use crate::cpu::interrupts::{InterruptType, Interrupts};

pub const TIMER_DIV_ADDRESS: u16 = 0xFF04;
pub const TIMER_TIMA_ADDRESS: u16 = 0xFF05;
pub const TIMER_TMA_ADDRESS: u16 = 0xFF06;
pub const TIMER_TAC_ADDRESS: u16 = 0xFF07;
pub const TIMER_TAC_CYCLES: [usize; 4] = [256, 4, 16, 64];
pub const TIMER_TAC_UNUSED_MASK: u8 = 0b11111_000;

#[derive(Debug, Clone)]
pub struct Timer {
    prev_div: u16,
    div: u16,
    tima: u8,
    tma: u8,
    tac: u8,
    tima_overflow: bool,
    interrupt_delay: usize,
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            // This value depends on the model. For the original Game Boy (DMG) it is 0xABCC.
            div: 0xABCC,
            prev_div: 0xABCC,
            tima: 0,
            tma: 0,
            tac: 0,
            tima_overflow: false,
            interrupt_delay: 0,
        }
    }
}

impl Timer {
    pub fn tick(&mut self, interrupts: &mut Interrupts) {
        // TIMA overflowed during the last cycle
        if self.tima_overflow {
            if self.interrupt_delay == 0 {
                self.tima_overflow = false;
                self.tima = self.tma;
                interrupts.request_interrupt(InterruptType::Timer);
            } else {
                self.interrupt_delay -= 1;
            }
        }

        self.div = self.div.wrapping_add(1);

        // detect when bit N transitions from 1 to 0 between the previous DIV and current DIV values
        let timer_update = match self.tac & 0b11 {
            // 0b00 (4096 Hz): div bit 9, increment every 256 M-cycles
            0b00 => (self.prev_div & (1 << 9)) != 0 && (self.div & (1 << 9)) == 0,
            // 0b01 (262144 Hz): div bit 3, increment every 4 M-cycles
            0b01 => (self.prev_div & (1 << 3)) != 0 && (self.div & (1 << 3)) == 0,
            // 0b10 (65536 Hz): div bit 5, increment every 16 M-cycles
            0b10 => (self.prev_div & (1 << 5)) != 0 && (self.div & (1 << 5)) == 0,
            // 0b11 (16384 Hz): div bit 7, increment every 64 M-cycles
            0b11 => (self.prev_div & (1 << 7)) != 0 && (self.div & (1 << 7)) == 0,
            _ => false,
        };

        self.prev_div = self.div;

        // If bit 2 of TAC is set to 0 then the timer is disabled
        let is_enabled = self.tac & (1 << 2) != 0;

        // Update TIMA if the timer is enabled and a timer update is triggered
        if timer_update && is_enabled {
            (self.tima, self.tima_overflow) = self.tima.overflowing_add(1);

            if self.tima_overflow {
                // Timer interrupt is delayed 1 cycle (4 clocks) from the TIMA
                // overflow. The TMA reload to TIMA is also delayed. For one cycle,
                // after overflowing TIMA, the value in TIMA is 00h, not TMA.
                self.tima = 0x00;
                self.interrupt_delay = T_CYCLES_PER_M_CYCLE;
            }
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            TIMER_DIV_ADDRESS => self.div = 0,
            TIMER_TIMA_ADDRESS => self.tima = value,
            TIMER_TMA_ADDRESS => self.tma = value,
            TIMER_TAC_ADDRESS => self.tac = value,
            _ => panic!("Invalid Timer address: {:02X}", address),
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            TIMER_DIV_ADDRESS => (self.div >> 8) as u8, // most significant byte in a 16-bit long number
            TIMER_TIMA_ADDRESS => self.tima,
            TIMER_TMA_ADDRESS => self.tma,
            TIMER_TAC_ADDRESS => self.tac | TIMER_TAC_UNUSED_MASK,
            _ => panic!("Invalid Timer address: {:02X}", address),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::auxiliary::timer::Timer;
    use crate::cpu::interrupts::Interrupts;

    #[test]
    pub fn test_timer_tima_01() {
        let mut timer = Timer {
            tac: 0b101,
            div: 0,
            prev_div: 0,
            ..Timer::default()
        };
        let mut interrupts = Interrupts::default();
        let mut prev_tima = 0;
        let cycles = 16;

        for i in 1..=500 {
            timer.tick(&mut interrupts);

            if prev_tima != timer.tima {
                assert_eq!(i % cycles, 0);
            }

            if i == cycles {
                assert_eq!(timer.tima, (cycles / i) as u8);
            }

            prev_tima = timer.tima;
        }
    }

    #[test]
    pub fn test_timer_tima_10() {
        let mut timer = Timer {
            tac: 0b110,
            div: 0,
            prev_div: 0,
            ..Timer::default()
        };
        let mut interrupts = Interrupts::default();
        let mut prev_tima = 0;
        let cycles = 64;

        for i in 1..=1000_usize {
            timer.tick(&mut interrupts);

            if prev_tima != timer.tima {
                assert_eq!(i % cycles, 0);
            }

            if i == cycles {
                assert_eq!(timer.tima, (cycles / i) as u8);
            }

            prev_tima = timer.tima;
        }
    }

    #[test]
    pub fn test_timer_tima_11() {
        let mut timer = Timer {
            tac: 0b111,
            div: 0,
            prev_div: 0,
            ..Timer::default()
        };
        let mut interrupts = Interrupts::default();
        let mut prev_tima = 0;
        let cycles = 256;

        for i in 1..=10000_usize {
            timer.tick(&mut interrupts);

            if prev_tima != timer.tima {
                assert_eq!(i % cycles, 0);
            }

            if i == cycles {
                assert_eq!(timer.tima, (cycles / i) as u8);
            }

            prev_tima = timer.tima;
        }
    }

    #[test]
    pub fn test_timer_tima_00() {
        let mut timer = Timer {
            tac: 0b100,
            div: 0,
            prev_div: 0,
            ..Timer::default()
        };
        let mut interrupts = Interrupts::default();
        let mut prev_tima = 0;
        let cycles = 1024;

        for i in 1..=100000_usize {
            timer.tick(&mut interrupts);

            if prev_tima != timer.tima {
                assert_eq!(i % cycles, 0);
            }

            if i == cycles {
                assert_eq!(timer.tima, (cycles / i) as u8);
            }

            prev_tima = timer.tima;
        }
    }

    #[test]
    pub fn test_timer_tima_00_trigger() {
        let mut timer = Timer {
            tac: 0b100,
            div: 0,
            prev_div: 0,
            ..Timer::default()
        };
        let mut interrupts = Interrupts::default();

        for _ in 1..=512 {
            timer.tick(&mut interrupts);
        }

        timer.div = 0;
        timer.tick(&mut interrupts);

        assert_eq!(1, timer.tima);
    }

    #[test]
    pub fn test_timer_tima_01_trigger() {
        let mut timer = Timer {
            tac: 0b101,
            div: 0,
            prev_div: 0,
            ..Timer::default()
        };
        let mut interrupts = Interrupts::default();

        for _ in 1..=8 {
            timer.tick(&mut interrupts);
        }

        timer.div = 0;
        timer.tick(&mut interrupts);

        assert_eq!(1, timer.tima);
    }

    #[test]
    pub fn test_timer_tima_10_trigger() {
        let mut timer = Timer {
            tac: 0b110,
            div: 0,
            prev_div: 0,
            ..Timer::default()
        };
        let mut interrupts = Interrupts::default();

        for _ in 1..=32 {
            timer.tick(&mut interrupts);
        }

        timer.div = 0;
        timer.tick(&mut interrupts);

        assert_eq!(1, timer.tima);
    }

    #[test]
    pub fn test_timer_tima_11_trigger() {
        let mut timer = Timer {
            tac: 0b111,
            div: 0,
            prev_div: 0,
            ..Timer::default()
        };
        let mut interrupts = Interrupts::default();

        for _ in 1..=128 {
            timer.tick(&mut interrupts);
        }

        timer.div = 0;
        timer.tick(&mut interrupts);

        assert_eq!(1, timer.tima);
    }
}
