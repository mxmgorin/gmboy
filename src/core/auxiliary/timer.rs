use crate::auxiliary::clock::T_CYCLES_PER_M_CYCLE;
use crate::cpu::interrupts::{InterruptType, Interrupts};

const DIV_ADDRESS: u16 = 0xFF04;
const TIMA_ADDRESS: u16 = 0xFF05;
const TMA_ADDRESS: u16 = 0xFF06;
const TAC_ADDRESS: u16 = 0xFF07;

#[derive(Debug, Clone)]
pub struct Timer {
    div: u16,
    tima: u8,
    tma: u8,
    tac: u8,
    tima_overflow: bool,
    interrupt_delay: usize,
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

impl Timer {
    pub fn new() -> Self {
        Self {
            div: 0xABBC,
            tima: 0,
            tma: 0,
            tac: 0,
            tima_overflow: false,
            interrupt_delay: 0,
        }
    }

    /// Updates timer if needed and returns is interrupt needed
    pub fn tick(&mut self, interrupts: &mut Interrupts) {
        let prev_div = self.div;
        self.div = self.div.wrapping_add(1);

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

        // detect when bit N transitions from 1 to 0 between the previous DIV and current DIV values
        let timer_update = match self.tac & 0b11 {
            // 0b00 (4096 Hz): div bit 9, increment every 256 M-cycles
            0b00 => (prev_div & (1 << 9)) != 0 && (self.div & (1 << 9)) == 0,
            // 0b01 (262144 Hz): div bit 3, increment every 4 M-cycles
            0b01 => (prev_div & (1 << 3)) != 0 && (self.div & (1 << 3)) == 0,
            // 0b10 (65536 Hz): div bit 5, increment every 16 M-cycles
            0b10 => (prev_div & (1 << 5)) != 0 && (self.div & (1 << 5)) == 0,
            // 0b11 (16384 Hz): div bit 7, increment every 64 M-cycles
            0b11 => (prev_div & (1 << 7)) != 0 && (self.div & (1 << 7)) == 0,
            _ => false,
        };

        // If bit 2 of TAC is set to 0 then the timer is disabled
        let is_enabled = self.tac & (1 << 2) != 0;

        // Update TIMA if the timer is enabled and a timer update is triggered
        if timer_update && is_enabled {
            self.tima = self.tima.wrapping_add(1);
            self.tima_overflow = self.tima == 0xFF;

            if self.tima_overflow {
                // Timer interrupt is delayed 1 cycle (4 clocks) from the TIMA
                // overflow. The TMA reload to TIMA is also delayed. For one cycle,
                // after overflowing TIMA, the value in TIMA is 00h, not TMA.
                self.tima = 0x00;
                self.interrupt_delay = T_CYCLES_PER_M_CYCLE;
            }
        }
    }

    pub fn write(&mut self, address: TimerAddress, value: u8) {
        match address {
            TimerAddress::Div => {
                self.div = 0;
            }
            TimerAddress::Tima => {
                self.tima = value;
            }
            TimerAddress::Tma => {
                self.tma = value;
            }
            TimerAddress::Tac => {
                self.tac = value;
            }
        }
    }

    pub fn read(&self, address: TimerAddress) -> u8 {
        match address {
            TimerAddress::Div => (self.div >> 8) as u8,
            TimerAddress::Tima => self.tima,
            TimerAddress::Tma => self.tma,
            TimerAddress::Tac => self.tac,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TimerAddress {
    Div,
    Tima,
    Tma,
    Tac,
}

impl TryFrom<u16> for TimerAddress {
    type Error = ();

    fn try_from(address: u16) -> Result<Self, Self::Error> {
        match address {
            DIV_ADDRESS => Ok(Self::Div),
            TIMA_ADDRESS => Ok(Self::Tima),
            TMA_ADDRESS => Ok(Self::Tma),
            TAC_ADDRESS => Ok(Self::Tac),
            _ => Err(()),
        }
    }
}

impl TimerAddress {
    pub const fn get_start() -> u16 {
        DIV_ADDRESS
    }

    pub const fn get_end() -> u16 {
        TAC_ADDRESS
    }
}

#[cfg(test)]
mod tests {
    use crate::auxiliary::clock::T_CYCLES_PER_M_CYCLE;
    use crate::auxiliary::timer::Timer;
    use crate::cpu::interrupts::Interrupts;

    #[test]
    pub fn test_timer_tima_inc_256() {
        let mut timer = Timer::new();
        timer.tac = 0b100;
        let mut interrupts = Interrupts::default();

        for _ in 0..(256 * T_CYCLES_PER_M_CYCLE) {
            timer.tick(&mut interrupts)
        }

        assert_eq!(timer.tima, 1);
    }

    #[test]
    pub fn test_timer_tima_inc_4() {
        let mut timer = Timer::new();
        timer.tac = 0b101;
        let mut interrupts = Interrupts::default();

        for _ in 0..T_CYCLES_PER_M_CYCLE {
            timer.tick(&mut interrupts)
        }

        assert_eq!(timer.tima, 1);
    }

    #[test]
    pub fn test_timer_tima_inc_16() {
        let mut timer = Timer::new();
        timer.tac = 0b110;
        let mut interrupts = Interrupts::default();

        for _ in 0..(16 * T_CYCLES_PER_M_CYCLE) {
            timer.tick(&mut interrupts)
        }

        assert_eq!(timer.tima, 1);
    }

    #[test]
    pub fn test_timer_tima_inc_64() {
        let mut timer = Timer::new();
        timer.tac = 0b111;
        let mut interrupts = Interrupts::default();

        for _ in 0..(64 * T_CYCLES_PER_M_CYCLE) {
            timer.tick(&mut interrupts)
        }

        assert_eq!(timer.tima, 1);
    }
}