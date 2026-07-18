//! `oxgbc-cli run <ROM>` — boot one ROM, report its outcome, optionally dump
//! the serial log and a screenshot.

use crate::args::{next_val, parse_dump, parse_vram, ArgMatch, CommonOpts};
use crate::inspect::{dump_memory, dump_ppu, dump_regs, dump_vram, trace};
use crate::report::{print_result_line, RomResult};
use crate::rom::{compare_to_reference, save_screenshot};
use core::harness;
use std::path::PathBuf;
use std::process::ExitCode;

pub fn cmd_run(args: &[String]) -> Result<ExitCode, String> {
    let mut rom: Option<PathBuf> = None;
    let mut opts = CommonOpts::default();
    let mut screenshot: Option<PathBuf> = None;
    let mut serial = false;
    let mut regs = false;
    let mut trace_len: Option<usize> = None;
    let mut compare: Option<PathBuf> = None;
    let mut tolerance: u8 = 0;
    let mut dumps: Vec<(u16, u16)> = Vec::new();
    let mut vram_dumps: Vec<(u8, u16, u16)> = Vec::new();
    let mut ppu = false;
    let mut no_detect = false;

    let mut it = args.iter();
    while let Some(arg) = it.next() {
        match opts.match_common(arg, &mut it)? {
            ArgMatch::Common => continue,
            ArgMatch::Help => {
                crate::print_usage();
                return Ok(ExitCode::SUCCESS);
            }
            ArgMatch::Other => {}
        }

        match arg.as_str() {
            "--screenshot" => screenshot = Some(PathBuf::from(next_val(&mut it, "--screenshot")?)),
            "--serial" => serial = true,
            "--regs" => regs = true,
            "--trace" => {
                let v = next_val(&mut it, "--trace")?;
                let n = v
                    .parse::<usize>()
                    .map_err(|_| format!("invalid trace length '{v}'"))?;
                if n == 0 {
                    return Err("trace length must be > 0".to_string());
                }
                trace_len = Some(n);
            }
            "--compare" => compare = Some(PathBuf::from(next_val(&mut it, "--compare")?)),
            "--tolerance" => {
                let v = next_val(&mut it, "--tolerance")?;
                tolerance = v
                    .parse::<u8>()
                    .map_err(|_| format!("invalid tolerance '{v}'"))?;
            }
            "--dump" => dumps.push(parse_dump(&next_val(&mut it, "--dump")?)?),
            "--vram" => vram_dumps.push(parse_vram(&next_val(&mut it, "--vram")?)?),
            "--ppu" => ppu = true,
            "--no-detect" => no_detect = true,
            other if other.starts_with('-') => return Err(format!("unknown flag '{other}'")),
            other if rom.is_none() => rom = Some(PathBuf::from(other)),
            other => return Err(format!("unexpected argument '{other}'")),
        }
    }

    let rom = rom.ok_or("missing <ROM> path")?;
    let mut cpu = harness::build_cpu_from_path(&rom, opts.model)?;

    let passed = if let Some(ref_path) = &compare {
        // Visual mode: run for the timeout, then diff the framebuffer against a
        // reference PNG (screenshot-based tests have no register/serial signal).
        harness::run_duration(&mut cpu, opts.timeout);
        match compare_to_reference(&cpu, ref_path, tolerance) {
            Ok(()) => {
                println!("PASS    {}  (visual)", rom.display());
                true
            }
            Err(detail) => {
                println!("FAIL    {}  (visual)  {detail}", rom.display());
                false
            }
        }
    } else if let Some(len) = trace_len {
        // Trace mode is a debugging run (no pass/fail protocol).
        trace(&mut cpu, opts.timeout, len);
        false
    } else if no_detect {
        // Run the full timeout with no pass/fail detection — lets you screenshot
        // or dump a ROM whose result is screen-only, or whose memory happens to
        // trip a false detection (e.g. `auto` matching gbmicrotest's $FF82),
        // without the detector stopping the run after the first frame.
        let start = std::time::Instant::now();
        harness::run_duration(&mut cpu, opts.timeout);
        println!(
            "RAN     {}  ({:.2}s, no-detect)",
            rom.display(),
            start.elapsed().as_secs_f64()
        );
        true
    } else {
        let run = harness::run(&mut cpu, opts.protocol, opts.timeout);
        print_result_line(&RomResult::from_run(rom.display().to_string(), &run));

        if serial && !run.serial.is_empty() {
            println!("--- serial ---");
            println!("{}", run.serial.trim_end_matches(['\n', '\r']));
        }

        run.outcome.is_pass()
    };

    if let Some(path) = screenshot {
        save_screenshot(&cpu, &path)?;
        println!("screenshot -> {}", path.display());
    }

    for (addr, len) in dumps {
        dump_memory(&cpu, addr, len);
    }

    for (bank, addr, len) in vram_dumps {
        dump_vram(&cpu, bank, addr, len);
    }

    if ppu {
        dump_ppu(&cpu);
    }

    if regs {
        dump_regs(&mut cpu);
    }

    Ok(crate::exit_code(passed))
}
