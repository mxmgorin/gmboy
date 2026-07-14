use crate::get_roms_path;
use core::{
    auxiliary::joypad::JoypadButton,
    emu::config::GbModel,
    harness,
    ppu::{LCD_X_RES, LCD_Y_RES},
};
use std::{
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

const WIDTH: usize = LCD_X_RES as usize;
const HEIGHT: usize = LCD_Y_RES as usize;
const ROW_BYTES: usize = WIDTH * 3;

// rtc3test renders a live "Tick" duration measurement on these rows. The value
// varies run to run (real-time jitter), so we exclude them from the comparison;
// every other row matches the shipped reference screenshot pixel-for-pixel.
const TICK_ROWS: std::ops::Range<usize> = 24..31;

fn reference_path() -> PathBuf {
    get_roms_path()
        .join("rtc3test")
        .join("rtc3test-basic-tests-dmg.png")
}

/// Boot rtc3test, select "Basic tests" (the default cursor position — press A),
/// let it run for `secs` of real time, and return the RGB888 framebuffer.
fn run_basic_tests(secs: f64) -> Vec<u8> {
    let rom = get_roms_path().join("rtc3test").join("rtc3test.gb");
    let mut cpu = harness::build_cpu_from_path(&rom, Some(GbModel::Dmg)).unwrap();

    let start = Instant::now();
    let total = Duration::from_secs_f64(secs);
    let mut a_pressed = false;
    let mut a_released = false;

    loop {
        cpu.step();
        let elapsed = start.elapsed();

        // Tap A shortly after boot to run the highlighted "Basic tests" entry.
        if !a_pressed && elapsed > Duration::from_secs_f64(1.0) {
            cpu.clock.bus.io.joypad.handle(JoypadButton::A, true);
            a_pressed = true;
        }
        if !a_released && elapsed > Duration::from_secs_f64(1.2) {
            cpu.clock.bus.io.joypad.handle(JoypadButton::A, false);
            a_released = true;
        }

        if elapsed > total {
            return cpu.clock.bus.io.ppu.lcd.buffer.rgb888();
        }
    }
}

fn load_rgb(path: &Path) -> Vec<u8> {
    image::open(path).unwrap().to_rgb8().into_raw()
}

/// Runs rtc3test's "Basic tests" and asserts the result screen matches the
/// reference — i.e. every sub-test reports PASS (RTC enable/disable, register
/// writes, seconds increment, rollovers, overflow, overflow stickiness).
///
/// Ignored by default: the MBC3 RTC ticks on real wall-clock time, so the ROM
/// genuinely has to wait real seconds to observe ticks (~20s wall time). Run it
/// explicitly with:
///   cargo test -p core --test mod rtc -- --ignored
#[test]
#[ignore = "slow (~20s): the RTC ticks on real wall-clock time"]
fn rtc3test_basic_tests() {
    let got = run_basic_tests(22.0);
    let want = load_rgb(&reference_path());
    assert_eq!(got.len(), want.len(), "framebuffer size mismatch");

    let mismatched: Vec<usize> = (0..HEIGHT)
        .filter(|y| !TICK_ROWS.contains(y))
        .filter(|y| {
            let s = y * ROW_BYTES;
            got[s..s + ROW_BYTES] != want[s..s + ROW_BYTES]
        })
        .collect();

    assert!(
        mismatched.is_empty(),
        "basic-tests screen differs from the rtc3test reference on rows {mismatched:?} \
         (a PASS -> FAIL regression would show up here)"
    );
}
