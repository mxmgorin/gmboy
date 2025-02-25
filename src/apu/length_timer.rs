use crate::apu::channel::ChannelType;
use crate::apu::registers::NRX4;
use crate::apu::NR52;

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
#[derive(Clone, Debug)]
pub struct LengthTimer {
    counter: u16, // write-only
    ch_type: ChannelType,
}

impl LengthTimer {    
    pub fn new(ch_type: ChannelType) -> Self {
        Self {
            counter: 0,
            ch_type,
        }
    }
    
    pub fn tick(&mut self, nr52: &mut NR52, nrx4: &mut NRX4) {
        if nrx4.is_length_enabled() && self.counter != 0 {
            self.counter += 1;
        }

        if self.is_expired() {
            nr52.deactivate_ch(&self.ch_type);
            self.reset();
        }
    }

    pub fn is_expired(&self) -> bool {
        self.counter >= self.ch_type.get_length_timer_expire()
    }

    pub fn set_counter(&mut self, counter: u8) {
        self.counter = (counter & self.ch_type.get_length_timer_mask()) as u16;
    }

    pub fn reset(&mut self) {
        self.counter = 0;
    }
}
