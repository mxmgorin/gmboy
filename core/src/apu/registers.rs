use crate::apu::channels::channel::ChannelType;
use crate::{get_bit_flag, set_bit};
use serde::{Deserialize, Serialize};

pub const NRX4_LENGTH_ENABLE_POS: u8 = 6;

/// FF11 — NR11: Channel 1 length timer & duty cycle
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct NRx1 {
    pub byte: u8,
    ch_type: ChannelType,
}

impl NRx1 {
    pub fn new(ch_type: ChannelType) -> NRx1 {
        Self { byte: 0, ch_type }
    }

    pub fn _read_duty_cycle(&self) -> u8 {
        if self.ch_type == ChannelType::CH3 || self.ch_type == ChannelType::CH4 {
            panic!("CH3 and CH4 doesn't have duty cycle")
        }

        self.byte & 0b1100_0000
    }

    #[inline(always)]
    pub fn get_duty_cycle_idx(&self) -> u8 {
        if self.ch_type == ChannelType::CH3 || self.ch_type == ChannelType::CH4 {
            panic!("CH3 and CH4 doesn't have duty cycle")
        }

        self.byte >> 6
    }

    /// (Write-only): The higher this field is, the shorter the time before the channel is cut.
    #[inline(always)]
    pub fn initial_length_timer(&self) -> u8 {
        self.byte & self.get_length_mask()
    }

    #[inline(always)]
    fn get_length_mask(&self) -> u8 {
        match self.ch_type {
            ChannelType::CH1 | ChannelType::CH2 | ChannelType::CH4 => 0b0011_1111,
            ChannelType::CH3 => 0xFF,
        }
    }
}

/// FF12 — NR12: Channel 1 volume & envelope
/// This register controls the digital amplitude of the “high” part of the pulse, and the sweep applied to that setting.
#[derive(Debug, Clone, Default, Copy, Serialize, Deserialize)]
pub struct NRx2 {
    pub byte: u8,
}

impl NRx2 {
    /// The envelope’s direction; 0 = decrease volume over time, 1 = increase volume over time.
    #[inline(always)]
    pub fn envelope_dir_up(&self) -> bool {
        get_bit_flag(self.byte, 3)
    }

    #[inline(always)]
    pub fn initial_volume(&self) -> u8 {
        self.byte >> 4
    }

    #[inline(always)]
    pub fn sweep_pace(&self) -> u8 {
        self.byte & 0b0000_0111
    }

    #[inline(always)]
    pub fn is_dac_enabled(&self) -> bool {
        (self.byte & 0xF0) != 0
    }
}

/// Merged together NRX3 and NRX4 for convenience
#[derive(Clone, Debug, Default, Copy, Serialize, Deserialize)]
pub struct NRx3x4 {
    pub period_low: NRx3,
    pub nrx4: NRx4,
}

impl NRx3x4 {
    #[inline(always)]
    pub fn get_period(&self) -> u16 {
        u16::from_le_bytes([self.period_low.byte, self.nrx4.get_period()])
    }

    #[inline(always)]
    pub fn set_period(&mut self, value: u16) {
        self.period_low.byte = (value & 0xFF) as u8; // Extract lower 8 bits
        self.nrx4.set_period((value >> 8) as u8);
    }
}

///  Period low, write-only
#[derive(Clone, Debug, Default, Copy, Serialize, Deserialize)]
pub struct NRx3 {
    byte: u8,
}

impl NRx3 {
    #[inline(always)]
    pub fn write(&mut self, value: u8) {
        self.byte = value;
    }
}

/// Period high & length timer control
#[derive(Clone, Debug, Default, Copy, Serialize, Deserialize)]
pub struct NRx4 {
    byte: u8,
}

impl NRx4 {
    /// Read value of 'length enable' bit. Trigger and period are write only
    #[inline(always)]
    pub fn read(&self) -> u8 {
        self.get_length_enable()
    }

    #[inline(always)]
    pub fn write(&mut self, value: u8) {
        self.byte = value;
    }

    #[inline(always)]
    pub fn is_triggered(&self) -> bool {
        get_bit_flag(self.byte, 7)
    }

    #[inline(always)]
    fn get_length_enable(&self) -> u8 {
        let mask = 1 << NRX4_LENGTH_ENABLE_POS;
        self.byte & mask
    }

    #[inline(always)]
    pub fn is_length_enabled(&self) -> bool {
        get_bit_flag(self.byte, NRX4_LENGTH_ENABLE_POS)
    }

    #[inline(always)]
    pub fn disable_length(&mut self) {
        set_bit(&mut self.byte, NRX4_LENGTH_ENABLE_POS, false);
    }

    #[inline(always)]
    pub fn get_period(&self) -> u8 {
        self.byte & 0b0000_0111
    }

    #[inline(always)]
    pub fn set_period(&mut self, value: u8) {
        let value = value & 0b0000_0111; // Extract 3 bits
        self.byte = (self.byte & 0b1111_1000) | value; // Preserve other bits
    }
}
