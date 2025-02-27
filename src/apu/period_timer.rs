use crate::apu::channel::ChannelType;
use crate::apu::registers::NRx3x4;

#[derive(Clone, Debug)]
pub struct PeriodTimer {
    counter: u16,
    ch_type: ChannelType,
}

impl PeriodTimer {
    pub fn new(ch_type: ChannelType) -> Self {
        Self {
            counter: 0,
            ch_type,
        }
    }

    /// Ticks internal counter when enable conditions are met and returns true if expired
    pub fn tick(&mut self, nrx3x4: &NRx3x4) -> bool {
        if !self.is_expired() {
            self.counter -= 1;
        }

        if self.is_expired() {
            self.reload(nrx3x4);
            return true;
        }

        false
    }

    pub fn reload(&mut self, nrx3x4: &NRx3x4) {
        self.counter = (2048 - nrx3x4.get_period()) * self.get_multiplier();
    }

    fn is_expired(&self) -> bool {
        self.counter == 0
    }

    fn get_multiplier(&self) -> u16 {
        match self.ch_type {
            ChannelType::CH1 | ChannelType::CH2 => 4,
            ChannelType::CH4 => unreachable!("CH4 doesn't have period timer"),
            ChannelType::CH3 => 2,
        }
    }
}
