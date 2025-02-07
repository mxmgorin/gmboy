use crate::blargg::util::{run_test_rom, TestRomCategory};
use std::time::Duration;

mod util;

#[test]
fn test_blargg_1() {
    let rom = "01-special.gb";
    let category = Some(TestRomCategory::CpuInstructions);
    let timeout = Duration::from_secs(30);

    run_test_rom(rom, category, timeout);
}

#[test]
fn test_blargg_6() {
    let rom = "06-ld r,r.gb";
    let category = Some(TestRomCategory::CpuInstructions);
    let timeout = Duration::from_secs(30);

    run_test_rom(rom, category, timeout);
}
