use crate::apu::channel::ChannelType;
use crate::apu::length_timer::LengthTimer;
use crate::apu::registers::NRX3_4;
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

pub const CH3_NR32_OUTPUT_LEVEL_MASK: u8 = 0b0110_0000;

pub const CH3_NR30_DAC_ENABLE_POS: u8 = 7;

#[derive(Clone, Debug)]
pub struct WaveChannel {
    dac_enable: NR30,
    // NR31
    length_timer: LengthTimer,
    output_level: NR32,
    // todo: Period changes (written to NR33 or NR34) only take effect after the following time wave RAM is read
    period_and_ctrl: NRX3_4,
    pub wave_ram: WaveRam,

    period_timer: u16, // Internal timer for frequency stepping
    volume_shift: u8,
}

impl Default for WaveChannel {
    fn default() -> Self {
        Self {
            dac_enable: Default::default(),
            length_timer: LengthTimer::new(ChannelType::CH3),
            output_level: Default::default(),
            period_and_ctrl: Default::default(),
            wave_ram: Default::default(),
            period_timer: 0,
            volume_shift: 0,
        }
    }
}

impl WaveChannel {
    pub fn read(&self, address: u16) -> u8 {
        match address {
            CH3_NR30_DAC_ENABLE_ADDRESS => self.dac_enable.read(),
            CH3_NR31_LENGTH_TIMER_ADDRESS => 0xFF, // write-only
            CH3_NR32_OUTPUT_LEVEL_ADDRESS => self.output_level.read(),
            CH3_NR33_PERIOD_LOW_ADDRESS => 0xFF, // write-only
            CH3_NR33_PERIOD_HIGH_CONTROL_ADDRESS => self.period_and_ctrl.high_and_ctrl.read(),
            _ => panic!("Invalid WaveChannel address: {:#X}", address),
        }
    }

    pub fn write(&mut self, address: u16, value: u8, master_ctrl: &mut NR52) {
        match address {
            CH3_NR30_DAC_ENABLE_ADDRESS => {
                self.dac_enable.byte = value;

                if !self.dac_enable.is_dac_enabled() {
                    master_ctrl.deactivate_ch(&ChannelType::CH3);
                }
            }
            CH3_NR31_LENGTH_TIMER_ADDRESS => self.length_timer.set_counter(value),
            CH3_NR32_OUTPUT_LEVEL_ADDRESS => self.output_level.byte = value,
            CH3_NR33_PERIOD_LOW_ADDRESS => self.period_and_ctrl.period_low.set(value),
            CH3_NR33_PERIOD_HIGH_CONTROL_ADDRESS => self.write_period_high(value, master_ctrl),
            _ => panic!("Invalid WaveChannel address: {:#X}", address),
        }
    }
    
    pub fn tick_length(&mut self, master_ctrl: &mut NR52) {
        self.length_timer.tick(master_ctrl, &mut self.period_and_ctrl.high_and_ctrl);
    }

    pub fn tick(&mut self, master_ctrl: &NR52) {
        if self.is_enabled(master_ctrl) {
            if self.period_timer > 0 {
                self.period_timer -= 1;
            }

            if self.period_timer == 0 {
                self.period_timer = (2048 - self.period_and_ctrl.get_period()) * 2;
                self.wave_ram.inc_sample_index();
            }
        }
    }

    pub fn get_output(&self, master_ctrl: &NR52) -> u8 {
        if !self.is_enabled(master_ctrl) {
            return 0;
        }

        self.wave_ram.sample_buffer >> self.volume_shift
    }

    fn trigger(&mut self, master_ctrl: &mut NR52) {
        master_ctrl.activate_ch(&ChannelType::CH3);

        if self.length_timer.is_expired() {
            self.length_timer.reset();
        }

        self.period_timer = self.period_and_ctrl.get_period();
        self.volume_shift = self.output_level.get_volume_shift();
        self.wave_ram.reset();
    }

    fn is_enabled(&self, master_ctrl: &NR52) -> bool {
        self.dac_enable.is_dac_enabled() && master_ctrl.is_ch_active(ChannelType::CH3)
    }

    fn write_period_high(&mut self, value: u8, nr52: &mut NR52) {
        self.period_and_ctrl.high_and_ctrl.write(value);

        if self.period_and_ctrl.high_and_ctrl.is_triggered() {
            self.trigger(nr52);
        }
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

    pub fn read_sample(&self) -> u8 {
        let byte_index = self.sample_index / 2;
        let high_nibble = self.sample_index % 2 == 0;

        if high_nibble {
            self.bytes[byte_index] >> 4
        } else {
            self.bytes[byte_index] & 0x0F
        }
    }

    pub fn inc_sample_index(&mut self) {
        self.sample_index = (self.sample_index + 1) % 32;
        self.sample_buffer = self.read_sample();
    }

    pub fn reset(&mut self) {
        self.sample_index = 0;
    }

    pub fn clear(&mut self) {
        self.sample_buffer = 0;
    }
}

#[derive(Clone, Debug, Default)]
pub struct NR30 {
    byte: u8,
}

impl NR30 {
    pub fn is_dac_enabled(&self) -> bool {
        (self.byte >> CH3_NR30_DAC_ENABLE_POS) != 0
    }

    pub fn read(&self) -> u8 {
        self.byte & 0b1000_0000
    }
}

#[derive(Clone, Debug, Default)]
pub struct NR32 {
    byte: u8,
}

impl NR32 {
    pub fn read(&self) -> u8 {
        self.byte & CH3_NR32_OUTPUT_LEVEL_MASK
    }

    pub fn get_volume_shift(&self) -> u8 {
        match self.read() {
            0b0000_0000 => 4, // mute
            0b0010_0000 => 0, // 100%
            0b0100_0000 => 1, // 50%
            0b0110_0000 => 2, // 25%
            _ => unreachable!(),
        }
    }
}
