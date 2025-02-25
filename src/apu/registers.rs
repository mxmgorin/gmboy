use crate::{get_bit_flag, set_bit};

pub const NRX4_LENGTH_ENABLE_POS: u8 = 6;

/// FF11 — NR11: Channel 1 length timer & duty cycle
pub struct NRX1 {
    pub byte: u8,
}

impl NRX1 {
    pub fn duty_cycle(&self) -> u8 {
        self.byte & 0b1100_0000
    }

    /// (Write-only): The higher this field is, the shorter the time before the channel is cut.
    pub fn initial_length_timer(&self) -> u8 {
        self.byte & 0b0011_1111
    }
}

/// FF12 — NR12: Channel 1 volume & envelope
/// This register controls the digital amplitude of the “high” part of the pulse, and the sweep applied to that setting.
pub struct NRX2 {
    pub byte: u8,
}

impl NRX2 {
    /// The envelope’s direction; 0 = decrease volume over time, 1 = increase volume over time.
    pub fn env_dir(&self) -> bool {
        get_bit_flag(self.byte, 3)
    }

    pub fn initial_volume(&self) -> u8 {
        self.byte & 0b1111_0000
    }

    pub fn sweep_pace(&self) -> u8 {
        self.byte & 0b0000_0111
    }

    pub fn dac_enabled(&self) -> bool {
        (self.byte & 0xF0) != 0
    }
}

/// period low [write-only]
pub struct NRX3 {
    pub byte: u8,
}

pub struct NRX4 {
    byte: u8,
}

impl NRX4 {
    pub fn read(&self) -> u8 {
        self.get_length_enable() // trigger and period are write only
    }

    pub fn trigger(&self) -> bool {
        get_bit_flag(self.byte, 7)
    }

    fn get_length_enable(&self) -> u8 {
        let mask = 1 << NRX4_LENGTH_ENABLE_POS;
        self.byte & mask
    }

    pub fn is_length_enabled(&self) -> bool {
        get_bit_flag(self.byte, NRX4_LENGTH_ENABLE_POS)
    }

    pub fn disable_length(&mut self) {
        set_bit(&mut self.byte, NRX4_LENGTH_ENABLE_POS, false);
    }

    pub fn period(&self) -> u8 {
        self.byte & 0b0000_0111
    }
}
