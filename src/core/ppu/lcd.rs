use crate::cpu::interrupts::{InterruptType, Interrupts};
use crate::{get_bit_flag, set_bit, struct_to_bytes, struct_to_bytes_mut};

pub const LCD_ADDRESS_START: u16 = 0xFF40;
pub const LCD_ADDRESS_END: u16 = 0xFF4B;

// Register addresses
pub const LCD_CONTROL_ADDRESS: u16 = 0xFF40;
pub const LCD_STATUS_ADDRESS: u16 = 0xFF41;
pub const LCD_SCROLL_Y_ADDRESS: u16 = 0xFF42;
pub const LCD_SCROLL_X_ADDRESS: u16 = 0xFF42;
pub const LCD_LY_ADDRESS: u16 = 0xFF44;
pub const LCD_LY_COMPARE_ADDRESS: u16 = 0xFF45;
pub const LCD_DMA_ADDRESS: u16 = 0xFF46;
pub const LCD_BG_PALETTE_ADDRESS: u16 = 0xFF47;
pub const LCD_TILE_PALETTE_0_ADDRESS: u16 = 0xFF48;
pub const LCD_TILE_PALETTE_1_ADDRESS: u16 = 0xFF49;
pub const LCD_WINDOW_Y_ADDRESS: u16 = 0xFF4A;
pub const LCD_WINDOW_X_ADDRESS: u16 = 0xFF4B;

pub const DEFAULT_COLORS: [u32; 4] = [0xFFFFFFFF, 0xC0C0C0FF, 0x444444FF, 0x000000FF];

#[derive(Debug, Clone)]
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
    pub win_y: u8,
    pub win_x: u8,

    // Other data
    pub bg_colors: [u32; 4],
    pub sp1_colors: [u32; 4],
    pub sp2_colors: [u32; 4],
}

impl Default for Lcd {
    fn default() -> Self {
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
            win_y: 0,
            win_x: 0,
            bg_colors: DEFAULT_COLORS,
            sp1_colors: DEFAULT_COLORS,
            sp2_colors: DEFAULT_COLORS,
        }
    }
}

impl Lcd {
    pub fn read(&self, address: u16) -> u8 {
        let offset = (address - LCD_ADDRESS_START) as usize;
        let bytes = struct_to_bytes(self);

        bytes[offset]
    }

    pub fn update_palette(&mut self, palette_data: u8, pal: u8) {
        let p_colors: &mut [u32; 4] = match pal {
            1 => &mut self.sp1_colors,
            2 => &mut self.sp2_colors,
            _ => &mut self.bg_colors,
        };

        p_colors[0] = DEFAULT_COLORS[(palette_data & 0b11) as usize];
        p_colors[1] = DEFAULT_COLORS[((palette_data >> 2) & 0b11) as usize];
        p_colors[2] = DEFAULT_COLORS[((palette_data >> 4) & 0b11) as usize];
        p_colors[3] = DEFAULT_COLORS[((palette_data >> 6) & 0b11) as usize];
    }

    pub fn write(&mut self, address: u16, value: u8) {
        let offset = (address - LCD_ADDRESS_START) as usize;

        let self_bytes = struct_to_bytes_mut(self);
        self_bytes[offset] = value;

        match address {
            0xFF47 => self.update_palette(value, 0),
            0xFF48 => self.update_palette(value & 0b11111100, 1),
            0xFF49 => self.update_palette(value & 0b11111100, 2),
            _ => {}
        }
    }

    pub fn increment_ly(&mut self, interrupts: &mut Interrupts) {
        self.ly += 1;

        if self.ly == self.ly_compare {
            self.status.lyc_set(true);

            if self.status.is_stat_interrupt(LcdStatSrc::Lyc) {
                interrupts.request_interrupt(InterruptType::LCDStat);
            } else {
                self.status.lyc_set(false);
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
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
    pub fn obj_enable(&self) -> bool {
        get_bit_flag(self.byte, 1)
    }

    pub fn obj_height(&self) -> u8 {
        if get_bit_flag(self.byte, 2) {
            16
        } else {
            8
        }
    }

    pub fn bg_map_area(&self) -> u16 {
        if get_bit_flag(self.byte, 3) {
            0x9C00
        } else {
            0x9800
        }
    }

    pub fn bgw_data_area(&self) -> u16 {
        if get_bit_flag(self.byte, 4) {
            0x8000
        } else {
            0x8800
        }
    }

    pub fn win_enable(&self) -> bool {
        get_bit_flag(self.byte, 5)
    }

    pub fn win_map_area(&self) -> u16 {
        if get_bit_flag(self.byte, 6) {
            0x9C00
        } else {
            0x9800
        }
    }

    pub fn lcd_enable(&self) -> bool {
        get_bit_flag(self.byte, 7)
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct LcdStatus {
    pub byte: u8,
}

impl LcdStatus {
    pub fn mode(&self) -> LcdMode {
        LcdMode::from(self.byte)
    }

    pub fn mode_set(&mut self, mode: LcdMode) {
        self.byte &= !0b11;
        self.byte |= mode as u8;
    }

    pub fn lyc(&self) -> bool {
        get_bit_flag(self.byte, 2)
    }

    pub fn lyc_set(&mut self, b: bool) {
        set_bit(&mut self.byte, 2, b);
    }

    pub fn is_stat_interrupt(&self, src: LcdStatSrc) -> bool {
        self.byte & (src as u8) != 0
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum LcdMode {
    HBlank,
    VBlank,
    Oam,
    Xfer,
}

#[derive(Copy, Clone)]
pub enum LcdStatSrc {
    HBlank = 1 << 3,
    VBlank = 1 << 4,
    Oam = 1 << 5,
    Lyc = 1 << 6,
}

impl From<u8> for LcdMode {
    fn from(value: u8) -> Self {
        match value & 0b11 {
            0 => LcdMode::HBlank,
            1 => LcdMode::VBlank,
            2 => LcdMode::Oam,
            3 => LcdMode::Xfer,
            _ => unreachable!(),
        }
    }
}

impl Into<u8> for LcdMode {
    fn into(self) -> u8 {
        match self {
            LcdMode::HBlank => 0,
            LcdMode::VBlank => 1,
            LcdMode::Oam => 2,
            LcdMode::Xfer => 3,
        }
    }
}
