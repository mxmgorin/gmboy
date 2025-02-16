use crate::mooneye::util::{assert_result, run_test_rom, MooneyeRomCategory};
use std::time::Duration;

mod util;

const TIMEOUT: Duration = Duration::from_secs(15);

#[test]
fn test_oam_dma_basic() {
    let name = "basic";
    let category = MooneyeRomCategory::OamDma.into();
    let result = run_test_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_oam_dma_reg_read() {
    let name = "reg_read";
    let category = MooneyeRomCategory::OamDma.into();
    let result = run_test_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_oam_dma_sources_gs() {
    let name = "sources-GS";
    let category = MooneyeRomCategory::OamDma.into();
    let result = run_test_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_bits_mem_oam() {
    let name = "mem_oam";
    let category = MooneyeRomCategory::Bits.into();
    let result = run_test_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_bits_reg_f() {
    let name = "reg_f";
    let category = MooneyeRomCategory::Bits.into();
    let result = run_test_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_bits_unused_hwio_gs() {
    let name = "unused_hwio-GS";
    let category = MooneyeRomCategory::Bits.into();
    let result = run_test_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[test]
fn test_instr_daa() {
    let name = "daa";
    let category = MooneyeRomCategory::Instr.into();
    let result = run_test_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}

#[ignore] // FIXME
#[test]
fn test_interrupts_ie_push() {
    let name = "ie_push";
    let category = MooneyeRomCategory::Interrupts.into();
    let result = run_test_rom(name, category, TIMEOUT);

    assert_result(name, category, result);
}