use crate::apu::registers::NRX4;

pub const CH3_START_ADDRESS: u16 = NR30_CH3_DAC_ENABLE_ADDRESS;
pub const CH3_END_ADDRESS: u16 = NR33_CH3_PERIOD_LOW_ADDRESS;

pub const NR30_CH3_DAC_ENABLE_ADDRESS: u16 = 0xFF1A;
pub const NR31_CH3_LENGTH_TIMER_ADDRESS: u16 = 0xFF1B;
pub const NR32_CH3_OUTPUT_LEVEL_ADDRESS: u16 = 0xFF1C;
pub const NR33_CH3_PERIOD_LOW_ADDRESS: u16 = 0xFF1D;
pub const CH3_WAVE_RAM_START: u16 = 0xFF30;
pub const CH3_WAVE_RAM_END: u16 = 0xFF3F;

pub const NR32_CH3_OUTPUT_LEVEL_MASK: u8 = 0b0110_0000;

pub struct WaveChannel {
    pub dac_enabled: bool,
    pub length_timer: u8,
    pub output_level: u8,
    // Period changes (written to NR33 or NR34) only take effect after the following time wave RAM is read
    pub period_low: u8,
    pub period_high_control: NRX4,
    pub wave_ram: [u8; 16],
}
