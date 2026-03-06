use core::{
    auxiliary::clock::Clock,
    bus::Bus,
    cart::Cart,
    cpu::Cpu,
    emu::config::GbModel,
    ppu::{LCD_X_RES, LCD_Y_RES},
};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

pub fn get_expected_path() -> PathBuf {
    crate::get_tests_path().join("visual").join("expected")
}

pub fn run_visual_test_dir(
    model: Option<GbModel>,
    path: &PathBuf,
    img_update: bool,
    stop_on_err: bool,
    duration: Duration,
) -> BTreeMap<PathBuf, Result<(), String>> {
    let dir = fs::read_dir(path).unwrap();
    let roms: Vec<_> = dir
        .filter_map(|dir| {
            if let Ok(entry) = dir {
                Some(entry.path())
            } else {
                None
            }
        })
        .collect();

    let mut results = BTreeMap::new();

    for rom_path in roms.into_iter() {
        let rom_name = rom_path.file_stem().unwrap();
        let img_path = get_expected_path().join(format!("{}.png", rom_name.display()));
        let result = run_visual_test(model, &rom_path, &img_path, img_update, duration);
        let is_err = result.is_err();
        results.insert(rom_path, result);

        if stop_on_err && is_err {
            return results;
        }
    }

    results
}

pub fn run_visual_test(
    model: Option<GbModel>,
    rom_path: &PathBuf,
    img_path: &PathBuf,
    img_update: bool,
    duration: Duration,
) -> Result<(), String> {
    let cart = Cart::new(core::read_bytes(rom_path.as_path())?)?;
    let bus = Bus::new(cart, Default::default(), model);
    let clock = Clock::new(bus);
    let mut cpu = Cpu::new(clock);
    let instant = Instant::now();

    loop {
        cpu.step();

        if instant.elapsed() > duration {
            let got_buffer = cpu.clock.bus.io.ppu.buffer.rgb888();

            if img_update {
                save_rgb888_image(img_path, &got_buffer, LCD_X_RES as u32, LCD_Y_RES as u32)?;
            }

            let want_buffer = load_image_rgb(&img_path)?;
            buffers_match(&got_buffer, &want_buffer)?;

            return Ok(());
        }
    }
}

fn load_image_rgb(path: &Path) -> Result<Vec<u8>, String> {
    let img = image::open(path).map_err(|e| e.to_string())?.to_rgb8();

    Ok(img.into_raw())
}

fn buffers_match(a: &[u8], b: &[u8]) -> Result<(), String> {
    if a.len() != b.len() {
        return Err(format!("Buffer size mismatch: {} vs {}", a.len(), b.len()));
    }

    for (i, (pa, pb)) in a.iter().zip(b.iter()).enumerate() {
        if pa != pb {
            return Err(format!("Pixel mismatch at byte {}: {} != {}", i, pa, pb));
        }
    }

    Ok(())
}

use image::ColorType;

pub fn save_rgb888_image(
    path: &Path,
    buffer: &[u8],
    width: u32,
    height: u32,
) -> Result<(), String> {
    image::save_buffer(path, buffer, width, height, ColorType::Rgb8).map_err(|e| e.to_string())
}
