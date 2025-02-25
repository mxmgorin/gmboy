use crate::apu::ch1_2_square::{
    Ch1, Ch2, SquareChannel, CH1_END_ADDRESS, CH1_START_ADDRESS, CH2_END_ADDRESS, CH2_START_ADDRESS,
};
use crate::apu::ch3_wave::{
    WaveChannel, CH3_END_ADDRESS, CH3_START_ADDRESS, CH3_WAVE_RAM_END, CH3_WAVE_RAM_START,
};
use crate::apu::ch4_noise::{NoiseChannel, CH4_END_ADDRESS, CH4_START_ADDRESS};
use crate::apu::channel::ChannelType;
use crate::apu::frame_sequencer::FrameSequencer;
use crate::{get_bit_flag, set_bit, CPU_CLOCK_SPEED};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

pub const APU_CLOCK_SPEED: u16 = 512;
pub const SAMPLING_FREQUENCY: u16 = 41000;

pub const AUDIO_MASTER_CONTROL_ADDRESS: u16 = 0xFF26;
pub const SOUND_PLANNING_ADDRESS: u16 = 0xFF25;
pub const MASTER_VOLUME_ADDRESS: u16 = 0xFF24;

#[derive(Debug, Clone)]
pub struct Apu {
    // internal mem
    ch1: SquareChannel,
    ch2: SquareChannel,
    ch3: WaveChannel,
    ch4: NoiseChannel,
    master_ctrl: NR52,
    sound_panning: NR51,
    master_volume: NR50,

    // other data
    counter: u16,
    frame_sequencer: FrameSequencer,
    pub buffer: Arc<Mutex<VecDeque<u8>>>,
}

impl Default for Apu {
    fn default() -> Self {
        Self {
            ch1: SquareChannel::Ch1(Ch1::default()),
            ch2: SquareChannel::Ch2(Ch2::default()),
            ch3: WaveChannel::default(),
            ch4: NoiseChannel::default(),
            master_ctrl: NR52::default(),
            sound_panning: NR51::default(),
            master_volume: Default::default(),
            counter: 0,
            frame_sequencer: Default::default(),
            buffer: Arc::new(Mutex::new(Default::default())),
        }
    }
}

impl Apu {
    pub fn tick(&mut self) {
        self.frame_sequencer
            .tick(&mut self.master_ctrl, &mut self.ch3);
        self.counter = self.counter.wrapping_add(1);

        self.ch3.tick(&self.master_ctrl);

        let cpu_cycles_per_sample = (CPU_CLOCK_SPEED / SAMPLING_FREQUENCY as u32) as u16;

        while self.counter >= cpu_cycles_per_sample {
            let (output_left, output_right) = self.mix_channels();
            let mut buffer = self.buffer.lock().unwrap();
            buffer.push_back(output_left);
            buffer.push_back(output_right);

            self.counter -= cpu_cycles_per_sample;
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        if (CH3_WAVE_RAM_START..=CH3_WAVE_RAM_END).contains(&address) {
            self.ch3.wave_ram.write(address, value);
            return;
        }

        if address == AUDIO_MASTER_CONTROL_ADDRESS {
            let prev_enable = self.master_ctrl.is_audio_on();
            self.master_ctrl.write(value);
            
            if !prev_enable && self.master_ctrl.is_audio_on() {
                // turning on
                self.ch3.wave_ram.clear();
            }
            
            return;
        }

        if !self.master_ctrl.is_audio_on() {
            return;
        }

        // todo: the length timers (in NRx1) on monochrome models also writable event when turned off

        match address {
            CH1_START_ADDRESS..=CH1_END_ADDRESS => {}
            CH2_START_ADDRESS..=CH2_END_ADDRESS => {}
            CH3_START_ADDRESS..=CH3_END_ADDRESS => {
                self.ch3.write(address, value, &mut self.master_ctrl)
            }
            CH4_START_ADDRESS..=CH4_END_ADDRESS => {}
            AUDIO_MASTER_CONTROL_ADDRESS => self.master_ctrl.write(value),
            SOUND_PLANNING_ADDRESS => self.sound_panning.byte = value,
            MASTER_VOLUME_ADDRESS => self.master_volume.byte = value,
            CH3_WAVE_RAM_START..=CH3_WAVE_RAM_END => self.ch3.wave_ram.write(address, value),
            _ => panic!("Invalid APU address: {:x}", address),
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            CH1_START_ADDRESS..=CH1_END_ADDRESS => 0,
            CH2_START_ADDRESS..=CH2_END_ADDRESS => 0,
            CH3_START_ADDRESS..=CH3_END_ADDRESS => self.ch3.read(address),
            CH4_START_ADDRESS..=CH4_END_ADDRESS => 0,
            AUDIO_MASTER_CONTROL_ADDRESS => self.master_ctrl.read(),
            SOUND_PLANNING_ADDRESS => self.sound_panning.byte,
            MASTER_VOLUME_ADDRESS => self.master_volume.byte,
            CH3_WAVE_RAM_START..=CH3_WAVE_RAM_END => self.ch3.wave_ram.read(address),
            _ => panic!("Invalid APU address: {:x}", address),
        }
    }

    /// Combines outputs from all channels
    fn mix_channels(&self) -> (u8, u8) {
        let mut left_output = 0;
        let mut right_output = 0;

        // Channel 3
        if self.sound_panning.ch3_left() {
            left_output += self.ch3.get_output(&self.master_ctrl);
        }

        if self.sound_panning.ch3_right() {
            right_output += self.ch3.get_output(&self.master_ctrl);
        }

        // Apply volume control from NR50
        let left_volume = self.master_volume.left_volume();
        let right_volume = self.master_volume.right_volume();

        // Apply NR50 scaling
        (left_output * left_volume, right_output * right_volume)
    }
}

/// FF26 — NR52: Audio master control
#[derive(Debug, Clone, Default)]
pub struct NR52 {
    byte: u8,
}

impl NR52 {
    pub fn write(&mut self, value: u8) {
        let prev_enabled = self.is_audio_on();
        let new_enabled = get_bit_flag(value, 7);

        if !new_enabled && prev_enabled {
            // APU is turning off, clear all but Wave RAM
            self.byte = 0;
        } else if new_enabled {
            // Turn on APU
            self.byte |= 0b1000_0000;
        }
    }

    pub fn read(&self) -> u8 {
        self.byte | 0b0111_0000 // Bits 4-6 always read as 1
    }

    pub fn is_audio_on(&self) -> bool {
        get_bit_flag(self.byte, 7)
    }

    /// Only the status of the channels’ generation circuits is reported
    pub fn is_ch_active(&self, ch_type: ChannelType) -> bool {
        get_bit_flag(self.byte, ch_type.get_enable_bit_pos())
    }

    pub fn deactivate_ch(&mut self, ch_type: &ChannelType) {
        set_bit(&mut self.byte, ch_type.get_enable_bit_pos(), false);
    }

    pub fn activate_ch(&mut self, ch_type: &ChannelType) {
        set_bit(&mut self.byte, ch_type.get_enable_bit_pos(), true);
    }
}

/// FF25 — NR51:
/// Each channel can be panned hard left, center, hard right, or ignored entirely.
/// Setting a bit to 1 enables the channel to go into the selected output.
#[derive(Debug, Clone, Default)]
pub struct NR51 {
    pub byte: u8,
}

impl NR51 {
    pub fn ch4_left(&self) -> bool {
        get_bit_flag(self.byte, 7)
    }
    pub fn ch3_left(&self) -> bool {
        get_bit_flag(self.byte, 6)
    }
    pub fn ch2_left(&self) -> bool {
        get_bit_flag(self.byte, 5)
    }
    pub fn ch1_left(&self) -> bool {
        get_bit_flag(self.byte, 4)
    }
    pub fn ch4_right(&self) -> bool {
        get_bit_flag(self.byte, 3)
    }
    pub fn ch3_right(&self) -> bool {
        get_bit_flag(self.byte, 2)
    }
    pub fn ch2_right(&self) -> bool {
        get_bit_flag(self.byte, 1)
    }
    pub fn ch1_right(&self) -> bool {
        get_bit_flag(self.byte, 0)
    }
}

/// FF24 — NR50: Master volume & VIN panning
/// A value of 0 is treated as a volume of 1 (very quiet), and a value of 7 is treated as a volume of 8 (no volume reduction). Importantly, the amplifier never mutes a non-silent input.
#[derive(Default, Debug, Clone)]
pub struct NR50 {
    pub byte: u8,
}

impl NR50 {
    pub fn left_volume(&self) -> u8 {
        let vol = (self.byte >> 4) & 0b111; // Extract bits 6-4

        vol + 1 // Convert 0-7 to 1-8
    }

    pub fn right_volume(&self) -> u8 {
        let vol = self.byte & 0b111; // Extract bits 2-0

        vol + 1 // Convert 0-7 to 1-8
    }

    pub fn vin_left_enabled(&self) -> bool {
        self.byte & 0b1000_0000 != 0
    }

    pub fn vin_right_enabled(&self) -> bool {
        self.byte & 0b0000_1000 != 0
    }
}
