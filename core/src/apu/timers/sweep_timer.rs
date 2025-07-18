use serde::{Deserialize, Serialize};
use crate::apu::channels::channel::ChannelType;
use crate::apu::channels::square_channel::NR10;
use crate::apu::registers::NRx3x4;
use crate::apu::NR52;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct SweepTimer {
    counter: u8,
    shadow_frequency: u16,
    enabled: bool,
    pub nr10: NR10,
}

impl SweepTimer {
    pub fn tick(&mut self, nr52: &mut NR52, nrx3x4: &mut NRx3x4) {
        if self.counter > 0 {
            self.counter -= 1;
        }

        if self.counter == 0 {
            let pace = self.reload_counter();

            if self.enabled && pace != 0 {
                let new_frequency = self.calc_frequency(nr52);

                if new_frequency <= 2047 && self.nr10.individual_step() > 0 {
                    nrx3x4.set_period(new_frequency);
                    self.shadow_frequency = new_frequency;

                    /* for overflow check */
                    self.calc_frequency(nr52);
                }
            }
        }
    }

    pub fn reload(&mut self, nr52: &mut NR52, nrx3x4: NRx3x4) {
        let pace = self.reload_counter();
        let individual_step_non_zero = self.nr10.individual_step() != 0;

        self.enabled = individual_step_non_zero || pace != 0;
        self.shadow_frequency = nrx3x4.get_period();

        if individual_step_non_zero {
            /* for overflow check */
            self.calc_frequency(nr52);
        }
    }

    fn reload_counter(&mut self) -> u8 {
        let pace = self.nr10.pace();

        if pace == 0 {
            self.counter = 8;
        } else {
            self.counter = pace;
        }

        pace
    }

    fn calc_frequency(&mut self, nr52: &mut NR52) -> u16 {
        let mut new_frequency = self.shadow_frequency >> self.nr10.individual_step();

        if self.nr10.direction_down() {
            new_frequency = self.shadow_frequency - new_frequency
        } else {
            new_frequency += self.shadow_frequency
        }

        /* overflow check */
        if new_frequency > 2047 {
            nr52.deactivate_ch(ChannelType::CH1);
        }

        new_frequency
    }
}
