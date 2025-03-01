use crate::apu::channels::channel::ChannelType;
use crate::apu::dac::{DacEnable, DigitalSampleProducer};
use crate::apu::registers::{NRx1, NRx3x4};
use crate::apu::timers::length_timer::LengthTimer;
use crate::apu::timers::period_timer::PeriodTimer;
use crate::apu::NR52;

pub const CH3_START_ADDRESS: u16 = CH3_NR30_DAC_ENABLE_ADDRESS;
pub const CH3_END_ADDRESS: u16 = CH3_NR33_PERIOD_HIGH_CONTROL_ADDRESS;

pub const CH3_NR30_DAC_ENABLE_ADDRESS: u16 = 0xFF1A;
pub const CH3_NR31_LENGTH_TIMER_ADDRESS: u16 = 0xFF1B;
pub const CH3_NR32_OUTPUT_LEVEL_ADDRESS: u16 = 0xFF1C;
pub const CH3_NR33_PERIOD_LOW_ADDRESS: u16 = 0xFF1D;
pub const CH3_NR33_PERIOD_HIGH_CONTROL_ADDRESS: u16 = 0xFF1E;

pub const CH3_WAVE_RAM_START: u16 = 0xFF30;
pub const CH3_WAVE_RAM_END: u16 = 0xFF3F;
pub const CH3_NR30_DAC_ENABLE_POS: u8 = 7;

impl DacEnable for WaveChannel {
    fn is_dac_enabled(&self) -> bool {
        self.nrx0_dac_enable.is_dac_enabled()
    }
}

impl DigitalSampleProducer for WaveChannel {
    fn get_sample(&self, nr52: NR52) -> u8 {
        if nr52.is_ch3_on() {
            let output = self.wave_ram.sample_buffer >> self.volume_shift;

            return output;
        }

        0
    }
}

#[derive(Clone, Debug)]
pub struct WaveChannel {
    // registers
    nrx0_dac_enable: NR30,
    nrx1_length_timer: NRx1,
    nrx2_output_level: NR32,
    nrx3x4_period_and_ctrl: NRx3x4,
    pub wave_ram: WaveRam,

    // other data
    // todo: Period changes (written to NR33 or NR34) only take effect after the following time wave RAM is read
    period_timer: PeriodTimer,
    length_timer: LengthTimer,
    volume_shift: u8,
}

impl Default for WaveChannel {
    fn default() -> Self {
        Self {
            nrx0_dac_enable: Default::default(),
            nrx1_length_timer: NRx1::new(ChannelType::CH3),
            nrx2_output_level: Default::default(),
            nrx3x4_period_and_ctrl: Default::default(),
            wave_ram: Default::default(),
            length_timer: LengthTimer::new(ChannelType::CH3),
            period_timer: PeriodTimer::new(ChannelType::CH3),
            volume_shift: 0,
        }
    }
}

impl WaveChannel {
    pub fn read(&self, address: u16) -> u8 {
        match address {
            CH3_NR30_DAC_ENABLE_ADDRESS => self.nrx0_dac_enable.read(),
            CH3_NR31_LENGTH_TIMER_ADDRESS => 0xFF, // write-only
            CH3_NR32_OUTPUT_LEVEL_ADDRESS => self.nrx2_output_level.read(),
            CH3_NR33_PERIOD_LOW_ADDRESS => 0xFF, // write-only
            CH3_NR33_PERIOD_HIGH_CONTROL_ADDRESS => self.nrx3x4_period_and_ctrl.nrx4.read(),
            _ => panic!("Invalid WaveChannel address: {:#X}", address),
        }
    }

    pub fn write(&mut self, address: u16, value: u8, nr52_master_ctrl: &mut NR52) {
        match address {
            CH3_NR30_DAC_ENABLE_ADDRESS => self.nrx0_dac_enable.byte = value,
            CH3_NR31_LENGTH_TIMER_ADDRESS => {
                self.nrx1_length_timer.byte = value;
                self.length_timer.reload(self.nrx1_length_timer); // research: do it must be reloaded after write?
            }
            CH3_NR32_OUTPUT_LEVEL_ADDRESS => self.nrx2_output_level.byte = value,
            CH3_NR33_PERIOD_LOW_ADDRESS => self.nrx3x4_period_and_ctrl.period_low.write(value),
            CH3_NR33_PERIOD_HIGH_CONTROL_ADDRESS => {
                self.nrx3x4_period_and_ctrl.nrx4.write(value);

                if self.nrx3x4_period_and_ctrl.nrx4.is_triggered() {
                    self.trigger(nr52_master_ctrl);
                }
            }
            _ => panic!("Invalid WaveChannel address: {:#X}", address),
        }
    }

    pub fn tick_length(&mut self, master_ctrl: &mut NR52) {
        self.length_timer
            .tick(master_ctrl, &mut self.nrx3x4_period_and_ctrl.nrx4);
    }

    pub fn tick(&mut self) {
        if self.period_timer.tick(&self.nrx3x4_period_and_ctrl) {
            self.wave_ram.inc_sample_index();
        }
    }

    fn trigger(&mut self, master_ctrl: &mut NR52) {
        master_ctrl.activate_ch3();

        if self.length_timer.is_expired() {
            self.length_timer.reload(self.nrx1_length_timer);
        }

        self.period_timer.reload(&self.nrx3x4_period_and_ctrl);
        self.volume_shift = self.nrx2_output_level.get_volume_shift();
        self.wave_ram.reset_sample_index();
    }
}

#[derive(Clone, Debug, Default)]
pub struct WaveRam {
    // 32 samples, 4 bit each
    bytes: [u8; 16],
    sample_index: usize,
    sample_buffer: u8,
}

impl WaveRam {
    pub fn read(&self, addr: u16) -> u8 {
        let addr = addr - CH3_WAVE_RAM_START;
        self.bytes[addr as usize]
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        let index = addr - CH3_WAVE_RAM_START;
        self.bytes[index as usize] = value;
    }

    fn read_sample(&self) -> u8 {
        let byte_index = self.sample_index / 2;
        let is_high_nibble = self.sample_index % 2 == 0;

        if is_high_nibble {
            self.bytes[byte_index] >> 4
        } else {
            self.bytes[byte_index] & 0x0F
        }
    }

    pub fn inc_sample_index(&mut self) {
        self.sample_index = (self.sample_index + 1) % 32;
        self.sample_buffer = self.read_sample();
    }

    pub fn reset_sample_index(&mut self) {
        self.sample_index = 0;
    }

    pub fn clear_sample_buffer(&mut self) {
        self.sample_index = 0;
        self.sample_buffer = 0;
    }
}

// DAC enable
#[derive(Clone, Debug, Default, Copy)]
pub struct NR30 {
    byte: u8,
}

impl NR30 {
    pub fn is_dac_enabled(&self) -> bool {
        (self.byte >> CH3_NR30_DAC_ENABLE_POS) != 0
    }

    pub fn read(&self) -> u8 {
        self.byte | (1 << CH3_NR30_DAC_ENABLE_POS)
    }
}

/// Output level
#[derive(Clone, Debug, Default, Copy)]
pub struct NR32 {
    byte: u8,
}

impl NR32 {
    pub fn read(&self) -> u8 {
        self.byte | 0b1001_1111
    }

    pub fn get_volume_shift(&self) -> u8 {
        match self.byte & 0b0110_0000 {
            0b0000_0000 => 4, // mute
            0b0010_0000 => 0, // 100%
            0b0100_0000 => 1, // 50%
            0b0110_0000 => 2, // 25%
            _ => unreachable!(),
        }
    }
}
