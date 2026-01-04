use serde::{Deserialize, Serialize};

pub const JOYPAD_ADDR: u16 = 0xFF00;

pub const A_OR_RIGHT_BIT: u8 = 0x00;
pub const B_OR_LEFT_BIT: u8 = 0x01;
pub const SELECT_OR_UP_BIT: u8 = 0x02;
pub const START_OR_DOWN_BIT: u8 = 0x03;

pub const SELECT_DIRECTIONS_BIT: u8 = 0x04;
pub const SELECT_ACTIONS_BIT: u8 = 0x05;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum JoypadButton {
    Start,
    Select,
    A,
    B,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
    #[inline(always)]
    pub fn handle(&mut self, button: JoypadButton, is_pressed: bool) {
        match button {
            JoypadButton::Start => self.start = is_pressed,
            JoypadButton::Select => self.select = is_pressed,
            JoypadButton::A => self.a = is_pressed,
            JoypadButton::B => self.b = is_pressed,
            JoypadButton::Up => self.up = is_pressed,
            JoypadButton::Down => self.down = is_pressed,
            JoypadButton::Left => self.left = is_pressed,
            JoypadButton::Right => self.right = is_pressed,
        }
    }

    #[inline(always)]
    pub fn get_byte(&self) -> u8 {
        if self.actions_selected {
            ((!self.a as u8) << A_OR_RIGHT_BIT)
                | ((!self.b as u8) << B_OR_LEFT_BIT)
                | ((!self.select as u8) << SELECT_OR_UP_BIT)
                | ((!self.start as u8) << START_OR_DOWN_BIT)
        } else if self.directions_selected {
            ((!self.right as u8) << A_OR_RIGHT_BIT)
                | ((!self.left as u8) << B_OR_LEFT_BIT)
                | ((!self.up as u8) << SELECT_OR_UP_BIT)
                | ((!self.down as u8) << START_OR_DOWN_BIT)
        } else {
            0xCF
        }
    }

    #[inline(always)]
    pub fn set_byte(&mut self, value: u8) {
        self.directions_selected = (value & 0x10) == 0;
        self.actions_selected = (value & 0x20) == 0;
    }

    pub fn reset(&mut self) {
        self.start = false;
        self.select = false;
        self.a = false;
        self.b = false;
        self.up = false;
        self.down = false;
        self.left = false;
        self.right = false;
        self.directions_selected = false;
        self.actions_selected = false;
    }
}
