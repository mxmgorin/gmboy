use crate::apu::channels::noise_channel::CH4_START_ADDRESS;
use crate::apu::channels::square_channel::{CH1_START_ADDRESS, CH2_START_ADDRESS};
use crate::apu::channels::wave_channel::CH3_START_ADDRESS;
use serde::{Deserialize, Serialize};

// Square 1: Sweep -> Timer -> Duty -> Length Counter -> Envelope -> Mixer
// Square 2:          Timer -> Duty -> Length Counter -> Envelope -> Mixer
// Wave:              Timer -> Wave -> Length Counter -> Volume   -> Mixer
// Noise:             Timer -> LFSR -> Length Counter -> Envelope -> Mixer
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ChannelType {
    CH1,
    CH2,
    CH3,
    CH4,
}

impl ChannelType {
    #[inline]
    pub fn get_start_address(&self) -> u16 {
        match self {
            ChannelType::CH1 => CH1_START_ADDRESS,
            ChannelType::CH2 => CH2_START_ADDRESS,
            ChannelType::CH3 => CH3_START_ADDRESS,
            ChannelType::CH4 => CH4_START_ADDRESS,
        }
    }
}
