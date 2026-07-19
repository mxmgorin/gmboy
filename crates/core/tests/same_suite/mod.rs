//! SameSuite (SameBoy) ROMs. These use the mooneye-style register signature, so
//! they run through the shared harness with `TestProtocol::Mooneye`.
//!
//! Only the ROMs the emulator currently passes are wired up here; add more as
//! they start passing so they are locked against regressions. The full suite
//! (mostly APU) can be surveyed with: `oxgbc-cli check roms/same-suite -r`.

use crate::get_roms_path;
use core::harness::{self, TestProtocol};
use std::time::Duration;

const TIMEOUT: Duration = Duration::from_secs(30);

fn run(rel: &str) -> Result<(), String> {
    let path = get_roms_path().join("same-suite").join(rel);

    harness::run_rom(&path, None, TestProtocol::Mooneye, TIMEOUT)?.into_result()
}

#[test]
fn ppu_blocking_bgpi_increase() {
    // Regression: during mode 3 the BCPS/OCPS index ports stay accessible and a
    // blocked BCPD/OCPD write still auto-increments the index.
    run("ppu/blocking_bgpi_increase.gb").unwrap();
}

#[test]
fn interrupt_ei_delay_halt() {
    // Regression: `EI; HALT` with interrupts already pending services them while
    // leaving PC on the HALT (return address = halt), re-running HALT until none
    // are pending; only a later interrupt that wakes the halt returns to halt+1.
    run("interrupt/ei_delay_halt.gb").unwrap();
}

#[test]
fn apu_channel_3_wave_ram_dac_on_rw() {
    run("apu/channel_3/channel_3_wave_ram_dac_on_rw.gb").unwrap();
}

#[test]
fn apu_div_write_trigger() {
    // The frame sequencer is clocked by the DIV-APU bit falling edge, so a
    // DIV write that resets the counter while the bit is set steps it early.
    run("apu/div_write_trigger.gb").unwrap();
}

#[test]
fn apu_div_write_trigger_volume() {
    run("apu/div_write_trigger_volume.gb").unwrap();
}

#[test]
fn apu_div_write_trigger_10() {
    // Power-on while the DIV-APU bit is set: the first event is swallowed and
    // the sequencer starts in the first half of a length period, so the NRx4
    // extra length clocking quirks apply to the very first trigger.
    run("apu/div_write_trigger_10.gb").unwrap();
}

#[test]
fn apu_div_trigger_volume_10() {
    run("apu/div_trigger_volume_10.gb").unwrap();
}

#[test]
fn apu_div_write_trigger_volume_10() {
    run("apu/div_write_trigger_volume_10.gb").unwrap();
}

#[test]
fn apu_channel_4_volume_div() {
    run("apu/channel_4/channel_4_volume_div.gb").unwrap();
}

// Square-channel trigger semantics: inactive channels freeze their frequency
// timer, a fresh activation is suppressed (digital 0) until its first duty
// step, and the trigger-to-step latency depends on channel state and the
// 1 MHz phase.

#[test]
fn apu_channel_1_delay() {
    run("apu/channel_1/channel_1_delay.gb").unwrap();
}

#[test]
fn apu_channel_1_freq_change() {
    run("apu/channel_1/channel_1_freq_change.gb").unwrap();
}

#[test]
fn apu_channel_1_restart() {
    run("apu/channel_1/channel_1_restart.gb").unwrap();
}

#[test]
fn apu_channel_1_restart_nrx2_glitch() {
    run("apu/channel_1/channel_1_restart_nrx2_glitch.gb").unwrap();
}

#[test]
fn apu_channel_1_stop_div() {
    run("apu/channel_1/channel_1_stop_div.gb").unwrap();
}

#[test]
fn apu_channel_1_stop_restart() {
    run("apu/channel_1/channel_1_stop_restart.gb").unwrap();
}

#[test]
fn apu_channel_1_volume_div() {
    run("apu/channel_1/channel_1_volume_div.gb").unwrap();
}

#[test]
fn apu_channel_2_delay() {
    run("apu/channel_2/channel_2_delay.gb").unwrap();
}

#[test]
fn apu_channel_2_freq_change() {
    run("apu/channel_2/channel_2_freq_change.gb").unwrap();
}

#[test]
fn apu_channel_2_restart() {
    run("apu/channel_2/channel_2_restart.gb").unwrap();
}

#[test]
fn apu_channel_2_restart_nrx2_glitch() {
    run("apu/channel_2/channel_2_restart_nrx2_glitch.gb").unwrap();
}

#[test]
fn apu_channel_2_stop_div() {
    run("apu/channel_2/channel_2_stop_div.gb").unwrap();
}

#[test]
fn apu_channel_2_stop_restart() {
    run("apu/channel_2/channel_2_stop_restart.gb").unwrap();
}

#[test]
fn apu_channel_2_volume() {
    run("apu/channel_2/channel_2_volume.gb").unwrap();
}

#[test]
fn apu_channel_2_volume_div() {
    run("apu/channel_2/channel_2_volume_div.gb").unwrap();
}

#[test]
fn dma_gbc_dma_cont() {
    run("dma/gbc_dma_cont.gb").unwrap();
}

#[test]
fn dma_gdma_addr_mask() {
    run("dma/gdma_addr_mask.gb").unwrap();
}
