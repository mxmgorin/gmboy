use crate::apu::channels::channel::ChannelType;
use crate::apu::registers::NRx3x4;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
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

    #[inline(always)]
    pub fn reload(&mut self, nrx3x4: &NRx3x4) {
        self.counter = (2048 - nrx3x4.get_period()) * self.get_multiplier();
    }

    /// Trigger reload with an explicit T-cycle latency instead of the steady
    /// `(2048 - period) * mult` cadence (square-channel trigger delays).
    #[inline(always)]
    pub fn reload_with_delay(&mut self, nrx3x4: &NRx3x4, delay: u16) {
        self.counter = (2047 - nrx3x4.get_period()) * self.get_multiplier() + delay;
    }

    #[inline(always)]
    fn is_expired(&self) -> bool {
        self.counter == 0
    }

    #[inline(always)]
    fn get_multiplier(&self) -> u16 {
        match self.ch_type {
            ChannelType::CH1 | ChannelType::CH2 => 4,
            ChannelType::CH4 => unreachable!("CH4 doesn't have period timer"),
            ChannelType::CH3 => 2,
        }
    }
}
