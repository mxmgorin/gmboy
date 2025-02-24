use crate::get_bit_flag;

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
}

/// period low [write-only]
pub struct NRX3 {
    pub byte: u8,
}

pub struct NRX4 {
    pub byte: u8,
}

impl NRX4 {
    pub fn trigger(&self) -> bool {
        get_bit_flag(self.byte, 7)
    }

    pub fn length_enable(&self) -> bool {
        get_bit_flag(self.byte, 7)
    }

    pub fn period(&self) -> u8 {
        self.byte & 0b0000_0111
    }
}
