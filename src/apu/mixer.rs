use crate::apu::{NR50, NR51};

#[derive(Debug, Clone, Default)]
pub struct Mixer {
    pub nr51_sound_panning: NR51,
    pub nr50_master_volume: NR50,
}

impl Mixer {
    /// Combines outputs from all channels
    pub fn mix(&self, outputs: [u8; 4]) -> (u8, u8) {
        let mut left_output = 0;
        let mut right_output = 0;

        // Channel 1
        if self.nr51_sound_panning.ch3_left() {
            left_output += outputs[0];
        }

        if self.nr51_sound_panning.ch3_right() {
            right_output += outputs[0];
        }

        // Channel 2
        if self.nr51_sound_panning.ch3_left() {
            left_output += outputs[1];
        }

        if self.nr51_sound_panning.ch3_right() {
            right_output += outputs[1];
        }

        // Channel 3
        if self.nr51_sound_panning.ch3_left() {
            left_output += outputs[2];
        }

        if self.nr51_sound_panning.ch3_right() {
            right_output += outputs[2];
        }

        let left_volume = self.nr50_master_volume.left_volume();
        let right_volume = self.nr50_master_volume.right_volume();

        // Apply NR50 scaling
        (left_output * left_volume, right_output * right_volume)
    }
}
