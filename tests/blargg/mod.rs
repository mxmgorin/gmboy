use crate::blargg::util::{assert_result, run_test_rom, TestRomCategory};
use std::time::Duration;

mod util;

const TIMEOUT: Duration = Duration::from_secs(30);

#[test]
fn test_mem_read_timing() {
    let name = "01-read_timing";
    let category = Some(TestRomCategory::MemTiming);

    assert_result(name, category, run_test_rom(name, category, TIMEOUT));
}

#[test]
fn test_mem_write_timing() {
    let name = "02-write_timing";
    let category = Some(TestRomCategory::MemTiming);

    assert_result(name, category, run_test_rom(name, category, TIMEOUT));
}

#[ignore]
#[test]
fn test_mem_modify_timing() {
    let name = "03-modify_timing";
    let category = Some(TestRomCategory::MemTiming);

    // FIXME
    assert_result(name, category, run_test_rom(name, category, TIMEOUT));
}

#[ignore]
#[test]
fn test_interrupt_time() {
    let name = "interrupt_time";
    let category = None;

    assert_result(
        name,
        category,
        run_test_rom(name, category, Duration::from_secs(180)),
    );
}

#[ignore]
#[test]
fn test_instr_timing() {
    let name = "instr_timing";
    let category = None;

    assert_result(
        name,
        category,
        run_test_rom(name, category, Duration::from_secs(180)),
    );
}

#[test]
fn test_cpu_instructs_1() {
    let name = "01-special";
    let category = Some(TestRomCategory::CpuInstructions);

    assert_result(name, category, run_test_rom(name, category, TIMEOUT));
}

#[test]
fn test_cpu_instructs_2() {
    let name = "02-interrupts";
    let category = Some(TestRomCategory::CpuInstructions);

    assert_result(name, category, run_test_rom(name, category, TIMEOUT));
}

#[test]
fn test_cpu_instructs_3() {
    let name = "03-op sp,hl";
    let category = Some(TestRomCategory::CpuInstructions);

    assert_result(name, category, run_test_rom(name, category, TIMEOUT));
}

#[test]
fn test_cpu_instructs_4() {
    let name = "04-op r,imm";
    let category = Some(TestRomCategory::CpuInstructions);

    assert_result(name, category, run_test_rom(name, category, TIMEOUT));
}

#[test]
fn test_cpu_instructs_5() {
    let name = "05-op rp";
    let category = Some(TestRomCategory::CpuInstructions);

    assert_result(name, category, run_test_rom(name, category, TIMEOUT));
}

#[test]
fn test_cpu_instructs_6() {
    let name = "06-ld r,r";
    let category = Some(TestRomCategory::CpuInstructions);

    assert_result(name, category, run_test_rom(name, category, TIMEOUT));
}

#[test]
fn test_cpu_instructs_7() {
    let name = "07-jr,jp,call,ret,rst";
    let category = Some(TestRomCategory::CpuInstructions);

    assert_result(name, category, run_test_rom(name, category, TIMEOUT));
}

#[test]
fn test_cpu_instructs_8() {
    let name = "08-misc instrs";
    let category = Some(TestRomCategory::CpuInstructions);

    assert_result(name, category, run_test_rom(name, category, TIMEOUT));
}

#[test]
fn test_cpu_instructs_9() {
    let name = "09-op r,r";
    let category = Some(TestRomCategory::CpuInstructions);

    assert_result(name, category, run_test_rom(name, category, TIMEOUT));
}

#[test]
fn test_cpu_instructs_10() {
    let name = "10-bit ops";
    let category = Some(TestRomCategory::CpuInstructions);

    assert_result(name, category, run_test_rom(name, category, TIMEOUT));
}

#[test]
fn test_cpu_instructs_11() {
    let name = "11-op a,(hl)";
    let category = Some(TestRomCategory::CpuInstructions);

    assert_result(name, category, run_test_rom(name, category, TIMEOUT));
}
