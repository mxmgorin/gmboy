//! Authentic Game Boy Color boot-ROM colorization of monochrome (DMG) games.
//!
//! When a DMG cartridge boots on a Game Boy Color, the boot ROM sums the
//! cartridge title bytes, looks the checksum up in a table, and assigns one of
//! ~30 built-in palettes — split into background, OBJ0 and OBJ1 — so the game
//! shows in color instead of grayscale. This module reproduces that mapping.
//!
//! The tables (title checksums, 4th-letter disambiguation, palette combinations
//! and the raw palette colors) are ported from SameBoy's reimplemented boot ROM
//! (`BootROMs/cgb_boot.asm`), MIT licensed — compatible with this project's
//! GPL-3.0. See <https://github.com/LIJI32/SameBoy>.
//!
//! The result maps cleanly onto [`DmgPalette::set_palettes`]: BG becomes CGB BG
//! palette 0, OBP0 becomes OBJ palette 0, OBP1 becomes OBJ palette 1, and the
//! game's own BGP/OBP register writes still permute the four colors within each.

use crate::ppu::lcd::DmgPalette;
use crate::ppu::tile::PixelColor;

/// A resolved boot-ROM palette: three independent 4-color sub-palettes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DmgCompatPalette {
    pub bg: [PixelColor; 4],
    pub obj0: [PixelColor; 4],
    pub obj1: [PixelColor; 4],
}

impl DmgCompatPalette {
    /// Apply this palette to a [`DmgPalette`], preserving the current register
    /// mappings.
    #[inline]
    pub fn apply(&self, palette: &mut DmgPalette) {
        palette.set_palettes(self.bg, self.obj0, self.obj1);
    }
}

/// Resolve the CGB boot-ROM colorization for the given ROM image.
///
/// `rom` must be at least a full cartridge header (0x150 bytes). Games the boot
/// ROM does not recognize — and non-Nintendo titles — fall back to palette
/// combination 0, exactly as on real hardware.
pub fn dmg_compat_palette(rom: &[u8]) -> DmgCompatPalette {
    let combo = &PALETTE_COMBINATIONS[palette_combination_id(rom) as usize];
    DmgCompatPalette {
        obj0: read_palette(combo[0]),
        obj1: read_palette(combo[1]),
        bg: read_palette(combo[2]),
    }
}

/// Read the 4 colors of a raw palette at `byte_offset` into [`PALETTES`].
/// Offsets are color-granular (a multiple of 2 bytes), matching the boot ROM's
/// `palette_comb` (`* 8`) and `raw_palette_comb` (`* 2`) offset schemes.
#[inline]
fn read_palette(byte_offset: u8) -> [PixelColor; 4] {
    let start = (byte_offset / 2) as usize;
    [
        PixelColor::from_bgr555(PALETTES[start]),
        PixelColor::from_bgr555(PALETTES[start + 1]),
        PixelColor::from_bgr555(PALETTES[start + 2]),
        PixelColor::from_bgr555(PALETTES[start + 3]),
    ]
}

/// The `GetPaletteIndex` routine: title checksum → combination index.
fn palette_combination_id(rom: &[u8]) -> u8 {
    if !is_nintendo(rom) {
        return 0;
    }

    let checksum = title_checksum(rom);
    let fourth_letter = rom.get(0x0137).copied().unwrap_or(0);

    let mut i = 0;
    while i < TITLE_CHECKSUMS.len() {
        if TITLE_CHECKSUMS[i] == checksum {
            if i < FIRST_DUPLICATE_INDEX {
                return PALETTE_PER_CHECKSUM[i];
            }
            // Ambiguous checksum: disambiguate on the title's 4th letter.
            if fourth_letter == DUPLICATE_4TH_LETTERS[i - FIRST_DUPLICATE_INDEX] {
                return PALETTE_PER_CHECKSUM[i];
            }
            // Otherwise keep scanning for the next matching checksum.
        }
        i += 1;
    }

    0
}

/// Sum of the 16 title bytes (0x134..=0x143), as the boot ROM computes it.
fn title_checksum(rom: &[u8]) -> u8 {
    let mut sum = 0u8;
    for addr in 0x0134..0x0144 {
        sum = sum.wrapping_add(rom.get(addr).copied().unwrap_or(0));
    }
    sum
}

/// The boot ROM only colorizes first-party titles (licensee code 01 / Nintendo).
fn is_nintendo(rom: &[u8]) -> bool {
    match rom.get(0x014B).copied().unwrap_or(0) {
        // New-licensee marker: the real code lives in the header at 0x144-0x145.
        0x33 => {
            rom.get(0x0144).copied() == Some(b'0') && rom.get(0x0145).copied() == Some(b'1')
        }
        old => old == 0x01,
    }
}

/// Index of the first checksum that requires 4th-letter disambiguation.
const FIRST_DUPLICATE_INDEX: usize = 65;

/// Title checksums (0x00..). Indices >= [`FIRST_DUPLICATE_INDEX`] share values
/// with each other and are disambiguated via [`DUPLICATE_4TH_LETTERS`].
#[rustfmt::skip]
const TITLE_CHECKSUMS: [u8; 94] = [
    0x00, 0x88, 0x16, 0x36, 0xD1, 0xDB, 0xF2, 0x3C, 0x8C, 0x92, 0x3D, 0x5C, 0x58, 0xC9, 0x3E, 0x70,
    0x1D, 0x59, 0x69, 0x19, 0x35, 0xA8, 0x14, 0xAA, 0x75, 0x95, 0x99, 0x34, 0x6F, 0x15, 0xFF, 0x97,
    0x4B, 0x90, 0x17, 0x10, 0x39, 0xF7, 0xF6, 0xA2, 0x49, 0x4E, 0x43, 0x68, 0xE0, 0x8B, 0xF0, 0xCE,
    0x0C, 0x29, 0xE8, 0xB7, 0x86, 0x9A, 0x52, 0x01, 0x9D, 0x71, 0x9C, 0xBD, 0x5D, 0x6D, 0x67, 0x3F,
    0x6B,
    // From here checksums repeat; the 4th title letter decides the match.
    0xB3, 0x46, 0x28, 0xA5, 0xC6, 0xD3, 0x27, 0x61, 0x18, 0x66, 0x6A, 0xBF, 0x0D, 0xF4, 0xB3, 0x46,
    0x28, 0xA5, 0xC6, 0xD3, 0x27, 0x61, 0x18, 0x66, 0x6A, 0xBF, 0x0D, 0xF4, 0xB3,
];

/// 4th-letter values for the duplicate checksums (index i -> checksum 65 + i).
const DUPLICATE_4TH_LETTERS: &[u8; 29] = b"BEFAARBEKEK R-URAR INAILICE R";

/// Palette combination id per checksum. (The boot ROM's `$80` "use DMG boot
/// tilemap" flag is irrelevant to color and has been stripped.)
#[rustfmt::skip]
const PALETTE_PER_CHECKSUM: [u8; 94] = [
     0,  4,  5, 35, 34,  3, 31, 15, 10,  5, 19, 36,  7, 37, 30, 44,
    21, 32, 31, 20,  5, 33, 13, 14,  5, 29,  5, 18,  9,  3,  2, 26,
    25, 25, 41, 42, 26, 45, 42, 45, 36, 38, 26, 42, 30, 41, 34, 34,
     5, 42,  6,  5, 33, 25, 42, 42, 40,  2, 16, 25, 42, 42,  5,  0,
    39,
    36, 22, 25,  6, 32, 12, 36, 11, 39, 18, 39, 24, 31, 50, 17, 46,
     6, 27,  0, 47, 41, 41,  0,  0, 19, 34, 23, 18, 29,
];

/// Palette combinations: `[obj0, obj1, bg]` byte offsets into [`PALETTES`].
#[rustfmt::skip]
const PALETTE_COMBINATIONS: [[u8; 3]; 55] = [
    [ 32,  32, 232], [144, 144, 144], [160, 160, 160], [192, 192, 192], //  0- 3
    [ 72,  72,  72], [  0,   0,   0], [216, 216, 216], [ 40,  40,  40], //  4- 7
    [ 96,  96,  96], [208, 208, 208], [128,  64,  64], [ 32, 224, 224], //  8-11
    [ 32,  16,  16], [ 24,  32,  32], [ 32, 232, 232], [224,  32, 224], // 12-15
    [ 16, 136,  16], [128, 128,  64], [ 32,  32,  56], [ 32,  32, 144], // 16-19
    [ 32,  32, 160], [152, 152,  72], [ 30,  30,  88], [136, 136,  16], // 20-23
    [ 32,  32,  16], [ 32,  32,  24], [224, 224,   0], [ 24,  24,   0], // 24-27
    [  0,   0,   8], [144, 176, 144], [160, 176, 160], [192, 176, 192], // 28-31
    [128, 176,  64], [136,  32, 104], [222,   0, 112], [222,  32, 120], // 32-35
    [152, 182,  72], [128, 224,  80], [ 32, 184, 224], [136, 176,  16], // 36-39
    [ 32,   0,  16], [ 32, 224,  24], [224,  24,   0], [ 24, 224,  32], // 40-43
    [168, 224,  32], [ 24, 224,   0], [200,  24, 224], [  0, 224,  64], // 44-47
    [ 32,  24, 224], [224,  24,  48], [ 32, 224, 232], [240, 240, 240], // 48-51
    [248, 248, 248], [224,  32,   8], [  0,   0,  16],                  // 52-54
];

/// Raw palettes, 32 x 4 colors, in BGR555. Combination offsets index this as a
/// flat color array (offset / 2 = color index).
#[rustfmt::skip]
const PALETTES: [u16; 128] = [
    0x7FFF, 0x32BF, 0x00D0, 0x0000, //  0
    0x639F, 0x4279, 0x15B0, 0x04CB, //  1
    0x7FFF, 0x6E31, 0x454A, 0x0000, //  2
    0x7FFF, 0x1BEF, 0x0200, 0x0000, //  3
    0x7FFF, 0x421F, 0x1CF2, 0x0000, //  4
    0x7FFF, 0x5294, 0x294A, 0x0000, //  5
    0x7FFF, 0x03FF, 0x012F, 0x0000, //  6
    0x7FFF, 0x03EF, 0x01D6, 0x0000, //  7
    0x7FFF, 0x42B5, 0x3DC8, 0x0000, //  8
    0x7E74, 0x03FF, 0x0180, 0x0000, //  9
    0x67FF, 0x77AC, 0x1A13, 0x2D6B, // 10
    0x7ED6, 0x4BFF, 0x2175, 0x0000, // 11
    0x53FF, 0x4A5F, 0x7E52, 0x0000, // 12
    0x4FFF, 0x7ED2, 0x3A4C, 0x1CE0, // 13
    0x03ED, 0x7FFF, 0x255F, 0x0000, // 14
    0x036A, 0x021F, 0x03FF, 0x7FFF, // 15
    0x7FFF, 0x01DF, 0x0112, 0x0000, // 16
    0x231F, 0x035F, 0x00F2, 0x0009, // 17
    0x7FFF, 0x03EA, 0x011F, 0x0000, // 18
    0x299F, 0x001A, 0x000C, 0x0000, // 19
    0x7FFF, 0x027F, 0x001F, 0x0000, // 20
    0x7FFF, 0x03E0, 0x0206, 0x0120, // 21
    0x7FFF, 0x7EEB, 0x001F, 0x7C00, // 22
    0x7FFF, 0x3FFF, 0x7E00, 0x001F, // 23
    0x7FFF, 0x03FF, 0x001F, 0x0000, // 24
    0x03FF, 0x001F, 0x000C, 0x0000, // 25
    0x7FFF, 0x033F, 0x0193, 0x0000, // 26
    0x0000, 0x4200, 0x037F, 0x7FFF, // 27
    0x7FFF, 0x7E8C, 0x7C00, 0x0000, // 28
    0x7FFF, 0x1BEF, 0x6180, 0x0000, // 29
    0x7FFF, 0x7FEA, 0x7D5F, 0x0000, // 30 (SameBoy CGA)
    0x4778, 0x3290, 0x1D87, 0x0861, // 31 (SameBoy DMG LCD)
];

#[cfg(test)]
mod tests {
    use super::*;

    /// Minimal cartridge header: 16-byte title area + old licensee code.
    fn make_rom(title: &[u8], old_licensee: u8) -> Vec<u8> {
        let mut rom = vec![0u8; 0x150];
        for (i, &b) in title.iter().take(16).enumerate() {
            rom[0x0134 + i] = b;
        }
        rom[0x014B] = old_licensee;
        rom
    }

    fn bgr(v: u16) -> PixelColor {
        PixelColor::from_bgr555(v)
    }

    #[test]
    fn tetris_maps_to_combination_3() {
        // "TETRIS" checksum = 0xDB -> combination 3 -> palette 24 for all layers.
        let rom = make_rom(b"TETRIS", 0x01);
        let p = dmg_compat_palette(&rom);
        let pal24 = [bgr(0x7FFF), bgr(0x03FF), bgr(0x001F), bgr(0x0000)];
        assert_eq!(p.bg, pal24);
        assert_eq!(p.obj0, pal24);
        assert_eq!(p.obj1, pal24);
    }

    #[test]
    fn super_mario_land_uses_4th_letter_and_raw_offset() {
        // Checksum 0x46 is ambiguous; 4th letter 'E' selects combination 22,
        // whose obj offset 30 reads across a palette boundary (raw_palette_comb).
        let rom = make_rom(b"SUPER MARIOLAND", 0x01);
        let p = dmg_compat_palette(&rom);
        // Colors 15,16,17,18 in the flat palette array: palette 3's last color
        // (0x0000) followed by palette 4's first three — the deliberate
        // cross-boundary read of raw_palette_comb.
        let expected = [bgr(0x0000), bgr(0x7FFF), bgr(0x421F), bgr(0x1CF2)];
        assert_eq!(p.obj0, expected);
        assert_eq!(p.obj1, expected);
        // BG offset 88 -> palette 11.
        assert_eq!(p.bg, [bgr(0x7ED6), bgr(0x4BFF), bgr(0x2175), bgr(0x0000)]);
    }

    #[test]
    fn zelda_maps_to_combination_44() {
        // "ZELDA" checksum = 0x70 -> combination 44 -> obj0=pal21, obj1=pal28, bg=pal4.
        let rom = make_rom(b"ZELDA", 0x01);
        let p = dmg_compat_palette(&rom);
        assert_eq!(p.obj0, [bgr(0x7FFF), bgr(0x03E0), bgr(0x0206), bgr(0x0120)]);
        assert_eq!(p.obj1, [bgr(0x7FFF), bgr(0x7E8C), bgr(0x7C00), bgr(0x0000)]);
        assert_eq!(p.bg, [bgr(0x7FFF), bgr(0x421F), bgr(0x1CF2), bgr(0x0000)]);
    }

    #[test]
    fn non_nintendo_falls_back_to_combination_0() {
        // Even with a recognized title, a non-Nintendo licensee gets combo 0.
        let rom = make_rom(b"TETRIS", 0x00);
        let p = dmg_compat_palette(&rom);
        // combo 0 = [obj0=pal4, obj1=pal4, bg=pal29]
        assert_eq!(p.obj0, [bgr(0x7FFF), bgr(0x421F), bgr(0x1CF2), bgr(0x0000)]);
        assert_eq!(p.bg, [bgr(0x7FFF), bgr(0x1BEF), bgr(0x6180), bgr(0x0000)]);
    }

    #[test]
    fn unrecognized_nintendo_title_falls_back_to_combination_0() {
        // A Nintendo cart whose checksum (0x02) isn't in the table -> combo 0.
        let rom = make_rom(&[0x01, 0x01], 0x01);
        assert_eq!(title_checksum(&rom), 0x02);
        let p = dmg_compat_palette(&rom);
        assert_eq!(p.bg, [bgr(0x7FFF), bgr(0x1BEF), bgr(0x6180), bgr(0x0000)]);
    }

    /// Structural safety: every table entry stays in bounds.
    #[test]
    fn tables_are_self_consistent() {
        for &id in PALETTE_PER_CHECKSUM.iter() {
            assert!((id as usize) < PALETTE_COMBINATIONS.len(), "combo id {id} OOB");
        }
        for combo in PALETTE_COMBINATIONS.iter() {
            for &offset in combo.iter() {
                let end = (offset / 2) as usize + 4;
                assert!(end <= PALETTES.len(), "palette offset {offset} OOB");
            }
        }
        assert_eq!(DUPLICATE_4TH_LETTERS.len(), TITLE_CHECKSUMS.len() - FIRST_DUPLICATE_INDEX);
    }
}
