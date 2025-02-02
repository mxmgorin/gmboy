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

impl Timer {
    pub fn new() -> Self {
        Self {
            div: 0xAC00,
            tima: 0,
            tma: 0,
            tac: 0,
        }
    }

    pub fn tick(&mut self) -> bool {
        let prev_div = self.div;
        self.div += 1;

        let mut timer_update = false;

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

        if timer_update && (self.tac & (1 << 2)) != 0 {
            self.tima += 1;

            if self.tima == 0xFF {
                self.tima = self.tma;
                
                return true;
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
