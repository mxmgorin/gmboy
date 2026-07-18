//! `oxgbc-cli check <DIR>` — batch-run every ROM under a directory and print a
//! pass/fail table (or a JSON scoreboard with `--json`).

use crate::args::{next_val, parse_args, print_common_usage, CommonOpts};
use crate::report::{print_result_line, Report, RomResult};
use crate::rom::{collect_roms, is_excluded, sanitize, save_screenshot};
use core::harness;
use std::path::PathBuf;
use std::process::ExitCode;

pub fn cmd_check(args: &[String]) -> Result<ExitCode, String> {
    let mut opts = CommonOpts::default();
    let mut dir: Option<PathBuf> = None;
    let mut recursive = false;
    let mut json = false;
    let mut screenshot_dir: Option<PathBuf> = None;
    let mut excludes: Vec<String> = Vec::new();

    let help = parse_args(args, &mut opts, |arg, it| {
        match arg {
            "-r" | "--recursive" => recursive = true,
            "--json" => json = true,
            "--screenshot-dir" => {
                screenshot_dir = Some(PathBuf::from(next_val(it, "--screenshot-dir")?))
            }
            "--exclude" => excludes.push(next_val(it, "--exclude")?),
            other if other.starts_with('-') => return Err(format!("unknown flag '{other}'")),
            other if dir.is_none() => dir = Some(PathBuf::from(other)),
            other => return Err(format!("unexpected argument '{other}'")),
        }
        Ok(())
    })?;
    if help {
        print_usage();
        return Ok(ExitCode::SUCCESS);
    }

    let dir = dir.ok_or("missing <DIR> path")?;
    if !dir.is_dir() {
        return Err(format!("not a directory: {}", dir.display()));
    }

    let mut roms = Vec::new();
    collect_roms(&dir, recursive, &mut roms).map_err(|e| e.to_string())?;
    roms.sort();
    if !excludes.is_empty() {
        roms.retain(|rom| {
            let rel = rom.strip_prefix(&dir).unwrap_or(rom);
            !is_excluded(rel, &excludes)
        });
    }
    if roms.is_empty() {
        return Err(format!("no .gb/.gbc ROMs found in {}", dir.display()));
    }

    if let Some(sd) = &screenshot_dir {
        std::fs::create_dir_all(sd).map_err(|e| e.to_string())?;
    }

    let mut results = Vec::with_capacity(roms.len());
    for rom in &roms {
        let rel = rom.strip_prefix(&dir).unwrap_or(rom).to_path_buf();
        let name = rel.display().to_string();

        let result = match harness::build_cpu_from_path(rom, opts.model) {
            Ok(mut cpu) => {
                let run = harness::run(&mut cpu, opts.protocol, opts.timeout);
                if let Some(sd) = &screenshot_dir {
                    let out = sd.join(format!("{}.png", sanitize(&rel)));
                    if let Err(e) = save_screenshot(&cpu, &out) {
                        eprintln!("screenshot error for {name}: {e}");
                    }
                }
                RomResult::from_run(name, &run)
            }
            Err(e) => RomResult::error(name, e),
        };

        if !json {
            print_result_line(&result);
        }
        results.push(result);
    }

    let report = Report::new(&dir, results);
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?
        );
    } else {
        report.print_summary();
    }

    Ok(crate::exit_code(report.passed == report.total))
}

/// `check`'s full help: synopsis, common options, own flags.
pub fn print_usage() {
    eprintln!("USAGE:  oxgbc-cli check <DIR> [options]\n");
    print_common_usage();
    print_options();
}

/// Only `check`'s option block (also part of the global usage).
pub fn print_options() {
    eprintln!("check OPTIONS:");
    eprintln!("  -r, --recursive          descend into subdirectories");
    eprintln!("  --exclude <GLOB>         skip ROMs matching a glob (repeatable; * ? incl. /)");
    eprintln!("  --json                   emit a JSON scoreboard");
    eprintln!("  --screenshot-dir <DIR>   save each ROM's framebuffer as PNG\n");
}
