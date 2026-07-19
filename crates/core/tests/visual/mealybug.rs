//! Mealybug Tearoom Tests (mattcurrie) — mid-scanline PPU register change
//! tests, compared against hardware screenshots. DMG results are checked
//! against the DMG-blob captures, CGB results against CPU CGB C.
//!
//! The suites are `#[ignore]`d until the emulator passes them fully (like
//! gbmicrotest); run with `cargo test mealybug -- --ignored --nocapture`
//! to see the current per-ROM score.
//!
//! Score as of 2026-07-17: DMG 1/24, CGB 0/31. These require dot-exact
//! write-to-pixel latency for mid-scanline register changes (the fetcher
//! cadence is a few dots off hardware); CGB runs additionally differ in the
//! DMG-compat boot colorization.

use crate::{get_roms_path, get_tests_path, visual::util::run_visual_test_tolerance};
use core::emu::config::GbModel;
use std::{fs, path::PathBuf, time::Duration};

const DURATION: Duration = Duration::from_secs(1);
/// The reference PNGs use the canonical `(c << 3) | (c >> 2)` color expansion;
/// our RGB565 framebuffer rounds up to 3 per channel differently.
const TOLERANCE: u8 = 4;

fn expected_dir(model: &str) -> PathBuf {
    get_tests_path()
        .join("visual")
        .join("expected")
        .join("mealybug")
        .join(model)
}

/// Run every mealybug ROM that has a reference image for `model`, print a
/// per-ROM verdict and return (passed, total, failed names).
fn run_suite(model: GbModel, expected_subdir: &str) -> (usize, usize, Vec<String>) {
    let roms_dir = get_roms_path().join("mealybug");
    let expected = expected_dir(expected_subdir);

    let mut roms: Vec<PathBuf> = fs::read_dir(&roms_dir)
        .unwrap()
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|e| e == "gb"))
        .collect();
    roms.sort();

    let mut passed = 0;
    let mut total = 0;
    let mut failed = Vec::new();

    for rom in roms {
        let name = rom.file_stem().unwrap().to_string_lossy().to_string();
        let img = expected.join(format!("{name}.png"));

        // No capture for this ROM on this hardware revision.
        if !img.exists() {
            continue;
        }

        total += 1;
        let result =
            run_visual_test_tolerance(Some(model), &rom, &img, false, DURATION, TOLERANCE);

        match result {
            Ok(()) => {
                passed += 1;
                println!("PASS  {name}");
            }
            Err(err) => {
                println!("FAIL  {name}: {}", err.lines().next().unwrap_or(""));
                failed.push(name);
            }
        }
    }

    println!("mealybug {expected_subdir}: {passed}/{total}");

    (passed, total, failed)
}

#[ignore]
#[test]
fn test_mealybug_dmg() {
    let (passed, total, failed) = run_suite(GbModel::Dmg, "dmg");
    assert_eq!(passed, total, "failed: {failed:?}");
}

#[ignore]
#[test]
fn test_mealybug_cgb() {
    let (passed, total, failed) = run_suite(GbModel::Cgb, "cgb");
    assert_eq!(passed, total, "failed: {failed:?}");
}
