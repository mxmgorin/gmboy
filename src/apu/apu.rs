use crate::apu::ch1_2_square::{
    SquareChannel, CH1_END_ADDRESS, CH1_START_ADDRESS, CH2_END_ADDRESS, CH2_START_ADDRESS,
};
use crate::apu::ch3_wave::{
    WaveChannel, CH3_END_ADDRESS, CH3_START_ADDRESS, CH3_WAVE_RAM_END, CH3_WAVE_RAM_START,
};
use crate::apu::ch4_noise::{NoiseChannel, CH4_END_ADDRESS, CH4_START_ADDRESS};
use crate::apu::channel::ChannelType;
use crate::apu::frame_sequencer::FrameSequencer;
use crate::{get_bit_flag, set_bit};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

pub const APU_CLOCK_SPEED: u16 = 512;

pub const AUDIO_MASTER_CONTROL_ADDRESS: u16 = 0xFF26;
pub const SOUND_PLANNING_ADDRESS: u16 = 0xFF25;
pub const MASTER_VOLUME_ADDRESS: u16 = 0xFF24;

pub struct Apu {
    // internal mem
    ch1: SquareChannel,
    ch2: SquareChannel,
    ch3: WaveChannel,
    ch4: NoiseChannel,
    master_ctrl: NR52,
    sound_panning: NR51,
    master_volume: NR50,
    // others
    counter: u16,
    frame_sequencer: FrameSequencer,
}

impl Apu {
    pub fn tick(&mut self) {
        self.frame_sequencer.tick();
        self.counter = self.counter.wrapping_add(1);

        self.ch3.tick(&self.master_ctrl);
    }

    pub fn write(&mut self, address: u16, value: u8) {
        if (CH3_WAVE_RAM_START..=CH3_WAVE_RAM_END).contains(&address) {
            self.ch3.wave_ram.write(address, value);
            return;
        }

        if address == AUDIO_MASTER_CONTROL_ADDRESS {
            self.master_ctrl.write(value);
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
}

/// FF26 — NR52: Audio master control
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
        if vol == 0 {
            1
        } else {
            vol + 1
        } // 0 -> 1, 1-7 -> 2-8
    }

    pub fn right_volume(&self) -> u8 {
        let vol = self.byte & 0b111; // Extract bits 2-0
        if vol == 0 {
            1
        } else {
            vol + 1
        } // 0 -> 1, 1-7 -> 2-8
    }

    pub fn vin_left_enabled(&self) -> bool {
        self.byte & 0b1000_0000 != 0
    }

    pub fn vin_right_enabled(&self) -> bool {
        self.byte & 0b0000_1000 != 0
    }
}
