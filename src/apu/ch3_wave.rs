use crate::apu::length_timer::LengthTimer;
use crate::apu::registers::NRX4;
use crate::LittleEndianBytes;

pub const CH3_START_ADDRESS: u16 = CH3_NR30_DAC_ENABLE_ADDRESS;
pub const CH3_END_ADDRESS: u16 = CH3_NR33_PERIOD_LOW_ADDRESS;

pub const CH3_NR30_DAC_ENABLE_ADDRESS: u16 = 0xFF1A;
pub const CH3_NR31_LENGTH_TIMER_ADDRESS: u16 = 0xFF1B;
pub const CH3_NR32_OUTPUT_LEVEL_ADDRESS: u16 = 0xFF1C;
pub const CH3_NR33_PERIOD_LOW_ADDRESS: u16 = 0xFF1D;
pub const CH3_NR33_PERIOD_HIGH_CONTROL_ADDRESS: u16 = 0xFF1E;

pub const CH3_WAVE_RAM_START: u16 = 0xFF30;
pub const CH3_WAVE_RAM_END: u16 = 0xFF3F;

pub const NR32_CH3_OUTPUT_LEVEL_MASK: u8 = 0b0110_0000;

#[derive(Default)]
pub struct WaveRam {
    bytes: [u8; 16],
}

impl WaveRam {
    pub fn read(&self, addr: u16) -> u8 {
        let addr = addr - CH3_WAVE_RAM_START;
        self.bytes[addr as usize]
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        let addr = addr - CH3_WAVE_RAM_START;
        self.bytes[addr as usize] = value;
    }
}

pub struct WaveChannel {
    pub dac_enabled: bool,
    length_timer: LengthTimer,
    output_level: u8,
    // Period changes (written to NR33 or NR34) only take effect after the following time wave RAM is read
    period_low: u8,
    period_high_control: NRX4,
    pub wave_ram: WaveRam,
}

impl WaveChannel {
    pub fn read(&self, address: u16) -> u8 {
        match address {
            CH3_NR30_DAC_ENABLE_ADDRESS => (self.dac_enabled as u8) << 7,
            CH3_NR31_LENGTH_TIMER_ADDRESS => 0, // write-only
            CH3_NR32_OUTPUT_LEVEL_ADDRESS => self.output_level & NR32_CH3_OUTPUT_LEVEL_MASK,
            CH3_NR33_PERIOD_LOW_ADDRESS => 0, // write-only
            CH3_NR33_PERIOD_HIGH_CONTROL_ADDRESS => self.period_high_control.read(),
            _ => panic!("Invalid WaveChannel address: {:#X}", address),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {}

    pub fn get_output_level(&self) -> OutputLevel {
        match self.output_level & NR32_CH3_OUTPUT_LEVEL_MASK {
            // 0b00 (4096 Hz): div bit 9, increment every 256 M-cycles
            0b0000_0000 => OutputLevel::Mute,
            // 0b01 (262144 Hz): div bit 3, increment every 4 M-cycles
            0b0010_0000 => OutputLevel::FullVolume,
            // 0b10 (65536 Hz): div bit 5, increment every 16 M-cycles
            0b0100_0000 => OutputLevel::HalfVolume,
            // 0b11 (16384 Hz): div bit 7, increment every 64 M-cycles
            0b0110_0000 => OutputLevel::QuarterVolume,
            _ => unreachable!(),
        }
    }

    pub fn get_period(&self) -> u16 {
        let value = LittleEndianBytes {
            low_byte: self.period_low,
            high_byte: self.period_high_control.period(),
        };

        value.into()
    }
}

pub enum OutputLevel {
    Mute,
    FullVolume,
    HalfVolume,
    QuarterVolume,
}
