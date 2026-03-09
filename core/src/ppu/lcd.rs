use crate::cpu::interrupts::{InterruptType, Interrupts};
use crate::emu::config::GbModel;
use crate::ppu::tile::TileFlags;
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
pub const CGB_PALLETE_START_ADDR: u16 = CGB_BG_PALLETE_INDEX_ADDR;
pub const CGB_PALLETE_END_ADDR: u16 = CGB_OBJ_PALLETE_DATA_ADDR;
/// BGPI
pub const CGB_BG_PALLETE_INDEX_ADDR: u16 = 0xFF68;
/// BGPD
pub const CGB_BG_PALLETE_DATA_ADDR: u16 = 0xFF69;
/// OBPI
pub const CGB_OBJ_PALLETE_INDEX_ADDR: u16 = 0xFF6A;
/// OBPD
pub const CGB_OBJ_PALLETE_DATA_ADDR: u16 = 0xFF6B;
/// OPRI
pub const CGB_OBJ_PRIORITY_MODE_ADDR: u16 = 0xFF6C;

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
    pub obj_priority_mode: u8,
    pub window: LcdWindow,
    pub dmg_palette: DmgPalette,
    pub cgb_palette: CgbPalette,
    pub model: GbModel,
}

impl Default for Lcd {
    fn default() -> Self {
        Self::new(
            [
                PixelColor::from_hex_rgba("FFFFFFFF"),
                PixelColor::from_hex_rgba("AAAAAAFF"),
                PixelColor::from_hex_rgba("555555FF"),
                PixelColor::from_hex_rgba("000000FF"),
            ],
            GbModel::default(),
        )
    }
}

impl Lcd {
    pub fn new(colors: [PixelColor; 4], model: GbModel) -> Self {
        Self {
            control: LcdControl::default(),
            status: LcdStatus::default(),
            scroll_y: 0,
            scroll_x: 0,
            ly: 0,
            ly_compare: 0,
            dma_byte: 0,
            window: LcdWindow::default(),
            dmg_palette: DmgPalette::new(colors),
            cgb_palette: CgbPalette::default(),
            obj_priority_mode: match model {
                GbModel::Cgb => 0x0,
                GbModel::Dmg => 0x1,
            },
            model,
        }
    }

    #[inline(always)]
    pub fn is_vram_blocked(&self) -> bool {
        self.status.get_ppu_mode() == PpuMode::Transfer && self.control.is_lcd_enabled()
    }

    #[inline(always)]
    pub fn is_oam_blocked(&self) -> bool {
        let mode = self.status.get_ppu_mode();

        (mode == PpuMode::Transfer || mode == PpuMode::Oam) && self.control.is_lcd_enabled()
    }

    #[inline(always)]
    pub fn set_model(&mut self, model: GbModel) {
        self.obj_priority_mode = match model {
            GbModel::Cgb => 0x0,
            GbModel::Dmg => 0x1,
        };
        self.model = model;
    }

    #[inline(always)]
    pub fn read_obj_priority_mode(&self) -> u8 {
        self.obj_priority_mode
    }

    #[inline(always)]
    pub fn write_obj_priority_mode(&mut self, val: u8) {
        self.obj_priority_mode = val;
    }

    #[inline(always)]
    pub fn is_dmg_obj_priority_mode(&self) -> bool {
        self.obj_priority_mode & 0x1 != 0
    }

    #[inline(always)]
    pub fn get_obj_color(&self, flags: TileFlags, color_idx: usize) -> PixelColor {
        match self.model {
            GbModel::Cgb => self
                .cgb_palette
                .get_color(flags.read_cgb_palette(), color_idx, true),
            GbModel::Dmg => self
                .dmg_palette
                .get_obj_color(flags.is_second_dmg_palette(), color_idx),
        }
    }

    pub fn get_bgw_color(&self, color_idx: usize, enabled: bool, flags: TileFlags) -> PixelColor {
        match self.model {
            GbModel::Cgb => self
                .cgb_palette
                .get_color(flags.read_cgb_palette(), color_idx, false),
            GbModel::Dmg => self.dmg_palette.get_gbw_color(color_idx, enabled),
        }
    }

    #[inline(always)]
    pub fn read(&self, address: u16) -> u8 {
        match address {
            LCD_CONTROL_ADDRESS => self.control.byte,
            LCD_STATUS_ADDRESS => {
                let val = if self.control.is_lcd_enabled() {
                    self.status.byte
                } else {
                    PpuMode::HBlank as u8
                };

                val | LCD_STATUS_UNUSED_MASK
            }
            LCD_SCROLL_Y_ADDRESS => self.scroll_y,
            LCD_SCROLL_X_ADDRESS => self.scroll_x,
            LCD_LY_ADDRESS => self.ly,
            LCD_LY_COMPARE_ADDRESS => self.ly_compare,
            LCD_DMA_ADDRESS => self.dma_byte,
            LCD_BG_PALETTE_ADDRESS => self.dmg_palette.bg_palette,
            LCD_OBJ_PALETTE_0_ADDRESS => self.dmg_palette.obj_palette[0],
            LCD_OBJ_PALETTE_1_ADDRESS => self.dmg_palette.obj_palette[1],
            LCD_WINDOW_Y_ADDRESS => self.window.y,
            LCD_WINDOW_X_ADDRESS => self.window.x,
            _ => unreachable!(),
        }
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
                self.dmg_palette.bg_palette = value;
                self.dmg_palette.update_palette(value, 0);
            }
            LCD_OBJ_PALETTE_0_ADDRESS => {
                self.dmg_palette.obj_palette[0] = value;
                self.dmg_palette.update_palette(value, 1);
            }
            LCD_OBJ_PALETTE_1_ADDRESS => {
                self.dmg_palette.obj_palette[1] = value;
                self.dmg_palette.update_palette(value, 2);
            }
            LCD_WINDOW_Y_ADDRESS => self.window.y = value,
            LCD_WINDOW_X_ADDRESS => self.window.x = value,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    pub fn increment_ly(&mut self, interrupts: &mut Interrupts) {
        if self.window.is_visible(self) && self.window.on(self) {
            self.window.line_number = self.window.line_number.wrapping_add(1);
        }

        self.ly = self.ly.wrapping_add(1);
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
            if self.status.is_stat_interrupt(LcdStatSrc::Lyc) {
                self.status.set_lyc(true);
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
    pub const fn get_ppu_mode(&self) -> PpuMode {
        PpuMode::from_u8(self.byte)
    }

    #[inline(always)]
    pub const fn set_ppu_mode(&mut self, mode: PpuMode) {
        self.byte &= !PPU_MODE_MASK;
        self.byte |= mode as u8;
    }

    #[inline(always)]
    pub const fn get_lyc(&self) -> bool {
        get_bit_flag(self.byte, 2)
    }

    #[inline(always)]
    pub const fn set_lyc(&mut self, b: bool) {
        set_bit(&mut self.byte, 2, b);
    }

    #[inline(always)]
    pub const fn is_stat_interrupt(&self, src: LcdStatSrc) -> bool {
        self.byte & (src as u8) != 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
#[repr(u8)]
pub enum PpuMode {
    #[default]
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

impl PpuMode {
    #[inline(always)]
    pub const fn from_u8(value: u8) -> Self {
        match value & PPU_MODE_MASK {
            0 => PpuMode::HBlank,
            1 => PpuMode::VBlank,
            2 => PpuMode::Oam,
            3 => PpuMode::Transfer,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DmgPalette {
    /// BGP register
    pub bg_palette: u8,
    /// OBP0 and OBP1 registers
    pub obj_palette: [u8; 2],
    pub bg_colors: [PixelColor; 4],
    pub sp1_colors: [PixelColor; 4],
    pub sp2_colors: [PixelColor; 4],
    pub current_colors: [PixelColor; 4],
}

impl Default for DmgPalette {
    fn default() -> Self {
        Self::new([
            PixelColor::from_hex_rgba("FFFFFFFF"),
            PixelColor::from_hex_rgba("AAAAAAFF"),
            PixelColor::from_hex_rgba("555555FF"),
            PixelColor::from_hex_rgba("000000FF"),
        ])
    }
}

impl DmgPalette {
    pub fn new(colors: [PixelColor; 4]) -> Self {
        Self {
            current_colors: colors,
            bg_colors: colors,
            sp1_colors: colors,
            sp2_colors: colors,
            bg_palette: 0xFC,
            obj_palette: [0xFF, 0xFF],
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
    fn get_obj_color(&self, is_second_palette: bool, color: usize) -> PixelColor {
        unsafe {
            if is_second_palette {
                *self.sp2_colors.get_unchecked(color)
            } else {
                *self.sp1_colors.get_unchecked(color)
            }
        }
    }

    #[inline(always)]
    fn get_gbw_color(&self, index: usize, enabled: bool) -> PixelColor {
        if enabled {
            // SAFETY: always index 0-3
            unsafe { *self.bg_colors.get_unchecked(index) }
        } else {
            // SAFETY: there is always 4 colors
            unsafe { *self.bg_colors.get_unchecked(0) }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CgbPalette {
    bg_ram: Box<[u8]>,
    obj_ram: Box<[u8]>,
    bg_index: u8,
    obj_index: u8,
}

impl Default for CgbPalette {
    fn default() -> Self {
        let mut obj = Self {
            bg_ram: vec![0; 64].into_boxed_slice(),
            obj_ram: vec![0; 64].into_boxed_slice(),
            bg_index: 0,
            obj_index: 0,
        };
        obj.boot_rom_init();

        obj
    }
}

impl CgbPalette {
    pub fn boot_rom_init(&mut self) {
        // Clear palette RAM
        self.bg_ram.fill(0);
        self.obj_ram.fill(0);

        // DMG grayscale palette (palette 0)
        let dmg_palette: [u16; 4] = [
            0x7FFF, // White
            0x5294, // Light gray
            0x294A, // Dark gray
            0x0000, // Black
        ];

        for (i, color) in dmg_palette.iter().enumerate() {
            let lo = (*color & 0xFF) as u8;
            let hi = (*color >> 8) as u8;

            // BG palette 0
            self.bg_ram[i * 2] = lo;
            self.bg_ram[i * 2 + 1] = hi;

            // OBJ palette 0
            self.obj_ram[i * 2] = lo;
            self.obj_ram[i * 2 + 1] = hi;
        }

        // BCPS / OCPS registers after boot
        self.bg_index = 0x00; // auto-increment disabled
        self.obj_index = 0x00; // auto-increment disabled
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            CGB_BG_PALLETE_INDEX_ADDR => {
                // Bit 6 always reads as 0
                self.bg_index & 0xBF
            }
            CGB_BG_PALLETE_DATA_ADDR => {
                let index = self.bg_index & 0x3F;
                self.bg_ram[index as usize]
            }
            CGB_OBJ_PALLETE_INDEX_ADDR => self.obj_index & 0xBF,
            CGB_OBJ_PALLETE_DATA_ADDR => {
                let index = self.obj_index & 0x3F;
                self.obj_ram[index as usize]
            }
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            CGB_BG_PALLETE_INDEX_ADDR => {
                // Bit 6 ignored
                self.bg_index = value & 0xBF;
            }
            CGB_BG_PALLETE_DATA_ADDR => {
                let index = self.bg_index & 0x3F;
                self.bg_ram[index as usize] = value;
                self.bg_index = Self::update_index(self.bg_index);
            }
            CGB_OBJ_PALLETE_INDEX_ADDR => {
                self.obj_index = value & 0xBF;
            }
            CGB_OBJ_PALLETE_DATA_ADDR => {
                let index = self.obj_index & 0x3F;
                self.obj_ram[index as usize] = value;
                self.obj_index = Self::update_index(self.obj_index);
            }

            _ => unreachable!(),
        }
    }

    fn get_color(&self, palette_number: u8, color_index: usize, is_obj: bool) -> PixelColor {
        let palette_number = palette_number as usize;
        // Each palette = 4 colors × 2 bytes
        let base = palette_number * 8 + color_index * 2;
        let ram = if is_obj { &self.obj_ram } else { &self.bg_ram };

        let lo = ram[base];
        let hi = ram[base + 1];
        let value = u16::from_le_bytes([lo, hi]);

        PixelColor::from_bgr555(value)
    }

    #[inline(always)]
    fn update_index(index: u8) -> u8 {
        const AUTO_INC_FLAG_MASK: u8 = 0x80;

        if index & AUTO_INC_FLAG_MASK != 0 {
            let new_index = ((index + 1) & 0x3F) | AUTO_INC_FLAG_MASK;
            return new_index;
        }

        index
    }
}
