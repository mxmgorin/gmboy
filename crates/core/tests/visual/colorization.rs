//! End-to-end check that a monochrome (DMG) game renders in color once the CGB
//! boot-ROM palette is applied — the path the app takes when "CGB" is selected
//! for a DMG-only cart.

use crate::get_roms_path;
use core::{
    auxiliary::clock::Clock,
    bus::Bus,
    cart::Cart,
    cpu::Cpu,
    emu::config::GbModel,
    ppu::cgb_boot_palette::{dmg_compat_palette, DmgCompatPalette},
};
use std::time::{Duration, Instant};

const DURATION: Duration = Duration::from_secs(1);

/// Run a DMG ROM to a stable frame and return its RGB888 framebuffer,
/// optionally applying a boot-ROM colorization palette first.
fn render_dmg(rom_bytes: &[u8], palette: Option<DmgCompatPalette>) -> Vec<u8> {
    let cart = Cart::new(rom_bytes.to_vec().into_boxed_slice()).unwrap();
    let bus = Bus::new(cart, Default::default(), Some(GbModel::Dmg));
    let clock = Clock::new(bus);
    let mut cpu = Cpu::new(clock);

    if let Some(palette) = palette {
        palette.apply(&mut cpu.clock.bus.io.ppu.lcd.dmg_palette);
    }

    let instant = Instant::now();
    loop {
        cpu.step();
        if instant.elapsed() > DURATION {
            return cpu.clock.bus.io.ppu.lcd.buffer.rgb888();
        }
    }
}

/// A pixel is "colored" when its channels spread wider than RGB565 rounding
/// noise (grayscale round-trips to a spread of at most ~3).
fn colored_pixel_count(rgb888: &[u8]) -> usize {
    rgb888
        .chunks_exact(3)
        .filter(|p| {
            let max = p.iter().copied().max().unwrap();
            let min = p.iter().copied().min().unwrap();
            max - min > 16
        })
        .count()
}

#[test]
fn dmg_game_is_colorized_by_boot_rom_palette() {
    let rom_path = get_roms_path().join("dmg-acid2.gb");
    let rom_bytes = core::read_bytes(&rom_path).unwrap();

    let palette = dmg_compat_palette(&rom_bytes);

    let gray = render_dmg(&rom_bytes, None);
    let color = render_dmg(&rom_bytes, Some(palette));

    // Colorization must actually change the rendered output.
    assert_eq!(gray.len(), color.len());
    assert_ne!(gray, color, "boot-ROM palette should change the rendered frame");

    // The grayscale render is (near-)monochrome; the colorized one is not.
    assert_eq!(
        colored_pixel_count(&gray),
        0,
        "default DMG render should be monochrome"
    );
    assert!(
        colored_pixel_count(&color) > 100,
        "colorized DMG-ACID2 should render many colored pixels, got {}",
        colored_pixel_count(&color)
    );
}
