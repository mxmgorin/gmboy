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
    pub fn tick(&mut self, interrupts: &mut Interrupts) {
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
            self.tima = self.tima.wrapping_add(1);

            let is_overflow = self.tima == 0xFF;
            if is_overflow {
                self.tima = self.tma;
                interrupts.request_interrupt(InterruptType::Timer);
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
