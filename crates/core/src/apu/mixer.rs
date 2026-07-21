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
    /// Combines samples from all channels. `channel_mask` is the user-facing
    /// mute (bit N audible = channel N+1); it gates the hardware panning
    /// selection without touching any observable APU state.
    #[inline]
    pub fn mix(&self, channel_mask: u8) -> (f32, f32) {
        let pan = NR51 {
            byte: self.nr51_panning.byte & (channel_mask << 4 | channel_mask),
        };
        let mut left_sample = 0.0;
        let mut right_sample = 0.0;

        // Channel 1
        if pan.ch1_left() {
            left_sample += self.sample1;
        }

        if pan.ch1_right() {
            right_sample += self.sample1;
        }

        // Channel 2
        if pan.ch2_left() {
            left_sample += self.sample2;
        }

        if pan.ch2_right() {
            right_sample += self.sample2;
        }

        // Channel 3
        if pan.ch3_left() {
            left_sample += self.sample3;
        }

        if pan.ch3_right() {
            right_sample += self.sample3;
        }

        // Channel 4
        if pan.ch4_left() {
            left_sample += self.sample4;
        }

        if pan.ch4_right() {
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
