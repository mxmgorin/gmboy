use crate::bus::Bus;
use crate::ppu::lcd::Lcd;
use crate::ppu::{LCD_X_RES, LCD_Y_RES};

#[derive(Debug, Clone, Default)]
pub struct Window {
    // registers
    pub y: u8,
    pub x: u8,
    // additional data
    pub line_number: u8,
}

impl Window {
    pub fn on(&self, lcd: &Lcd) -> bool {
        self.is_visible(lcd) && lcd.ly >= self.y && lcd.ly < self.y + LCD_Y_RES
    }
    pub fn is_visible(&self, lcd: &Lcd) -> bool {
        lcd.control.win_enable() && self.x <= 166 && self.y < LCD_Y_RES
    }

    pub fn get_tile_idx(&self, fetch_x: u16, bus: &Bus) -> Option<u8> {
        if !self.is_visible(&bus.io.lcd) {
            return None;
        }

        let fetch_x = fetch_x + 7;

        if fetch_x >= self.x as u16
            && fetch_x < self.x as u16 + LCD_Y_RES as u16 + 14
            && bus.io.lcd.ly as u16 >= self.y as u16
            && (bus.io.lcd.ly as u16) < self.y as u16 + LCD_X_RES as u16
        {
            let w_tile_y = self.line_number.wrapping_div(8) as u16;
            let area = bus.io.lcd.control.win_map_area();
            let addr = area
                .wrapping_add(fetch_x)
                .wrapping_sub(self.x as u16)
                .wrapping_div(8)
                .wrapping_add(w_tile_y * 32);

            return Some(bus.read(addr));
        }

        None
    }
}
