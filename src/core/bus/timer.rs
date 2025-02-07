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
        }
    }

    /// Updates timer if needed and returns is interrupt needed
    pub fn tick(&mut self) -> bool {
        let prev_div = self.div;
        self.div = self.div.wrapping_add(1);

        let mut timer_update = false;

        // 0b00 (4096 Hz): div bit 9
        // 0b01 (262144 Hz): div bit 3
        // 0b10 (65536 Hz): div bit 5
        // 0b11 (16384 Hz): div bit 7
        // detect when bit N transitions from 1 to 0 between the previous DIV and current DIV values
        match self.tac & 0b11 {
            0b00 => {
                timer_update = (prev_div & (1 << 9)) != 0 && (self.div & (1 << 9)) == 0;
            }
            0b01 => {
                timer_update = (prev_div & (1 << 3)) != 0 && (self.div & (1 << 3)) == 0;
            }
            0b10 => {
                timer_update = (prev_div & (1 << 5)) != 0 && (self.div & (1 << 5)) == 0;
            }
            0b11 => {
                timer_update = (prev_div & (1 << 7)) != 0 && (self.div & (1 << 7)) == 0;
            }
            _ => {}
        }

        // Update TIMA if the timer is enabled and a timer update is triggered
        if timer_update && (self.tac & (1 << 2)) != 0 {
            self.tima += 1;

            let is_overflow = self.tima == 0xFF;
            if is_overflow {
                self.tima = self.tma;
                return true; // Signal an interrupt
            }
        }

        false
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
    use super::*;

    #[test]
    fn test_div_increments_correctly() {
        let mut timer = Timer::new();
        let initial_div = timer.div;

        timer.tick(); // Simulate one clock cycle
        assert_eq!(timer.div, initial_div.wrapping_add(1));
    }

    #[test]
    fn test_tima_increments_at_correct_frequency() {
        let mut timer = Timer::new();

        // Set TAC to use bit 9 of DIV (4096 Hz)
        timer.tac = 0b100; // Timer enabled, clock select 0b00

        // Simulate enough ticks to trigger an increment
        for _ in 0..512 {
            // 2^9 = 512 ticks for a full cycle
            timer.tick();
        }

        assert_eq!(timer.tima, 1); // TIMA should have incremented once
    }

    #[test]
    fn test_tima_overflow() {
        let mut timer = Timer::new();
        timer.tac = 0b100; // Timer enabled
        timer.tima = 0xFE; // Set TIMA near overflow
        timer.tma = 0x42; // Set TMA (reload value)

        // Simulate enough ticks to cause an overflow
        for _ in 0..512 {
            timer.tick();
        }

        assert_eq!(timer.tima, 0x42); // TIMA should reload from TMA
                                      // Check for interrupt flag (if implemented)
                                      // assert_eq!(timer.interrupt_flag, 0x04);
    }

    #[test]
    fn test_timer_disabled() {
        let mut timer = Timer::new();
        timer.tac = 0b000; // Timer disabled

        for _ in 0..1024 {
            timer.tick();
        }

        assert_eq!(timer.tima, 0); // TIMA should not increment
    }
}
