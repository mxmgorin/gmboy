//! `oxgbc-cli run <ROM>` — boot one ROM, report its outcome, optionally dump
//! the serial log and a screenshot.

use crate::args::{next_val, parse_dump, ArgMatch, CommonOpts};
use crate::report::{print_result_line, RomResult};
use crate::rom::{dump_memory, save_screenshot};
use core::harness;
use std::path::PathBuf;
use std::process::ExitCode;

pub fn cmd_run(args: &[String]) -> Result<ExitCode, String> {
    let mut rom: Option<PathBuf> = None;
    let mut opts = CommonOpts::default();
    let mut screenshot: Option<PathBuf> = None;
    let mut serial = false;
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
            "--dump" => dumps.push(parse_dump(&next_val(&mut it, "--dump")?)?),
            other if other.starts_with('-') => return Err(format!("unknown flag '{other}'")),
            other if rom.is_none() => rom = Some(PathBuf::from(other)),
            other => return Err(format!("unexpected argument '{other}'")),
        }
    }

    let rom = rom.ok_or("missing <ROM> path")?;
    let mut cpu = harness::build_cpu_from_path(&rom, opts.model)?;
    let run = harness::run(&mut cpu, opts.protocol, opts.timeout);

    print_result_line(&RomResult::from_run(rom.display().to_string(), &run));

    if serial && !run.serial.is_empty() {
        println!("--- serial ---");
        println!("{}", run.serial.trim_end_matches(['\n', '\r']));
    }

    if let Some(path) = screenshot {
        save_screenshot(&cpu, &path)?;
        println!("screenshot -> {}", path.display());
    }

    for (addr, len) in dumps {
        dump_memory(&cpu, addr, len);
    }

    Ok(crate::exit_code(run.outcome.is_pass()))
}
