use crate::apu::{NR50, NR51};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
    #[inline]
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

        let (left_sample, right_sample) = self.amplify(left_sample, right_sample);

        (
            adjust_volume(left_sample / 4.0),
            adjust_volume(right_sample / 4.0),
        )
    }

    #[inline(always)]
    fn amplify(&self, sample_left: f32, sample_right: f32) -> (f32, f32) {
        let left_sample = apply_volume(sample_left, self.nr50_volume.left_volume());
        let right_sample = apply_volume(sample_right, self.nr50_volume.right_volume());

        (left_sample, right_sample)
    }
}

#[inline(always)]
fn adjust_volume(sample: f32) -> f32 {
    sample
}

#[inline(always)]
fn apply_volume(sample: f32, volume: u8) -> f32 {
    let volume_factor = (volume as f32 + 1.0) / 8.0;
    sample * volume_factor
}
