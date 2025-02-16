use crate::mooneye::util::{assert_result, run_mooneye_rom, MooneyeRomCategory};
use std::time::Duration;

mod util;

const TIMEOUT: Duration = Duration::from_secs(10);

#[test]
fn test_oam_dma_basic() {
    let name = "basic";
    let category = MooneyeRomCategory::OamDma.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_oam_dma_reg_read() {
    let name = "reg_read";
    let category = MooneyeRomCategory::OamDma.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_oam_dma_sources_gs() {
    let name = "sources-GS";
    let category = MooneyeRomCategory::OamDma.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_bits_mem_oam() {
    let name = "mem_oam";
    let category = MooneyeRomCategory::Bits.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_bits_reg_f() {
    let name = "reg_f";
    let category = MooneyeRomCategory::Bits.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_bits_unused_hwio_gs() {
    let name = "unused_hwio-GS";
    let category = MooneyeRomCategory::Bits.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_instr_daa() {
    let name = "daa";
    let category = MooneyeRomCategory::Instr.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[ignore] // FIXME
#[test]
fn test_interrupts_ie_push() {
    let name = "ie_push";
    let category = MooneyeRomCategory::Interrupts.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_timer_div_write() {
    let name = "div_write";
    let category = MooneyeRomCategory::Timer.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[ignore] // FIXME
#[test]
fn test_timer_rapid_toggle() {
    let name = "rapid_toggle";
    let category = MooneyeRomCategory::Timer.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_timer_tim00() {
    let name = "tim00";
    let category = MooneyeRomCategory::Timer.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[ignore] // FIXME
#[test]
fn test_timer_tim00_div_trigger() {
    let name = "tim00_div_trigger";
    let category = MooneyeRomCategory::Timer.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_timer_tim01() {
    let name = "tim01";
    let category = MooneyeRomCategory::Timer.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[ignore] // FIXME
#[test]
fn test_timer_tim01_div_trigger() {
    let name = "tim01_div_trigger";
    let category = MooneyeRomCategory::Timer.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_timer_tim10() {
    let name = "tim10";
    let category = MooneyeRomCategory::Timer.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[ignore] // FIXME
#[test]
fn test_timer_tim10_div_trigger() {
    let name = "tim10_div_trigger";
    let category = MooneyeRomCategory::Timer.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_timer_tim11() {
    let name = "tim11";
    let category = MooneyeRomCategory::Timer.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[ignore] // FIXME
#[test]
fn test_timer_tim11_div_trigger() {
    let name = "tim11_div_trigger";
    let category = MooneyeRomCategory::Timer.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[ignore] // FIXME
#[test]
fn test_timer_tima_reload() {
    let name = "tima_reload";
    let category = MooneyeRomCategory::Timer.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[ignore] // FIXME: E: 7F!
#[test]
fn test_timer_tima_write_reloading() {
    let name = "tima_write_reloading";
    let category = MooneyeRomCategory::Timer.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_timer_tma_write_reloading() {
    let name = "tma_write_reloading";
    let category = MooneyeRomCategory::Timer.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}