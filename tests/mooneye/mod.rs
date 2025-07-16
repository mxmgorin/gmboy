use std::path::{PathBuf};
use crate::mooneye::util::{assert_result, assert_result_path, run_mooneye_rom, run_mooneye_rom_path, MooneyeRomCategory};
use std::time::Duration;

mod util;

const TIMEOUT: Duration = Duration::from_secs(5);

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
fn test_oam_dma_restart() {
    let name = "oam_dma_restart";
    let category = MooneyeRomCategory::OamDma.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_oam_dma_start() {
    let name = "oam_dma_start";
    let category = MooneyeRomCategory::OamDma.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_oam_dma_timing() {
    let name = "oam_dma_timing";
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

#[ignore] // FIXME
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

#[test]
fn test_timer_tim00() {
    let name = "tim00";
    let category = MooneyeRomCategory::Timer.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

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

#[test]
fn test_timer_tim11_div_trigger() {
    let name = "tim11_div_trigger";
    let category = MooneyeRomCategory::Timer.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_timer_tima_reload() {
    let name = "tima_reload";
    let category = MooneyeRomCategory::Timer.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_timer_rapid_toggle() {
    let name = "rapid_toggle";
    let category = MooneyeRomCategory::Timer.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

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

#[test]
fn test_add_sp_e_timing() {
    let name = "add_sp_e_timing";
    let category = MooneyeRomCategory::Timing.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_call_cc_timing() {
    let name = "call_cc_timing";
    let category = MooneyeRomCategory::Timing.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_call_cc_timing2() {
    let name = "call_cc_timing2";
    let category = MooneyeRomCategory::Timing.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_call_timing() {
    let name = "call_timing";
    let category = MooneyeRomCategory::Timing.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_call_timing2() {
    let name = "call_timing2";
    let category = MooneyeRomCategory::Timing.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[ignore] // FIXME
#[test]
fn test_di_timing_gs() {
    let name = "di_timing-GS";
    let category = MooneyeRomCategory::Timing.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_div_timing() {
    let name = "div_timing";
    let category = MooneyeRomCategory::Timing.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_ei_timing() {
    let name = "ei_timing";
    let category = MooneyeRomCategory::Timing.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_halt_ime0_ei() {
    let name = "halt_ime0_ei";
    let category = MooneyeRomCategory::Timing.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_halt_ime0_nointr_timing() {
    let name = "halt_ime0_nointr_timing";
    let category = MooneyeRomCategory::Timing.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_halt_ime1_timing() {
    let name = "halt_ime1_timing";
    let category = MooneyeRomCategory::Timing.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_halt_ime1_timing2_gs() {
    let name = "halt_ime1_timing2-GS";
    let category = MooneyeRomCategory::Timing.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[ignore] // FIXME
#[test]
fn test_intr_timing() {
    let name = "intr_timing";
    let category = MooneyeRomCategory::Timing.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_jp_cc_timing() {
    let name = "jp_cc_timing";
    let category = MooneyeRomCategory::Timing.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_jp_timing() {
    let name = "jp_timing";
    let category = MooneyeRomCategory::Timing.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_ld_hl_sp_e_timing() {
    let name = "ld_hl_sp_e_timing";
    let category = MooneyeRomCategory::Timing.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_pop_timing() {
    let name = "pop_timing";
    let category = MooneyeRomCategory::Timing.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_push_timing() {
    let name = "push_timing";
    let category = MooneyeRomCategory::Timing.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_ret_cc_timing() {
    let name = "ret_cc_timing";
    let category = MooneyeRomCategory::Timing.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_ret_timing() {
    let name = "ret_timing";
    let category = MooneyeRomCategory::Timing.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_reti_intr_timing() {
    let name = "reti_intr_timing";
    let category = MooneyeRomCategory::Timing.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_reti_timing() {
    let name = "reti_timing";
    let category = MooneyeRomCategory::Timing.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_rst_timing() {
    let name = "rst_timing";
    let category = MooneyeRomCategory::Timing.into();
    let result = run_mooneye_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[ignore]
#[test]
fn test_mbc2_bits_ramg() {
    let path = PathBuf::from("tests/mooneye/emulator-only/mbc2/bits_ramg.gb");
    let result = run_mooneye_rom_path(path.clone(), TIMEOUT);

    assert_result_path(path, result);
}

#[test]
fn test_mbc2_bits_romb() {
    let path = PathBuf::from("tests/mooneye/emulator-only/mbc2/bits_romb.gb");
    let result = run_mooneye_rom_path(path.clone(), TIMEOUT);

    assert_result_path(path, result);
}

#[test]
fn test_mbc2_bits_unused() {
    let path = PathBuf::from("tests/mooneye/emulator-only/mbc2/bits_unused.gb");
    let result = run_mooneye_rom_path(path.clone(), TIMEOUT);

    assert_result_path(path, result);
}

#[ignore]
#[test]
fn test_mbc2_ram() {
    let path = PathBuf::from("tests/mooneye/emulator-only/mbc2/ram.gb");
    let result = run_mooneye_rom_path(path.clone(), TIMEOUT);

    assert_result_path(path, result);
}

#[ignore]
#[test]
fn test_mbc2_rom_1mb() {
    let path = PathBuf::from("tests/mooneye/emulator-only/mbc2/rom_1Mb.gb");
    let result = run_mooneye_rom_path(path.clone(), TIMEOUT);

    assert_result_path(path, result);
}

#[ignore]
#[test]
fn test_mbc2_rom_2mb() {
    let path = PathBuf::from("tests/mooneye/emulator-only/mbc2/rom_2Mb.gb");
    let result = run_mooneye_rom_path(path.clone(), TIMEOUT);

    assert_result_path(path, result);
}

#[ignore]
#[test]
fn test_mbc2_rom_512kb() {
    let path = PathBuf::from("tests/mooneye/emulator-only/mbc2/rom_512kb.gb");
    let result = run_mooneye_rom_path(path.clone(), TIMEOUT);

    assert_result_path(path, result);
}
