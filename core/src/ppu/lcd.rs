use crate::cpu::interrupts::{InterruptType, Interrupts};
pub use crate::ppu::tile::{
    PixelColor, BG_TILE_MAP_1_ADDR_START, BG_TILE_MAP_2_ADDR_START, TILE_SET_DATA_1_START,
    TILE_SET_DATA_2_START,
};
use crate::ppu::window::LcdWindow;
use crate::{get_bit_flag, set_bit};
use serde::{Deserialize, Serialize};

pub const LCD_ADDRESS_START: u16 = 0xFF40;
pub const LCD_ADDRESS_END: u16 = 0xFF4B;

// Register addresses
pub const LCD_CONTROL_ADDRESS: u16 = 0xFF40;
pub const LCD_STATUS_ADDRESS: u16 = 0xFF41;
pub const LCD_SCROLL_Y_ADDRESS: u16 = 0xFF42;
pub const LCD_SCROLL_X_ADDRESS: u16 = 0xFF43;
pub const LCD_LY_ADDRESS: u16 = 0xFF44;
pub const LCD_LY_COMPARE_ADDRESS: u16 = 0xFF45;
pub const LCD_DMA_ADDRESS: u16 = 0xFF46;
pub const LCD_BG_PALETTE_ADDRESS: u16 = 0xFF47;
pub const LCD_OBJ_PALETTE_0_ADDRESS: u16 = 0xFF48;
pub const LCD_OBJ_PALETTE_1_ADDRESS: u16 = 0xFF49;
pub const LCD_WINDOW_Y_ADDRESS: u16 = 0xFF4A;
pub const LCD_WINDOW_X_ADDRESS: u16 = 0xFF4B;
const LCD_STATUS_UNUSED_MASK: u8 = 0b1000_0000;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(C)]
pub struct Lcd {
    // Registers
    pub control: LcdControl,
    pub status: LcdStatus,
    pub scroll_y: u8,
    pub scroll_x: u8,
    pub ly: u8,
    pub ly_compare: u8,
    pub dma_byte: u8,
    pub bg_palette: u8,
    pub obj_palette: [u8; 2],
    pub window: LcdWindow,

    // Other data
    pub bg_colors: [PixelColor; 4],
    pub sp1_colors: [PixelColor; 4],
    pub sp2_colors: [PixelColor; 4],
    pub current_colors: [PixelColor; 4],
}

impl Default for Lcd {
    fn default() -> Self {
        Self::new([
            PixelColor::from_hex_rgba("FFFFFFFF"),
            PixelColor::from_hex_rgba("AAAAAAFF"),
            PixelColor::from_hex_rgba("555555FF"),
            PixelColor::from_hex_rgba("000000FF"),
        ])
    }
}

impl Lcd {
    pub fn new(colors: [PixelColor; 4]) -> Self {
        Self {
            control: LcdControl::default(),
            status: LcdStatus::default(),
            scroll_y: 0,
            scroll_x: 0,
            ly: 0,
            ly_compare: 0,
            dma_byte: 0,
            bg_palette: 0xFC,
            obj_palette: [0xFF, 0xFF],
            window: LcdWindow::default(),
            current_colors: colors,
            bg_colors: colors,
            sp1_colors: colors,
            sp2_colors: colors,
        }
    }

    #[inline(always)]
    pub fn read(&self, address: u16) -> u8 {
        match address {
            LCD_CONTROL_ADDRESS => self.control.byte,
            LCD_STATUS_ADDRESS => self.status.byte | LCD_STATUS_UNUSED_MASK,
            LCD_SCROLL_Y_ADDRESS => self.scroll_y,
            LCD_SCROLL_X_ADDRESS => self.scroll_x,
            LCD_LY_ADDRESS => self.ly,
            LCD_LY_COMPARE_ADDRESS => self.ly_compare,
            LCD_DMA_ADDRESS => self.dma_byte,
            LCD_BG_PALETTE_ADDRESS => self.bg_palette,
            LCD_OBJ_PALETTE_0_ADDRESS => self.obj_palette[0],
            LCD_OBJ_PALETTE_1_ADDRESS => self.obj_palette[1],
            LCD_WINDOW_Y_ADDRESS => self.window.y,
            LCD_WINDOW_X_ADDRESS => self.window.x,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn update_palette(&mut self, palette_data: u8, pallet_type: u8) {
        let colors: &mut [PixelColor; 4] = match pallet_type {
            1 => &mut self.sp1_colors,
            2 => &mut self.sp2_colors,
            _ => &mut self.bg_colors,
        };

        colors[0] = self.current_colors[(palette_data & 0b11) as usize];
        colors[1] = self.current_colors[((palette_data >> 2) & 0b11) as usize];
        colors[2] = self.current_colors[((palette_data >> 4) & 0b11) as usize];
        colors[3] = self.current_colors[((palette_data >> 6) & 0b11) as usize];
    }

    #[inline(always)]
    pub fn set_colors(&mut self, colors: [PixelColor; 4]) {
        self.current_colors = colors;
        // re-apply existing palette mappings
        self.update_palette(self.bg_palette, 0);
        self.update_palette(self.obj_palette[0], 1);
        self.update_palette(self.obj_palette[1], 2);
    }

    #[inline(always)]
    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            LCD_CONTROL_ADDRESS => self.control.byte = value,
            LCD_STATUS_ADDRESS => self.status.byte = value,
            LCD_SCROLL_Y_ADDRESS => self.scroll_y = value,
            LCD_SCROLL_X_ADDRESS => self.scroll_x = value,
            LCD_LY_ADDRESS => self.ly = value,
            LCD_LY_COMPARE_ADDRESS => self.ly_compare = value,
            LCD_DMA_ADDRESS => self.dma_byte = value,
            LCD_BG_PALETTE_ADDRESS => {
                self.bg_palette = value;
                self.update_palette(value, 0);
            }
            LCD_OBJ_PALETTE_0_ADDRESS => {
                self.obj_palette[0] = value;
                self.update_palette(value, 1);
            }
            LCD_OBJ_PALETTE_1_ADDRESS => {
                self.obj_palette[1] = value;
                self.update_palette(value, 2);
            }
            LCD_WINDOW_Y_ADDRESS => self.window.y = value,
            LCD_WINDOW_X_ADDRESS => self.window.x = value,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    pub fn increment_ly(&mut self, interrupts: &mut Interrupts) {
        if self.window.is_visible(self) && self.window.on(self) {
            self.window.line_number += 1;
        }

        self.ly += 1;
        self.compare_ly(interrupts);
    }

    #[inline(always)]
    pub fn reset_ly(&mut self, interrupts: &mut Interrupts) {
        self.ly = 0;
        self.window.line_number = 0;
        self.compare_ly(interrupts);
    }

    #[inline(always)]
    fn compare_ly(&mut self, interrupts: &mut Interrupts) {
        if self.ly == self.ly_compare {
            self.status.set_lyc(true);

            if self.status.is_stat_interrupt(LcdStatSrc::Lyc) {
                interrupts.request_interrupt(InterruptType::LCDStat);
            } else {
                self.status.set_lyc(false);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[repr(C)]
pub struct LcdControl {
    pub byte: u8,
}

impl Default for LcdControl {
    fn default() -> Self {
        Self { byte: 0x91 }
    }
}

impl LcdControl {
    #[inline(always)]
    pub fn is_bgw_enabled(&self) -> bool {
        get_bit_flag(self.byte, 0)
    }
    #[inline(always)]
    pub fn is_obj_enabled(&self) -> bool {
        get_bit_flag(self.byte, 1)
    }

    #[inline(always)]
    pub fn get_obj_height(&self) -> u8 {
        if get_bit_flag(self.byte, 2) {
            16
        } else {
            8
        }
    }

    #[inline(always)]
    pub fn get_bg_map_area(&self) -> u16 {
        if get_bit_flag(self.byte, 3) {
            BG_TILE_MAP_2_ADDR_START
        } else {
            BG_TILE_MAP_1_ADDR_START
        }
    }

    #[inline(always)]
    pub fn get_bgw_data_area(&self) -> u16 {
        if get_bit_flag(self.byte, 4) {
            TILE_SET_DATA_1_START
        } else {
            TILE_SET_DATA_2_START
        }
    }

    #[inline(always)]
    pub fn is_win_enabled(&self) -> bool {
        get_bit_flag(self.byte, 5)
    }

    #[inline(always)]
    pub fn get_win_map_area(&self) -> u16 {
        if get_bit_flag(self.byte, 6) {
            BG_TILE_MAP_2_ADDR_START
        } else {
            BG_TILE_MAP_1_ADDR_START
        }
    }

    #[inline(always)]
    pub fn is_lcd_enabled(&self) -> bool {
        get_bit_flag(self.byte, 7)
    }
}

pub const PPU_MODE_MASK: u8 = 0b11;

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[repr(C)]
pub struct LcdStatus {
    pub byte: u8,
}

impl LcdStatus {
    #[inline(always)]
    pub fn get_ppu_mode(&self) -> PpuMode {
        PpuMode::from(self.byte)
    }

    #[inline(always)]
    pub fn set_ppu_mode(&mut self, mode: PpuMode) {
        self.byte &= !PPU_MODE_MASK;
        self.byte |= mode as u8;
    }

    #[inline(always)]
    pub fn get_lyc(&self) -> bool {
        get_bit_flag(self.byte, 2)
    }

    #[inline(always)]
    pub fn set_lyc(&mut self, b: bool) {
        set_bit(&mut self.byte, 2, b);
    }

    #[inline(always)]
    pub fn is_stat_interrupt(&self, src: LcdStatSrc) -> bool {
        self.byte & (src as u8) != 0
    }
}

#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum PpuMode {
    HBlank = 0,
    VBlank = 1,
    Oam = 2,
    Transfer = 3,
}

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum LcdStatSrc {
    HBlank = 1 << 3,
    VBlank = 1 << 4,
    Oam = 1 << 5,
    Lyc = 1 << 6,
}

impl From<u8> for PpuMode {
    #[inline(always)]
    fn from(value: u8) -> Self {
        match value & PPU_MODE_MASK {
            0 => PpuMode::HBlank,
            1 => PpuMode::VBlank,
            2 => PpuMode::Oam,
            3 => PpuMode::Transfer,
            _ => unreachable!(),
        }
    }
}
