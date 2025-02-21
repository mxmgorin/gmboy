pub const JOYPAD_ADDR: u16 = 0xFF00;

pub const A_RIGHT_BIT: u8 = 0x00;
pub const B_LEFT_BIT: u8 = 0x01;
pub const SELECT_UP_BIT: u8 = 0x02;
pub const START_DOWN_BIT: u8 = 0x03;

pub const SELECT_DIRECTIONS_BIT: u8 = 0x04;
pub const SELECT_ACTIONS_BIT: u8 = 0x05;

#[derive(Debug, Clone, Default)]
pub struct Joypad {
    pub start: bool,
    pub select: bool,
    pub a: bool,
    pub b: bool,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,

    pub directions_selected: bool,
    pub actions_selected: bool,
}

impl Joypad {
    pub fn get_byte(&self) -> u8 {
        if self.actions_selected {
            return (if self.a { 0 } else { 1 }) << A_RIGHT_BIT
                | (if self.b { 0 } else { 1 }) << B_LEFT_BIT
                | (if self.select { 0 } else { 1 }) << SELECT_UP_BIT
                | (if self.start { 0 } else { 1 }) << START_DOWN_BIT;
        }

        if self.directions_selected {
            return (if self.right { 0 } else { 1 }) << A_RIGHT_BIT
                | (if self.left { 0 } else { 1 }) << B_LEFT_BIT
                | (if self.up { 0 } else { 1 }) << SELECT_UP_BIT
                | (if self.down { 0 } else { 1 }) << START_DOWN_BIT;
        }

        0xCF
    }

    pub fn set_byte(&mut self, value: u8) {
        self.directions_selected = (value >> SELECT_DIRECTIONS_BIT) & 0x01 == 0;
        self.actions_selected = (value >> SELECT_ACTIONS_BIT) & 0x01 == 0;
    }
}
