use crate::apu::{NR50, NR51};

#[derive(Debug, Clone, Default)]
pub struct Mixer {
    pub nr51_sound_panning: NR51,
    pub nr50_master_volume: NR50,

    pub outputs: [f32; 4],
}

impl Mixer {
    /// Combines outputs from all channels
    pub fn mix(&self) -> (f32, f32) {
        let mut left_output = 0.0;
        let mut right_output = 0.0;

        // Channel 1
        if self.nr51_sound_panning.ch1_left() {
            left_output += self.outputs[0];
        }

        if self.nr51_sound_panning.ch1_right() {
            right_output += self.outputs[0];
        }

        // Channel 2
        if self.nr51_sound_panning.ch2_left() {
            left_output += self.outputs[1];
        }

        if self.nr51_sound_panning.ch2_right() {
            right_output += self.outputs[1];
        }

        // Channel 3
        if self.nr51_sound_panning.ch3_left() {
            left_output += self.outputs[2];
        }

        if self.nr51_sound_panning.ch3_right() {
            right_output += self.outputs[2];
        }

        // Channel 4
        if self.nr51_sound_panning.ch4_left() {
            left_output += self.outputs[3];
        }

        if self.nr51_sound_panning.ch4_right() {
            right_output += self.outputs[3];
        }

        let left_output = apply_volume(left_output, self.nr50_master_volume.left_volume());
        let right_output = apply_volume(right_output, self.nr50_master_volume.right_volume());

        (left_output, right_output)
    }
}

fn apply_volume(sample: f32, volume: u8) -> f32 {
    let volume_factor = (volume as f32 + 1.0) / 8.0;
    sample * volume_factor
}
