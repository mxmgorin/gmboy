use crate::apu::{NR50, NR51};

#[derive(Debug, Clone, Default)]
pub struct Mixer {
    pub nr51_panning: NR51,
    pub nr50_volume: NR50,

    pub sample1: f32,
    pub sample2: f32,
    pub sample3: f32,
    pub sample4: f32,
}

impl Mixer {
    /// Combines samples from all channels
    pub fn mix(&self) -> (f32, f32) {
        let mut left_sample = 0.0;
        let mut right_sample = 0.0;

        // Channel 1
        if self.nr51_panning.ch1_left() {
            left_sample += self.sample1;
        }

        if self.nr51_panning.ch1_right() {
            right_sample += self.sample1;
        }

        // Channel 2
        if self.nr51_panning.ch2_left() {
            left_sample += self.sample2;
        }

        if self.nr51_panning.ch2_right() {
            right_sample += self.sample2;
        }

        // Channel 3
        if self.nr51_panning.ch3_left() {
            left_sample += self.sample3;
        }

        if self.nr51_panning.ch3_right() {
            right_sample += self.sample3;
        }

        // Channel 4
        if self.nr51_panning.ch4_left() {
            left_sample += self.sample4;
        }

        if self.nr51_panning.ch4_right() {
            right_sample += self.sample4;
        }

        let left_sample = apply_volume(left_sample, self.nr50_volume.left_volume());
        let right_sample = apply_volume(right_sample, self.nr50_volume.right_volume());

        (left_sample, right_sample)
    }
}

fn apply_volume(sample: f32, volume: u8) -> f32 {
    let volume_factor = volume as f32 / 8.0;
    sample * volume_factor
}
