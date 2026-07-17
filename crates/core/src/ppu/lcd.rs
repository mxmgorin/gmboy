use crate::emu::config::GbModel;
use crate::ppu::framebuffer::FrameBuffer;
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
    /// LY indicates the current horizontal line
    pub ly: u8,
    pub ly_compare: u8,
    pub dma_byte: u8,
    pub obj_priority_mode: u8,
    pub window: LcdWindow,
    pub dmg_palette: DmgPalette,
    pub cgb_palette: CgbPalette,
    pub model: GbModel,
    pub buffer: FrameBuffer,
    /// OAM read/write accessibility, maintained by the PPU at event dots.
    /// Reads are blocked from 4 dots before a visible line starts through the
    /// end of mode 3; writes stay open until mode 2 proper begins and reopen
    /// for the last 4 dots of mode 2 (mooneye lcdon_write_timing-GS).
    #[serde(default)]
    pub oam_read_blocked: bool,
    #[serde(default)]
    pub oam_write_blocked: bool,
    /// VRAM reads are blocked 4 dots before mode 3 begins; writes only from
    /// mode 3 itself (mooneye lcdon_timing-GS vs lcdon_write_timing-GS).
    #[serde(default)]
    pub vram_read_prelock: bool,
    /// CGB hardware running a DMG-only cart (compatibility mode): most
    /// CGB-only registers (KEY1, SVBK, HDMA, FF74, OPRI, palette data ports)
    /// read as $FF and ignore writes.
    #[serde(default)]
    pub dmg_compat: bool,
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
            buffer: FrameBuffer::default(),
            oam_read_blocked: false,
            oam_write_blocked: false,
            vram_read_prelock: false,
            dmg_compat: false,
        }
    }

    /// Full CGB register set available: CGB hardware running a CGB cart.
    #[inline(always)]
    pub fn is_cgb_mode(&self) -> bool {
        self.model == GbModel::Cgb && !self.dmg_compat
    }

    #[inline(always)]
    pub fn push_pixel(&mut self, pixel: PixelColor) {
        self.buffer.push(self.ly, pixel);
    }

    #[inline(always)]
    pub fn is_vram_blocked(&self) -> bool {
        self.status.get_ppu_mode() == PpuMode::Transfer && self.control.is_lcd_enabled()
    }

    #[inline(always)]
    pub fn is_vram_read_blocked(&self) -> bool {
        (self.vram_read_prelock || self.status.get_ppu_mode() == PpuMode::Transfer)
            && self.control.is_lcd_enabled()
    }

    #[inline(always)]
    pub fn is_oam_read_blocked(&self) -> bool {
        self.oam_read_blocked && self.control.is_lcd_enabled()
    }

    #[inline(always)]
    pub fn is_oam_write_blocked(&self) -> bool {
        self.oam_write_blocked && self.control.is_lcd_enabled()
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

    pub fn get_bgw_color(&self, color_id: usize, enabled: bool, flags: TileFlags) -> PixelColor {
        match self.model {
            GbModel::Cgb => self
                .cgb_palette
                .get_color(flags.read_cgb_palette(), color_id, false),
            GbModel::Dmg => self.dmg_palette.get_gbw_color(color_id, enabled),
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
                    // LCD off: mode bits read 0, but the interrupt-enable bits
                    // and the (frozen) LYC coincidence flag stay readable.
                    self.status.byte & !PPU_MODE_MASK
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
            LCD_STATUS_ADDRESS => self.status.write(value),
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
    pub fn increment_ly(&mut self) {
        if self.window.is_visible(self) && self.window.on(self) {
            self.window.line_number = self.window.line_number.wrapping_add(1);
        }

        self.ly = self.ly.wrapping_add(1);
        self.update_lyc_flag();
    }

    #[inline(always)]
    pub fn reset_ly(&mut self) {
        self.ly = 0;
        self.window.line_number = 0;
        self.update_lyc_flag();
    }

    /// Recompute the LY=LYC coincidence flag. The flag is set whenever the
    /// values match, independently of the LYC interrupt-enable bit; the STAT
    /// interrupt itself is edge-triggered off the composite STAT line in `Ppu`.
    #[inline(always)]
    pub fn update_lyc_flag(&mut self) {
        self.status.set_lyc(self.ly == self.ly_compare);
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
pub const LYC_MASK: u8 = 0b100;

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[repr(C)]
pub struct LcdStatus {
    byte: u8,
}

impl LcdStatus {
    #[inline(always)]
    pub fn read(&self) -> u8 {
        self.byte
    }

    #[inline(always)]
    pub fn write(&mut self, value: u8) {
        const MASK: u8 = PPU_MODE_MASK | LYC_MASK;
        // clear lyc and mode bits in new value (they are read-only)
        // get lyc and mode from the current value and combine
        self.byte = (self.byte & MASK) | (value & !MASK);
    }

    #[inline(always)]
    pub const fn get_ppu_mode(&self) -> PpuMode {
        PpuMode::from_u8(self.byte)
    }

    #[inline(always)]
    pub const fn set_ppu_mode(&mut self, mode: PpuMode) {
        self.byte &= !PPU_MODE_MASK; // clear ppu mode bits
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
    /// Base color LUTs (indexed by color id 0-3), one per hardware palette.
    /// Kept independent so a colorizer can tint BG / OBJ0 / OBJ1 differently —
    /// as the CGB boot ROM does when it colorizes a DMG game.
    bg_base: [PixelColor; 4],
    obj_base: [[PixelColor; 4]; 2],
    /// Colors actually drawn, after applying each register's index permutation.
    pub bg_colors: [PixelColor; 4],
    pub sp1_colors: [PixelColor; 4],
    pub sp2_colors: [PixelColor; 4],
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
            bg_palette: 0xFC,
            obj_palette: [0xFF, 0xFF],
            bg_base: colors,
            obj_base: [colors, colors],
            bg_colors: colors,
            sp1_colors: colors,
            sp2_colors: colors,
        }
    }

    #[inline(always)]
    fn update_palette(&mut self, palette_data: u8, pallet_type: u8) {
        let base = match pallet_type {
            1 => &self.obj_base[0],
            2 => &self.obj_base[1],
            _ => &self.bg_base,
        };

        // Resolve into a local first so the immutable borrow of `base` ends
        // before we take the mutable borrow of the destination array.
        let resolved = [
            base[(palette_data & 0b11) as usize],
            base[((palette_data >> 2) & 0b11) as usize],
            base[((palette_data >> 4) & 0b11) as usize],
            base[((palette_data >> 6) & 0b11) as usize],
        ];

        let dst = match pallet_type {
            1 => &mut self.sp1_colors,
            2 => &mut self.sp2_colors,
            _ => &mut self.bg_colors,
        };
        *dst = resolved;
    }

    /// Set a single 4-color palette shared by BG, OBJ0 and OBJ1 (the classic
    /// monochrome recolor).
    #[inline(always)]
    pub fn set_colors(&mut self, colors: [PixelColor; 4]) {
        self.set_palettes(colors, colors, colors);
    }

    /// Assign independent base palettes to BG, OBJ0 and OBJ1. Reproduces the CGB
    /// boot ROM's colorization of DMG games, where the background and the two
    /// sprite layers can each get a different set of colors. The game's
    /// BGP/OBP register mappings are re-applied on top of the new bases.
    pub fn set_palettes(
        &mut self,
        bg: [PixelColor; 4],
        obj0: [PixelColor; 4],
        obj1: [PixelColor; 4],
    ) {
        self.bg_base = bg;
        self.obj_base = [obj0, obj1];
        self.update_palette(self.bg_palette, 0);
        self.update_palette(self.obj_palette[0], 1);
        self.update_palette(self.obj_palette[1], 2);
    }

    /// The BG base palette — used to carry the active colors across a cart reload.
    #[inline(always)]
    pub fn base_colors(&self) -> [PixelColor; 4] {
        self.bg_base
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
    fn get_gbw_color(&self, id: usize, enabled: bool) -> PixelColor {
        if enabled {
            // SAFETY: always index 0-3
            unsafe { *self.bg_colors.get_unchecked(id) }
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

        // BCPS / OCPS after boot: the boot ROM leaves auto-increment on with
        // the index past its last palette write (mooneye boot_hwio-C).
        self.bg_index = 0x88;
        self.obj_index = 0x90;
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            CGB_BG_PALLETE_INDEX_ADDR => {
                // Bit 6 is unused and always reads back as 1 on CGB.
                self.bg_index | 0x40
            }
            CGB_BG_PALLETE_DATA_ADDR => {
                let index = self.bg_index & 0x3F;
                self.bg_ram[index as usize]
            }
            CGB_OBJ_PALLETE_INDEX_ADDR => self.obj_index | 0x40,
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

    /// A write to a data port (BCPD/OCPD) that is blocked because the PPU is in
    /// mode 3: the data byte is dropped, but the auto-increment index still
    /// advances — matches CGB hardware (see same-suite `blocking_bgpi_increase`).
    pub fn tick_index_on_blocked_write(&mut self, addr: u16) {
        match addr {
            CGB_BG_PALLETE_DATA_ADDR => self.bg_index = Self::update_index(self.bg_index),
            CGB_OBJ_PALLETE_DATA_ADDR => self.obj_index = Self::update_index(self.obj_index),
            _ => {}
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

#[cfg(test)]
mod dmg_palette_tests {
    use super::*;

    // Distinct, easily identifiable base palettes.
    fn ramp(base: u8) -> [PixelColor; 4] {
        [
            PixelColor::new(base, 0, 0),
            PixelColor::new(0, base, 0),
            PixelColor::new(0, 0, base),
            PixelColor::new(base, base, base),
        ]
    }

    /// Identity BGP/OBP mapping (id0->0, id1->1, id2->2, id3->3).
    fn set_identity_regs(p: &mut DmgPalette) {
        p.bg_palette = 0xE4;
        p.obj_palette = [0xE4, 0xE4];
    }

    #[test]
    fn set_colors_keeps_all_palettes_identical() {
        let colors = ramp(0xFF);
        let mut p = DmgPalette::new(ramp(0x11));
        set_identity_regs(&mut p);
        p.set_colors(colors);

        for id in 0..4 {
            assert_eq!(p.get_gbw_color(id, true), colors[id]);
            assert_eq!(p.get_obj_color(false, id), colors[id]); // OBJ0
            assert_eq!(p.get_obj_color(true, id), colors[id]); // OBJ1
        }
    }

    #[test]
    fn set_palettes_are_independent() {
        let bg = ramp(0x10);
        let obj0 = ramp(0x20);
        let obj1 = ramp(0x30);

        let mut p = DmgPalette::new(ramp(0x00));
        set_identity_regs(&mut p);
        p.set_palettes(bg, obj0, obj1);

        for id in 0..4 {
            assert_eq!(p.get_gbw_color(id, true), bg[id]);
            assert_eq!(p.get_obj_color(false, id), obj0[id]);
            assert_eq!(p.get_obj_color(true, id), obj1[id]);
        }
    }

    #[test]
    fn register_permutes_within_its_own_base() {
        let bg = ramp(0x10);
        let obj0 = ramp(0x20);
        let obj1 = ramp(0x30);

        let mut p = DmgPalette::new(ramp(0x00));
        set_identity_regs(&mut p);
        p.set_palettes(bg, obj0, obj1);

        // Reverse the BG mapping: 0x1B = 0b00_01_10_11 -> id0->3, id1->2, id2->1, id3->0.
        p.update_palette(0x1B, 0);

        assert_eq!(p.get_gbw_color(0, true), bg[3]);
        assert_eq!(p.get_gbw_color(1, true), bg[2]);
        assert_eq!(p.get_gbw_color(2, true), bg[1]);
        assert_eq!(p.get_gbw_color(3, true), bg[0]);

        // Sprite palettes are untouched by a BG register write.
        for id in 0..4 {
            assert_eq!(p.get_obj_color(false, id), obj0[id]);
            assert_eq!(p.get_obj_color(true, id), obj1[id]);
        }
    }

    #[test]
    fn base_colors_returns_bg_base() {
        let bg = ramp(0x40);
        let mut p = DmgPalette::new(ramp(0x00));
        p.set_palettes(bg, ramp(0x50), ramp(0x60));
        assert_eq!(p.base_colors(), bg);
    }
}
