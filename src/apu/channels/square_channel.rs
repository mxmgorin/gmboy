use crate::apu::channels::channel::ChannelType;
use crate::apu::dac::{DacEnable, DigitalSampleProducer};
use crate::apu::registers::{NRx1, NRx2, NRx3x4};
use crate::apu::timers::length_timer::LengthTimer;
use crate::apu::timers::period_timer::PeriodTimer;
use crate::apu::NR52;
use crate::get_bit_flag;
use crate::timers::envelope_timer::EnvelopeTimer;
use crate::timers::sweep_timer::SweepTimer;

pub const CH1_START_ADDRESS: u16 = NR10_CH1_SWEEP_ADDRESS;
pub const CH1_END_ADDRESS: u16 = NR14_CH1_PERIOD_HIGH_CONTROL_ADDRESS;

pub const CH2_START_ADDRESS: u16 = NR21_CH2_LEN_TIMER_DUTY_CYCLE_ADDRESS;
pub const CH2_END_ADDRESS: u16 = NR24_CH2_PERIOD_HIGH_CONTROL_ADDRESS;

pub const NR10_CH1_SWEEP_ADDRESS: u16 = 0xFF10;
pub const NR11_CH1_LEN_TIMER_DUTY_CYCLE_ADDRESS: u16 = 0xFF11;
pub const NR12_CH1_VOL_ENVELOPE_ADDRESS: u16 = 0xFF12;
pub const NR13_CH1_PERIOD_LOW_ADDRESS: u16 = 0xFF13;
pub const NR14_CH1_PERIOD_HIGH_CONTROL_ADDRESS: u16 = 0xFF14;

pub const NR21_CH2_LEN_TIMER_DUTY_CYCLE_ADDRESS: u16 = 0xFF16;
pub const NR22_CH2_VOL_ENVELOPE_ADDRESS: u16 = 0xFF17;
pub const NR23_CH2_PERIOD_LOW_ADDRESS: u16 = 0xFF18;
pub const NR24_CH2_PERIOD_HIGH_CONTROL_ADDRESS: u16 = 0xFF19;

pub const WAVE_DUTY_PATTERNS: [[u8; 8]; 4] = [
    [0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 1, 1, 1],
    [0, 1, 1, 1, 1, 1, 1, 0],
];

#[derive(Debug, Clone)]
pub struct SquareChannel {
    // registers
    sweep_timer: Option<SweepTimer>,
    nrx1_len_timer_duty_cycle: NRx1,
    nrx2_volume_envelope_and_dac: NRx2,
    nrx3x4_period_and_ctrl: NRx3x4,

    // other data
    ch_type: ChannelType,
    period_timer: PeriodTimer,
    length_timer: LengthTimer,
    envelope_timer: EnvelopeTimer,
    duty_sequence: usize,
}

impl DacEnable for SquareChannel {
    fn is_dac_enabled(&self) -> bool {
        self.nrx2_volume_envelope_and_dac.is_dac_enabled()
    }
}

impl DigitalSampleProducer for SquareChannel {
    fn get_sample(&self, master_ctrl: NR52) -> u8 {
        if master_ctrl.is_ch_active(&self.ch_type) {
            let duty_cycle = self.nrx1_len_timer_duty_cycle.get_duty_cycle_idx() as usize;

            let output = if WAVE_DUTY_PATTERNS[duty_cycle][self.duty_sequence] == 1 {
                self.envelope_timer.get_volume()
            } else {
                0
            };

            return output;
        }

        0
    }
}

impl SquareChannel {
    pub fn ch1() -> SquareChannel {
        Self::new(ChannelType::CH1)
    }

    pub fn ch2() -> SquareChannel {
        Self::new(ChannelType::CH2)
    }

    fn new(ch_type: ChannelType) -> Self {
        Self {
            sweep_timer: if ch_type == ChannelType::CH1 {
                Some(Default::default())
            } else {
                None
            },
            nrx1_len_timer_duty_cycle: NRx1::new(ch_type),
            nrx2_volume_envelope_and_dac: Default::default(),
            nrx3x4_period_and_ctrl: Default::default(),
            ch_type,
            length_timer: LengthTimer::new(ch_type),
            period_timer: PeriodTimer::new(ch_type),
            duty_sequence: 0,
            envelope_timer: Default::default(),
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        let mut offset = self.get_offset(address);

        if let Some(sweep_timer) = &self.sweep_timer {
            if offset == 0 {
                return sweep_timer.nr10.byte;
            }
        } else {
            offset += 1;
        }

        match offset {
            1 => self.nrx1_len_timer_duty_cycle.byte,
            2 => self.nrx2_volume_envelope_and_dac.byte,
            3 => 0xFF,
            4 => self.nrx3x4_period_and_ctrl.nrx4.read(),
            _ => panic!("Invalid Square address: {:#X}", address),
        }
    }

    pub fn write(&mut self, address: u16, value: u8, master_ctrl: &mut NR52) {
        let mut offset = self.get_offset(address);

        if let Some(sweep_timer) = self.sweep_timer.as_mut() {
            if offset == 0 {
                sweep_timer.nr10.byte = value;
                return;
            }
        } else {
            offset += 1;
        }

        match offset {
            1 => self.nrx1_len_timer_duty_cycle.byte = value,
            // Writes to this register while the channel is on require re-triggering it after wards.
            // If the write turns the channel off, re-triggering is not necessary (it would do nothing).
            2 => self.nrx2_volume_envelope_and_dac.byte = value,
            3 => self.nrx3x4_period_and_ctrl.period_low.write(value),
            4 => {
                self.nrx3x4_period_and_ctrl.nrx4.write(value);

                if self.nrx3x4_period_and_ctrl.nrx4.is_triggered() {
                    self.trigger(master_ctrl);
                }
            }
            _ => panic!("Invalid Square address: {:#X}", address),
        }
    }

    fn get_offset(&self, address: u16) -> u16 {
        address - self.ch_type.get_start_address()
    }

    pub fn tick_envelope(&mut self) {
        self.envelope_timer.tick(self.nrx2_volume_envelope_and_dac);
    }

    pub fn tick_sweep(&mut self, nr52: &mut NR52) {
        if let Some(sweep) = self.sweep_timer.as_mut() {
            sweep.tick(nr52, &mut self.nrx3x4_period_and_ctrl);
        }
    }

    pub fn tick_length(&mut self, master_ctrl: &mut NR52) {
        self.length_timer
            .tick(master_ctrl, &mut self.nrx3x4_period_and_ctrl.nrx4);
    }

    pub fn tick(&mut self) {
        if self.period_timer.tick(&self.nrx3x4_period_and_ctrl) {
            self.duty_sequence = (self.duty_sequence + 1) & 0x07;
        }
    }

    fn trigger(&mut self, nr52: &mut NR52) {
        nr52.activate_ch(&self.ch_type);

        if self.length_timer.is_expired() {
            self.length_timer.reload(&self.nrx1_len_timer_duty_cycle);
        }

        self.period_timer.reload(&self.nrx3x4_period_and_ctrl);
        self.envelope_timer
            .reload(self.nrx2_volume_envelope_and_dac);

        if let Some(sweep_timer) = self.sweep_timer.as_mut() {
            sweep_timer.reload(nr52, self.nrx3x4_period_and_ctrl);
        }
    }
}

/// FF10 — NR10: Channel 1 sweep
/// This register controls CH1’s period sweep functionality.
#[derive(Debug, Clone, Default, Copy)]
pub struct NR10 {
    pub byte: u8,
}

impl NR10 {
    /// This dictates how often sweep “iterations” happen, in units of 128 Hz ticks5 (7.8 ms). Note
    /// that the value written to this field is not re-read by the hardware until a sweep iteration
    /// completes, or the channel is (re)triggered.
    /// However, if 0 is written to this field, then iterations are instantly disabled (but see below),
    /// and it will be reloaded as soon as it’s set to something else.
    pub fn pace(&self) -> u8 {
        self.byte & 0b0111_0000
    }

    /// 0 = Addition (period increases); 1 = Subtraction (period decreases)
    pub fn direction_down(&self) -> bool {
        get_bit_flag(self.byte, 3)
    }

    pub fn individual_step(&self) -> u8 {
        self.byte & 0b0000_0111
    }
}
