use crate::apu::channel::ChannelType;
use crate::apu::length_timer::LengthTimer;
use crate::apu::registers::NRX3X4;
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

#[derive(Clone, Debug)]
pub struct WaveChannel {
    nr30_dac_enable: NR30,
    // NR31
    nr31_length_timer: LengthTimer,
    rn32_output_level: NR32,
    // todo: Period changes (written to NR33 or NR34) only take effect after the following time wave RAM is read
    nr33_34_period_and_ctrl: NRX3X4,
    pub wave_ram: WaveRam,

    period_timer: u16, // Internal timer for frequency stepping
    volume_shift: u8,
}

impl Default for WaveChannel {
    fn default() -> Self {
        Self {
            nr30_dac_enable: Default::default(),
            nr31_length_timer: LengthTimer::new(ChannelType::CH3),
            rn32_output_level: Default::default(),
            nr33_34_period_and_ctrl: Default::default(),
            wave_ram: Default::default(),
            period_timer: 0,
            volume_shift: 0,
        }
    }
}

impl WaveChannel {
    pub fn read(&self, address: u16) -> u8 {
        match address {
            CH3_NR30_DAC_ENABLE_ADDRESS => self.nr30_dac_enable.read(),
            CH3_NR31_LENGTH_TIMER_ADDRESS => 0xFF, // write-only
            CH3_NR32_OUTPUT_LEVEL_ADDRESS => self.rn32_output_level.read(),
            CH3_NR33_PERIOD_LOW_ADDRESS => 0xFF, // write-only
            CH3_NR33_PERIOD_HIGH_CONTROL_ADDRESS => {
                self.nr33_34_period_and_ctrl.high_and_ctrl.read()
            }
            _ => panic!("Invalid WaveChannel address: {:#X}", address),
        }
    }

    pub fn write(&mut self, address: u16, value: u8, master_ctrl: &mut NR52) {
        match address {
            CH3_NR30_DAC_ENABLE_ADDRESS => self.nr30_dac_enable.byte = value,
            CH3_NR31_LENGTH_TIMER_ADDRESS => self.nr31_length_timer.write(value),
            CH3_NR32_OUTPUT_LEVEL_ADDRESS => self.rn32_output_level.byte = value,
            CH3_NR33_PERIOD_LOW_ADDRESS => self.nr33_34_period_and_ctrl.period_low.write(value),
            CH3_NR33_PERIOD_HIGH_CONTROL_ADDRESS => self.write_period_high(value, master_ctrl),
            _ => panic!("Invalid WaveChannel address: {:#X}", address),
        }
    }

    pub fn tick_length(&mut self, master_ctrl: &mut NR52) {
        self.nr31_length_timer
            .tick(master_ctrl, &mut self.nr33_34_period_and_ctrl.high_and_ctrl);
    }

    pub fn tick(&mut self, master_ctrl: &NR52) {
        if master_ctrl.is_ch_active(&self.nr31_length_timer.ch_type)
            && self.nr30_dac_enable.is_dac_enabled()
        {
            if self.period_timer > 0 {
                self.period_timer -= 1;
            }

            if self.period_timer == 0 {
                self.period_timer = (2048 - self.nr33_34_period_and_ctrl.get_period()) * 2;
                self.wave_ram.inc_sample_index(); // generate sample
            }
        }
    }

    pub fn get_output(&self, master_ctrl: &NR52) -> u8 {
        if master_ctrl.is_ch_active(&self.nr31_length_timer.ch_type)
            && self.nr30_dac_enable.is_dac_enabled()
        {
            return self.wave_ram.sample_buffer >> self.volume_shift;
        }

        0
    }

    fn trigger(&mut self, master_ctrl: &mut NR52) {
        master_ctrl.activate_ch(&self.nr31_length_timer.ch_type);

        if self.nr31_length_timer.is_expired() {
            self.nr31_length_timer.reset();
        }

        //self.period_timer = self.period_and_ctrl.get_period();
        self.volume_shift = self.rn32_output_level.get_volume_shift();
        self.wave_ram.reset_sample_index();
    }

    fn write_period_high(&mut self, value: u8, nr52: &mut NR52) {
        self.nr33_34_period_and_ctrl.high_and_ctrl.write(value);

        if self.nr33_34_period_and_ctrl.high_and_ctrl.is_triggered() {
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
#[derive(Clone, Debug, Default)]
pub struct NR30 {
    byte: u8,
}

impl NR30 {
    pub fn is_dac_enabled(&self) -> bool {
        (self.byte >> CH3_NR30_DAC_ENABLE_POS) != 0
    }

    pub fn read(&self) -> u8 {
        self.byte | 1 << CH3_NR30_DAC_ENABLE_POS
    }
}

/// Output level
#[derive(Clone, Debug, Default)]
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
