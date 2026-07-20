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

// Noise: a free-running 14-bit counter clocks the LFSR on rising edges of the
// NR43-selected bit; it keeps counting in the background while the channel is
// inactive, and trigger/NR43-write countdown reseeds depend on the 2 MHz
// alignment (SameBoy's model).

// Wave: the output byte is latched at each sample step (a trigger restarts
// the position but keeps the stale byte until the first step, 4 T late), the
// NR32 level applies immediately, and CPU wave-RAM access while the channel
// plays hits the currently-addressed byte (CGB).

#[test]
fn apu_channel_3_and_glitch() {
    run("apu/channel_3/channel_3_and_glitch.gb").unwrap();
}

#[test]
fn apu_channel_3_delay() {
    run("apu/channel_3/channel_3_delay.gb").unwrap();
}

#[test]
fn apu_channel_3_first_sample() {
    run("apu/channel_3/channel_3_first_sample.gb").unwrap();
}

#[test]
fn apu_channel_3_freq_change_delay() {
    run("apu/channel_3/channel_3_freq_change_delay.gb").unwrap();
}

#[test]
fn apu_channel_3_restart_delay() {
    run("apu/channel_3/channel_3_restart_delay.gb").unwrap();
}

#[test]
fn apu_channel_3_restart_during_delay() {
    run("apu/channel_3/channel_3_restart_during_delay.gb").unwrap();
}

#[test]
fn apu_channel_3_restart_stop_delay() {
    run("apu/channel_3/channel_3_restart_stop_delay.gb").unwrap();
}

#[test]
fn apu_channel_3_shift_delay() {
    run("apu/channel_3/channel_3_shift_delay.gb").unwrap();
}

#[test]
fn apu_channel_3_shift_skip_delay() {
    run("apu/channel_3/channel_3_shift_skip_delay.gb").unwrap();
}

#[test]
fn apu_channel_3_stop_div() {
    run("apu/channel_3/channel_3_stop_div.gb").unwrap();
}

#[test]
fn apu_channel_3_wave_ram_locked_write() {
    run("apu/channel_3/channel_3_wave_ram_locked_write.gb").unwrap();
}

#[test]
fn apu_channel_3_wave_ram_sync() {
    run("apu/channel_3/channel_3_wave_ram_sync.gb").unwrap();
}

#[test]
fn apu_channel_4_align() {
    run("apu/channel_4/channel_4_align.gb").unwrap();
}

#[test]
fn apu_channel_4_delay() {
    run("apu/channel_4/channel_4_delay.gb").unwrap();
}

#[test]
fn apu_channel_4_equivalent_frequencies() {
    run("apu/channel_4/channel_4_equivalent_frequencies.gb").unwrap();
}

#[test]
fn apu_channel_4_freq_change() {
    run("apu/channel_4/channel_4_freq_change.gb").unwrap();
}

#[test]
fn apu_channel_4_frequency_alignment() {
    run("apu/channel_4/channel_4_frequency_alignment.gb").unwrap();
}

#[test]
fn apu_channel_4_lfsr() {
    run("apu/channel_4/channel_4_lfsr.gb").unwrap();
}

#[test]
fn apu_channel_4_lfsr15() {
    run("apu/channel_4/channel_4_lfsr15.gb").unwrap();
}

#[test]
fn apu_channel_4_lfsr_15_7() {
    run("apu/channel_4/channel_4_lfsr_15_7.gb").unwrap();
}

#[test]
fn apu_channel_4_lfsr_7_15() {
    run("apu/channel_4/channel_4_lfsr_7_15.gb").unwrap();
}

#[test]
fn apu_channel_4_lfsr_restart() {
    run("apu/channel_4/channel_4_lfsr_restart.gb").unwrap();
}

#[test]
fn apu_channel_4_lfsr_restart_fast() {
    run("apu/channel_4/channel_4_lfsr_restart_fast.gb").unwrap();
}

// Square-channel trigger semantics: inactive channels freeze their frequency
// timer, a fresh activation is suppressed (digital 0) until its first duty
// step, and the trigger-to-step latency depends on channel state and the
// 1 MHz phase.

#[test]
fn apu_channel_1_align() {
    // Runs in double speed: PPU/APU tick on every other CPU T-cycle,
    // phase-continuous, so a 1 M-cycle shift flips the 1 MHz phase the
    // trigger delay depends on.
    run("apu/channel_1/channel_1_align.gb").unwrap();
}

#[test]
fn apu_channel_2_align() {
    run("apu/channel_2/channel_2_align.gb").unwrap();
}

#[test]
fn apu_channel_1_align_cpu() {
    run("apu/channel_1/channel_1_align_cpu.gb").unwrap();
}

#[test]
fn apu_channel_2_align_cpu() {
    run("apu/channel_2/channel_2_align_cpu.gb").unwrap();
}

#[test]
fn apu_channel_1_duty() {
    run("apu/channel_1/channel_1_duty.gb").unwrap();
}

#[test]
fn apu_channel_2_duty() {
    run("apu/channel_2/channel_2_duty.gb").unwrap();
}

#[test]
fn apu_channel_1_duty_delay() {
    // An NRx1 duty change becomes effective only after the current duty step
    // finishes (the output sample is latched per step).
    run("apu/channel_1/channel_1_duty_delay.gb").unwrap();
}

#[test]
fn apu_channel_2_duty_delay() {
    run("apu/channel_2/channel_2_duty_delay.gb").unwrap();
}

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

// Envelope pipeline (SameBoy model): 64 Hz countdown -> clock armed at the
// DIV-APU rising edge -> volume step on the next event; NRx2 writes while
// active go through the zombie-mode glitch, and the DAC stays on for 0x08
// (direction bit only).

#[test]
fn apu_channel_1_volume() {
    run("apu/channel_1/channel_1_volume.gb").unwrap();
}

#[test]
fn apu_channel_1_nrx2_glitch() {
    run("apu/channel_1/channel_1_nrx2_glitch.gb").unwrap();
}

#[test]
fn apu_channel_2_nrx2_glitch() {
    run("apu/channel_2/channel_2_nrx2_glitch.gb").unwrap();
}

#[test]
fn apu_channel_1_nrx2_speed_change() {
    run("apu/channel_1/channel_1_nrx2_speed_change.gb").unwrap();
}

#[test]
fn apu_channel_2_nrx2_speed_change() {
    run("apu/channel_2/channel_2_nrx2_speed_change.gb").unwrap();
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

#[test]
fn dma_hdma_lcd_off() {
    // HDMA5's length field is stored on every write (a pausing write reads
    // back its own length), and starting HBlank DMA with the LCD off (mode
    // reads 0) copies one block immediately.
    run("dma/hdma_lcd_off.gb").unwrap();
}

#[test]
fn dma_hdma_mode0() {
    run("dma/hdma_mode0.gb").unwrap();
}
