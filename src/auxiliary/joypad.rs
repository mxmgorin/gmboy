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
            return ((!self.a as u8) << A_RIGHT_BIT)
                | ((!self.b as u8) << B_LEFT_BIT)
                | ((!self.select as u8) << SELECT_UP_BIT)
                | ((!self.start as u8) << START_DOWN_BIT);
        }

        if self.directions_selected {
            return ((!self.right as u8) << A_RIGHT_BIT)
                | ((!self.left as u8) << B_LEFT_BIT)
                | ((!self.up as u8) << SELECT_UP_BIT)
                | ((!self.down as u8) << START_DOWN_BIT);
        }

        0xFF
    }

    pub fn set_byte(&mut self, value: u8) {
        self.directions_selected = (value >> SELECT_DIRECTIONS_BIT) & 0x01 == 0;
        self.actions_selected = (value >> SELECT_ACTIONS_BIT) & 0x01 == 0;
    }
}
