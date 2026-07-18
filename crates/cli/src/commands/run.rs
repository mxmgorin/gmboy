//! `oxgbc-cli run <ROM>` — boot one ROM, report its outcome, optionally dump
//! the serial log and a screenshot.

use crate::args::{next_val, parse_args, parse_dump, parse_vram, print_common_usage, CommonOpts};
use crate::inspect::{dump_memory, dump_ppu, dump_regs, dump_vram, trace};
use crate::report::{print_result_line, RomResult};
use crate::rom::{compare_to_reference, save_screenshot};
use core::cpu::Cpu;
use core::harness;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

/// How the ROM is run and how its pass/fail is decided. The three non-default
/// modes are mutually exclusive.
enum Mode {
    /// Run under the pass/fail protocol detector (the default).
    Detect,
    /// `--no-detect`: run the full timeout with no detection.
    NoDetect,
    /// `--trace N`: record the last N instructions; a debugging run, never a pass.
    Trace(usize),
    /// `--compare`: run the full timeout, then diff the framebuffer against a
    /// reference PNG.
    Compare { reference: PathBuf, tolerance: u8 },
}

/// Everything `run` accepts, parsed and validated.
struct RunOpts {
    rom: PathBuf,
    common: CommonOpts,
    mode: Mode,
    screenshot: Option<PathBuf>,
    serial: bool,
    regs: bool,
    ppu: bool,
    dumps: Vec<(u16, u16)>,
    vram_dumps: Vec<(u8, u16, u16)>,
}

pub fn cmd_run(args: &[String]) -> Result<ExitCode, String> {
    let Some(opts) = parse(args)? else {
        print_usage();
        return Ok(ExitCode::SUCCESS);
    };

    let mut cpu = harness::build_cpu_from_path(&opts.rom, opts.common.model)?;

    let passed = match &opts.mode {
        Mode::Detect => run_detect(&mut cpu, &opts),
        Mode::NoDetect => run_no_detect(&mut cpu, &opts),
        Mode::Trace(len) => {
            trace(&mut cpu, opts.common.timeout, *len);
            false
        }
        Mode::Compare {
            reference,
            tolerance,
        } => run_compare(&mut cpu, &opts, reference, *tolerance),
    };

    inspect_after(&mut cpu, &opts)?;

    Ok(crate::exit_code(passed))
}

/// Parse `run`'s arguments; `None` means help was requested.
fn parse(args: &[String]) -> Result<Option<RunOpts>, String> {
    let mut common = CommonOpts::default();
    let mut rom: Option<PathBuf> = None;
    let mut screenshot: Option<PathBuf> = None;
    let mut serial = false;
    let mut regs = false;
    let mut ppu = false;
    let mut dumps: Vec<(u16, u16)> = Vec::new();
    let mut vram_dumps: Vec<(u8, u16, u16)> = Vec::new();
    let mut compare: Option<PathBuf> = None;
    let mut tolerance: u8 = 0;
    let mut trace_len: Option<usize> = None;
    let mut no_detect = false;

    let help = parse_args(args, &mut common, |arg, it| {
        match arg {
            "--screenshot" => screenshot = Some(PathBuf::from(next_val(it, "--screenshot")?)),
            "--serial" => serial = true,
            "--regs" => regs = true,
            "--ppu" => ppu = true,
            "--dump" => dumps.push(parse_dump(&next_val(it, "--dump")?)?),
            "--vram" => vram_dumps.push(parse_vram(&next_val(it, "--vram")?)?),
            "--compare" => compare = Some(PathBuf::from(next_val(it, "--compare")?)),
            "--tolerance" => {
                let v = next_val(it, "--tolerance")?;
                tolerance = v
                    .parse::<u8>()
                    .map_err(|_| format!("invalid tolerance '{v}'"))?;
            }
            "--trace" => {
                let v = next_val(it, "--trace")?;
                let n = v
                    .parse::<usize>()
                    .map_err(|_| format!("invalid trace length '{v}'"))?;
                if n == 0 {
                    return Err("trace length must be > 0".to_string());
                }
                trace_len = Some(n);
            }
            "--no-detect" => no_detect = true,
            other if other.starts_with('-') => return Err(format!("unknown flag '{other}'")),
            other if rom.is_none() => rom = Some(PathBuf::from(other)),
            other => return Err(format!("unexpected argument '{other}'")),
        }
        Ok(())
    })?;
    if help {
        return Ok(None);
    }

    let mode = match (compare, trace_len, no_detect) {
        (Some(reference), None, false) => Mode::Compare {
            reference,
            tolerance,
        },
        (None, Some(len), false) => Mode::Trace(len),
        (None, None, true) => Mode::NoDetect,
        (None, None, false) => Mode::Detect,
        _ => return Err("--compare, --trace and --no-detect are mutually exclusive".to_string()),
    };

    Ok(Some(RunOpts {
        rom: rom.ok_or("missing <ROM> path")?,
        common,
        mode,
        screenshot,
        serial,
        regs,
        ppu,
        dumps,
        vram_dumps,
    }))
}

/// Default mode: run under the pass/fail detector, print the result line and
/// (with `--serial`) the captured serial log.
fn run_detect(cpu: &mut Cpu, opts: &RunOpts) -> bool {
    let run = harness::run(cpu, opts.common.protocol, opts.common.timeout);
    print_result_line(&RomResult::from_run(opts.rom.display().to_string(), &run));

    if opts.serial && !run.serial.is_empty() {
        println!("--- serial ---");
        println!("{}", run.serial.trim_end_matches(['\n', '\r']));
    }

    run.outcome.is_pass()
}

/// `--no-detect`: run the full timeout with no pass/fail detection — lets you
/// screenshot or dump a ROM whose result is screen-only, or whose memory
/// happens to trip a false detection (e.g. `auto` matching gbmicrotest's
/// $FF82), without the detector stopping the run after the first frame.
fn run_no_detect(cpu: &mut Cpu, opts: &RunOpts) -> bool {
    let start = std::time::Instant::now();
    harness::run_duration(cpu, opts.common.timeout);
    println!(
        "RAN     {}  ({:.2}s, no-detect)",
        opts.rom.display(),
        start.elapsed().as_secs_f64()
    );

    true
}

/// `--compare`: run for the timeout, then diff the framebuffer against a
/// reference PNG (screenshot-based tests have no register/serial signal).
fn run_compare(cpu: &mut Cpu, opts: &RunOpts, reference: &Path, tolerance: u8) -> bool {
    harness::run_duration(cpu, opts.common.timeout);

    match compare_to_reference(cpu, reference, tolerance) {
        Ok(()) => {
            println!("PASS    {}  (visual)", opts.rom.display());
            true
        }
        Err(detail) => {
            println!("FAIL    {}  (visual)  {detail}", opts.rom.display());
            false
        }
    }
}

/// Post-run inspection: screenshot, memory/VRAM hex dumps, PPU and CPU state.
fn inspect_after(cpu: &mut Cpu, opts: &RunOpts) -> Result<(), String> {
    if let Some(path) = &opts.screenshot {
        save_screenshot(cpu, path)?;
        println!("screenshot -> {}", path.display());
    }

    for &(addr, len) in &opts.dumps {
        dump_memory(cpu, addr, len);
    }

    for &(bank, addr, len) in &opts.vram_dumps {
        dump_vram(cpu, bank, addr, len);
    }

    if opts.ppu {
        dump_ppu(cpu);
    }

    if opts.regs {
        dump_regs(cpu);
    }

    Ok(())
}

/// `run`'s full help: synopsis, common options, own flags.
pub fn print_usage() {
    eprintln!("USAGE:  oxgbc-cli run <ROM> [options]\n");
    print_common_usage();
    print_options();
}

/// Only `run`'s option block (also part of the global usage).
pub fn print_options() {
    eprintln!("run OPTIONS:");
    eprintln!("  --screenshot <PATH>      save the final framebuffer as PNG");
    eprintln!("  --no-detect              run the full timeout with no pass/fail detection");
    eprintln!("                           (for screen-only ROMs / to avoid false detections)");
    eprintln!("  --serial                 print captured serial output");
    eprintln!("  --regs                   print CPU registers + opcode bytes at PC after the run");
    eprintln!("  --dump <ADDR[:LEN]>      hex-dump memory after the run (ADDR hex, LEN dec;");
    eprintln!("                           repeatable, e.g. --dump C000:8)");
    eprintln!("  --vram <B:ADDR[:LEN]>    hex-dump VRAM bank B directly (no mode-3 blocking;");
    eprintln!("                           repeatable, e.g. --vram 1:9C00:32)");
    eprintln!("  --ppu                    print PPU registers, window state, and OAM after the run");
    eprintln!("  --trace <N>              record the last N instructions (freezes on a hang)");
    eprintln!("  --compare <PNG>          diff the final framebuffer against a reference PNG");
    eprintln!("  --tolerance <N>          per-channel diff allowed by --compare (default 0)\n");
}
