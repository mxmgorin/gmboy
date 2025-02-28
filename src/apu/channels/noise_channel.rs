use crate::apu::registers::{NRx2, NRx4};
use crate::channels::channel::ChannelType;
use crate::registers::NRx1;
use crate::timers::length_timer::LengthTimer;
use crate::{get_bit_flag, NR52};
use crate::timers::envelope_timer::EnvelopeTimer;

pub const CH4_START_ADDRESS: u16 = NR41_CH4_LENGTH_TIMER_ADDRESS;
pub const CH4_END_ADDRESS: u16 = NR44_CH4_CONTROL_ADDRESS;

pub const NR41_CH4_LENGTH_TIMER_ADDRESS: u16 = 0xFF20;
pub const NR42_CH4_VOLUME_ENVELOPE_ADDRESS: u16 = 0xFF21;
pub const NR43_CH4_FREQUENCY_RANDOMNESS_ADDRESS: u16 = 0xFF22;
pub const NR44_CH4_CONTROL_ADDRESS: u16 = 0xFF23;

#[derive(Debug, Clone)]
pub struct NoiseChannel {
    nrx1_len: NRx1,
    nrx2_envelope_and_dac: NRx2,
    nr43_freq_and_rnd: NR43,
    nrx4_ctrl: NRx4,

    length_timer: LengthTimer,
    envelope_timer: EnvelopeTimer,
}

impl Default for NoiseChannel {
    fn default() -> NoiseChannel {
        Self {
            nrx1_len: NRx1::new(ChannelType::CH4),
            nrx2_envelope_and_dac: Default::default(),
            nr43_freq_and_rnd: Default::default(),
            nrx4_ctrl: Default::default(),
            length_timer: LengthTimer::new(ChannelType::CH4),
            envelope_timer: Default::default(),
        }
    }
}

impl NoiseChannel {
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            NR41_CH4_LENGTH_TIMER_ADDRESS => 0xFF,
            NR42_CH4_VOLUME_ENVELOPE_ADDRESS => self.nrx2_envelope_and_dac.byte,
            NR43_CH4_FREQUENCY_RANDOMNESS_ADDRESS => self.nr43_freq_and_rnd.byte,
            NR44_CH4_CONTROL_ADDRESS => self.nrx4_ctrl.read(),
            _ => panic!("Invalid NoiseChannel address: {:#X}", addr),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            NR41_CH4_LENGTH_TIMER_ADDRESS => self.nrx1_len.byte = value,
            NR42_CH4_VOLUME_ENVELOPE_ADDRESS => self.nrx2_envelope_and_dac.byte = value,
            NR43_CH4_FREQUENCY_RANDOMNESS_ADDRESS => self.nr43_freq_and_rnd.byte = value,
            NR44_CH4_CONTROL_ADDRESS => self.nrx4_ctrl.write(value),
            _ => panic!("Invalid NoiseChannel address: {:#X}", addr),
        }
    }

    pub fn tick_length(&mut self, master_ctrl: &mut NR52) {
        self.length_timer.tick(master_ctrl, &mut self.nrx4_ctrl);
    }

    pub fn tick_envelope(&mut self) {
        self.envelope_timer.tick(self.nrx2_envelope_and_dac);
    }
}

/// FF22 — NR43: Channel 4 frequency & randomness
/// This register allows controlling the way the amplitude is randomly switched.
#[derive(Debug, Clone, Default)]
pub struct NR43 {
    byte: u8,
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
