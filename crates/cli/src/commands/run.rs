//! `oxgbc-cli run <ROM>` — boot one ROM, report its outcome, optionally dump
//! the serial log and a screenshot.

use crate::args::{next_val, parse_dump, ArgMatch, CommonOpts};
use crate::inspect::{dump_memory, dump_regs, trace};
use crate::report::{print_result_line, RomResult};
use crate::rom::save_screenshot;
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
    let mut dumps: Vec<(u16, u16)> = Vec::new();

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
            "--dump" => dumps.push(parse_dump(&next_val(&mut it, "--dump")?)?),
            other if other.starts_with('-') => return Err(format!("unknown flag '{other}'")),
            other if rom.is_none() => rom = Some(PathBuf::from(other)),
            other => return Err(format!("unexpected argument '{other}'")),
        }
    }

    let rom = rom.ok_or("missing <ROM> path")?;
    let mut cpu = harness::build_cpu_from_path(&rom, opts.model)?;

    // Trace mode is a debugging run (no pass/fail protocol); otherwise run the
    // ROM normally and report its outcome.
    let passed = if let Some(len) = trace_len {
        trace(&mut cpu, opts.timeout, len);
        false
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

    if regs {
        dump_regs(&mut cpu);
    }

    Ok(crate::exit_code(passed))
}
