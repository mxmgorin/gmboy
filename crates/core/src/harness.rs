//! Headless test-ROM harness shared by the integration tests and the `cli`
//! crate. It is the single place that knows how to boot a ROM and detect the
//! pass/fail signals the common Game Boy test suites use, so the detection
//! logic lives in one spot instead of being copied across every `tests/*/util.rs`.
//!
//! Supported result protocols:
//! - **Mooneye**: the fib register signature (`b,c,d,e,h,l = 3,5,8,13,21,34`)
//!   means pass; all-`0x42` means fail.
//! - **Blargg (serial)**: an ASCII `Passed`/`Failed` report on the serial port.
//! - **Blargg (memory)**: a result byte at `$A000`, guarded by the `$DE $B0 $61`
//!   signature at `$A001..=$A003`.
//! - **gbmicrotest**: a result byte at `$FF82` (`0x01` pass, `0xFF` fail), with
//!   the expected/actual bytes at `$FF81`/`$FF80`.

use crate::auxiliary::clock::Clock;
use crate::auxiliary::io::Io;
use crate::bus::Bus;
use crate::cart::Cart;
use crate::cpu::Cpu;
use crate::debugger::{DebugLogType, Debugger};
use crate::emu::config::GbModel;
use std::path::Path;
use std::time::Duration;
use web_time::Instant;

/// The outcome of a headless test-ROM run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TestOutcome {
    Pass,
    Fail(String),
    /// Reached the timeout without the protocol reporting a result. For a
    /// self-checking ROM this is a failure; for a visual ROM it just means
    /// "no serial/register signal — inspect the framebuffer".
    Timeout,
}

impl TestOutcome {
    pub fn is_pass(&self) -> bool {
        matches!(self, TestOutcome::Pass)
    }
}

/// Which result protocol(s) to watch while a ROM runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestProtocol {
    /// Mooneye: fib register signature = pass, all-`0x42` = fail.
    Mooneye,
    /// Blargg: ASCII `Passed`/`Failed` on the serial port.
    BlarggSerial,
    /// Blargg: result byte at `$A000`, guarded by the `$DE $B0 $61` signature.
    BlarggMemory,
    /// gbmicrotest: result byte at `$FF82` (`0x01` = pass, `0xFF` = fail).
    GbMicrotest,
    /// Watch mooneye + both blargg protocols; whichever reports first wins.
    /// Reliable for blargg (serial *and* memory ROMs in one pass), mooneye and
    /// same-suite. Deliberately excludes [`GbMicrotest`], whose bare `$FF82` byte
    /// reads `0xFF` (= "fail") on unrelated ROMs and would false-trigger — run
    /// gbmicrotest with `--protocol gbmicrotest` explicitly.
    Auto,
}

impl TestProtocol {
    fn watches(self, p: TestProtocol) -> bool {
        self == TestProtocol::Auto || self == p
    }
}

/// A completed headless run: the outcome plus any serial output and wall time.
#[derive(Debug, Clone)]
pub struct TestRun {
    pub outcome: TestOutcome,
    pub serial: String,
    pub elapsed: Duration,
}

impl TestRun {
    /// Collapse into the `Result<(), String>` the integration tests assert on.
    pub fn into_result(self) -> Result<(), String> {
        match self.outcome {
            TestOutcome::Pass => Ok(()),
            TestOutcome::Fail(msg) => Err(msg),
            TestOutcome::Timeout => Err(format!("TIMEOUT ({:.1}s)", self.elapsed.as_secs_f64())),
        }
    }
}

/// Build a ready-to-step CPU from raw ROM bytes.
pub fn build_cpu(rom: Box<[u8]>, model: Option<GbModel>) -> Result<Cpu, String> {
    let cart = Cart::new(rom)?;
    let bus = Bus::new(cart, Io::default(), model);

    Ok(Cpu::new(Clock::new(bus)))
}

/// Build a ready-to-step CPU from a ROM file on disk.
pub fn build_cpu_from_path(path: &Path, model: Option<GbModel>) -> Result<Cpu, String> {
    build_cpu(crate::read_bytes(path)?, model)
}

// Wall-clock and serial are only checked every so often; the cheap register /
// memory probes run every step so the timeout stays tight without paying for an
// `Instant::now()` (and a serial re-scan) on every CPU step.
const POLL_STEPS: u32 = 1 << 12;

/// Run `cpu` until the chosen protocol reports pass/fail or `timeout` elapses.
///
/// Takes `&mut Cpu` so the caller keeps ownership and can inspect state (e.g.
/// grab the framebuffer for a screenshot) after the run returns.
pub fn run(cpu: &mut Cpu, protocol: TestProtocol, timeout: Duration) -> TestRun {
    let watch_serial = protocol.watches(TestProtocol::BlarggSerial);
    let mut debugger = Debugger::new(DebugLogType::None, true);
    let start = Instant::now();
    let mut since_poll: u32 = 0;

    loop {
        // Only feed the debugger (which polls the serial port) when a serial
        // protocol is in play; other suites match the plain `cpu.step()` path.
        if watch_serial {
            cpu.step_debug(&mut debugger);
        } else {
            cpu.step();
        }

        if let Some(outcome) = probe(cpu, protocol) {
            return finish(outcome, &debugger, start);
        }

        since_poll += 1;
        if since_poll >= POLL_STEPS {
            since_poll = 0;

            if watch_serial {
                let msg = debugger.get_serial_msg();
                let lower = msg.to_lowercase();
                if lower.contains("passed") {
                    return finish(TestOutcome::Pass, &debugger, start);
                } else if lower.contains("failed") || lower.contains("error") {
                    let msg = msg.into_owned();
                    return finish(TestOutcome::Fail(msg), &debugger, start);
                }
            }

            if start.elapsed() > timeout {
                return finish(TestOutcome::Timeout, &debugger, start);
            }
        }
    }
}

/// Boot a ROM from disk and run it to completion in one call. Use this when the
/// framebuffer is not needed; use [`build_cpu_from_path`] + [`run`] otherwise.
pub fn run_rom(
    path: &Path,
    model: Option<GbModel>,
    protocol: TestProtocol,
    timeout: Duration,
) -> Result<TestRun, String> {
    let mut cpu = build_cpu_from_path(path, model)?;

    Ok(run(&mut cpu, protocol, timeout))
}

/// Step `cpu` for `duration` of wall-clock time, ignoring every protocol. For
/// visual ROMs whose result is the framebuffer rather than a serial/register code.
pub fn run_duration(cpu: &mut Cpu, duration: Duration) {
    let start = Instant::now();

    loop {
        cpu.step();

        if start.elapsed() > duration {
            return;
        }
    }
}

fn finish(outcome: TestOutcome, debugger: &Debugger, start: Instant) -> TestRun {
    TestRun {
        outcome,
        serial: debugger.get_serial_msg().into_owned(),
        elapsed: start.elapsed(),
    }
}

/// Cheap per-step checks (register compares + a few memory reads).
fn probe(cpu: &Cpu, protocol: TestProtocol) -> Option<TestOutcome> {
    if protocol.watches(TestProtocol::Mooneye) {
        if let Some(o) = mooneye_probe(cpu) {
            return Some(o);
        }
    }

    if protocol.watches(TestProtocol::BlarggMemory) {
        if let Some(o) = blargg_memory_probe(cpu) {
            return Some(o);
        }
    }

    // gbmicrotest is opt-in only: its bare `$FF82` byte collides with unrelated
    // ROMs' HRAM, so it is never part of `Auto`.
    if protocol == TestProtocol::GbMicrotest {
        if let Some(o) = gbmicrotest_probe(cpu) {
            return Some(o);
        }
    }

    None
}

fn mooneye_probe(cpu: &Cpu) -> Option<TestOutcome> {
    // Mooneye signals its result with the `LD B,B` debug breakpoint; the
    // registers are only meaningful there. Sampling them continuously gives
    // false failures (boot_sclk_align counts B..L in lockstep through $42).
    if cpu.step_ctx.opcode != 0x40 {
        return None;
    }

    let r = &cpu.registers;

    if r.b == 3 && r.c == 5 && r.d == 8 && r.e == 13 && r.h == 21 && r.l == 34 {
        Some(TestOutcome::Pass)
    } else if r.b == 0x42 && r.c == 0x42 && r.d == 0x42 && r.e == 0x42 && r.h == 0x42 && r.l == 0x42 {
        Some(TestOutcome::Fail("mooneye failure register signature".to_string()))
    } else {
        None
    }
}

fn blargg_memory_probe(cpu: &Cpu) -> Option<TestOutcome> {
    let bus = &cpu.clock.bus;

    if bus.read(0xA001) != 0xDE || bus.read(0xA002) != 0xB0 || bus.read(0xA003) != 0x61 {
        return None;
    }

    match bus.read(0xA000) {
        0x80 => None, // still running
        0x00 => Some(TestOutcome::Pass),
        code => Some(TestOutcome::Fail(format!("blargg result code {code:#04x}"))),
    }
}

fn gbmicrotest_probe(cpu: &Cpu) -> Option<TestOutcome> {
    let bus = &cpu.clock.bus;

    match bus.read(0xFF82) {
        0x01 => Some(TestOutcome::Pass),
        0xFF => {
            let expected = bus.read(0xFF81);
            let actual = bus.read(0xFF80);
            Some(TestOutcome::Fail(format!(
                "gbmicrotest: expected {expected:#04x}, got {actual:#04x}"
            )))
        }
        _ => None,
    }
}
