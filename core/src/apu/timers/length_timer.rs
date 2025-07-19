use crate::apu::channels::channel::ChannelType;
use crate::apu::registers::{NRx1, NRx4};
use crate::apu::NR52;
use serde::{Deserialize, Serialize};

//A length counter disables a channel when it decrements to zero. It contains an internal counter
// and enabled flag. Writing a byte to NRx1 loads the counter with 64-data (256-data for wave channel).
// The counter can be reloaded at any time.
//
// A channel is said to be disabled when the internal enabled flag is clear. When a channel is disabled,
// its volume unit receives 0, otherwise its volume unit receives the output of the waveform generator.
// Other units besides the length counter can enable/disable the channel as well.
//
// Each length counter is clocked at 256 Hz by the frame sequencer. When clocked while enabled by NRx4
// and the counter is not zero, it is decremented. If it becomes zero, the channel is disabled.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LengthTimer {
    counter: u16,
    ch_type: ChannelType,
}

impl LengthTimer {
    pub fn new(ch_type: ChannelType) -> Self {
        Self {
            counter: 0,
            ch_type,
        }
    }

    pub fn tick(&mut self, nr52: &mut NR52, nrx4: &mut NRx4) {
        if !nrx4.is_length_enabled() || self.is_expired() {
            return;
        }

        self.counter = self.counter.saturating_sub(1);

        if self.is_expired() {
            nr52.deactivate_ch(self.ch_type);
        }
    }

    pub fn is_expired(&self) -> bool {
        self.counter == 0
    }

    pub fn reload(&mut self, nrx1: NRx1) {
        self.counter = self.get_initial_length() - nrx1.initial_length_timer() as u16;
    }

    pub fn reset(&mut self) {
        self.counter = self.get_initial_length();
    }

    pub fn get_initial_length(&self) -> u16 {
        match self.ch_type {
            ChannelType::CH1 | ChannelType::CH2 | ChannelType::CH4 => 64,
            ChannelType::CH3 => 256,
        }
    }
}
