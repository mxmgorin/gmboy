use crate::bus::Bus;
use crate::ppu::lcd::Lcd;
use crate::ppu::{LCD_X_RES, LCD_Y_RES};
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

    #[inline(always)]
    pub fn get_tile_idx(&self, fetch_x: u16, bus: &Bus) -> Option<u8> {
        if !self.is_visible(&bus.io.lcd) {
            return None;
        }

        let fetch_x = fetch_x + 7;

        if fetch_x >= self.x as u16
            && fetch_x < self.x as u16 + LCD_X_RES as u16 + 14
            && bus.io.lcd.ly as u16 >= self.y as u16
            && (bus.io.lcd.ly as u16) < self.y as u16 + LCD_Y_RES as u16
        {
            let w_tile_x = (fetch_x - self.x as u16) / 8; // Convert pixel X to tile X
            let w_tile_y = (self.line_number / 8) as u16; // Convert pixel Y to tile Y
            let area = bus.io.lcd.control.get_win_map_area(); // Get window tile map base address (0x9800 or 0x9C00)
            let addr = area + w_tile_x + (w_tile_y * 32); // Calculate correct tile map index

            return Some(bus.read(addr)); // Fetch tile index from VRAM
        }

        None
    }
}
