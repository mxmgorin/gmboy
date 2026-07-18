use crate::ppu::lcd::Lcd;
use crate::ppu::LCD_Y_RES;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LcdWindow {
    // registers
    pub y: u8,
    pub x: u8,
    // additional data
    pub line_number: u8,
}

impl LcdWindow {
    #[inline(always)]
    pub fn on(&self, lcd: &Lcd) -> bool {
        self.is_visible(lcd)
            && lcd.ly >= self.y
            && (lcd.ly as u16) < (self.y as u16 + LCD_Y_RES as u16)
    }
    #[inline(always)]
    pub fn is_visible(&self, lcd: &Lcd) -> bool {
        lcd.control.is_win_enabled() && self.x <= 166 && self.y < LCD_Y_RES
    }

}
