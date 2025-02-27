use crate::apu::registers::{NRx1, NRx2, NRx4};
use crate::get_bit_flag;

pub const CH4_START_ADDRESS: u16 = NR41_CH4_LENGTH_TIMER_ADDRESS;
pub const CH4_END_ADDRESS: u16 = NR44_CH4_CONTROL_ADDRESS;

pub const NR41_CH4_LENGTH_TIMER_ADDRESS: u16 = 0xFF20;
pub const NR42_CH4_VOLUME_ENVELOPE_ADDRESS: u16 = 0xFF21;
pub const NR43_CH4_FREQUENCY_RANDOMNESS_ADDRESS: u16 = 0xFF22;
pub const NR44_CH4_CONTROL_ADDRESS: u16 = 0xFF23;

#[derive(Debug, Clone, Default)]
pub struct NoiseChannel {
    pub length_timer: NRx1,
    pub volume_envelope: NRx2,
    pub frequency_randomness: NR43,
    pub control: NRx4,
}

/// FF22 — NR43: Channel 4 frequency & randomness
/// This register allows controlling the way the amplitude is randomly switched.
#[derive(Debug, Clone, Default)]
pub struct NR43 {
    pub byte: u8,
}

impl NR43 {
    pub fn clock_shift(&self) -> u8 {
        self.byte & 0b1111_0000
    }

    pub fn lfsr_width(&self) -> bool {
        get_bit_flag(self.byte, 3)
    }

    pub fn clock_divider(&self) -> u8 {
        self.byte & 0b0000_0111
    }
}
