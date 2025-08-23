use crate::apu::registers::NRx2;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EnvelopeTimer {
    counter: u8,
    volume: u8,
}

impl EnvelopeTimer {
    pub fn tick(&mut self, nrx2: NRx2) {
        if self.counter == 0 {
            return;
        }

        self.counter -= 1;

        if self.counter == 0 {
            self.counter = nrx2.sweep_pace();
            let up_dir = nrx2.envelope_dir_up();

            if self.volume < 0xF && up_dir {
                self.volume += 1;
            } else if self.volume > 0x0 && !up_dir {
                self.volume -= 1;
            }
        }
    }

    #[inline(always)]
    pub fn get_volume(&self) -> u8 {
        self.volume
    }

    #[inline(always)]
    pub fn reload(&mut self, nrx2: NRx2) {
        self.volume = nrx2.initial_volume();
        self.counter = nrx2.sweep_pace();
    }
}
