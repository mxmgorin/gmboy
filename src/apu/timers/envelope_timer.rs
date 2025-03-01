use crate::registers::NRx2;

#[derive(Clone, Debug, Default)]
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

    pub fn get_volume(&self) -> u8 {
        self.volume
    }

    pub fn reload(&mut self, nrx2: NRx2) {
        self.volume = nrx2.initial_volume();
        self.counter = nrx2.sweep_pace();
    }
}
