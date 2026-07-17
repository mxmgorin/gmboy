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
fn dma_gbc_dma_cont() {
    run("dma/gbc_dma_cont.gb").unwrap();
}

#[test]
fn dma_gdma_addr_mask() {
    run("dma/gdma_addr_mask.gb").unwrap();
}
