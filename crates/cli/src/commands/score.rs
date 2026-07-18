//! `oxgbc-cli score [SUITE...]` — regenerate the project's test-ROM scoreboards.
//!
//! Unlike `check` (which scores an arbitrary directory), `score` knows this
//! repo's suites: where each lives, which result protocol fits, and which
//! visual-only ROMs to skip. It runs them with the reliable `auto` protocol and
//! writes `<suite>.json` per suite plus a combined `README.md` into the output
//! directory (default `docs/testing/scores`).

use crate::args::{next_val, ArgMatch, CommonOpts};
use crate::report::{render_index, Report, RomResult};
use crate::rom::{collect_roms, is_excluded};
use core::harness;
use std::path::PathBuf;
use std::process::ExitCode;

/// A known suite: display name, ROM directory (relative to the repo root), and
/// globs for ROMs that have no auto-detectable result (visual-only).
struct Suite {
    name: &'static str,
    dir: &'static str,
    excludes: &'static [&'static str],
}

const SUITES: &[Suite] = &[
    Suite {
        name: "blargg",
        dir: "crates/core/tests/blargg/roms",
        // Screen-only: renders "Passed" on the LCD, no serial/memory signal.
        excludes: &["interrupt_time.gb"],
    },
    Suite {
        name: "mooneye",
        dir: "crates/core/tests/mooneye",
        excludes: &[
            // Visual test: no register signature to detect.
            "manual-only/*",
            // Hardware revisions oxGBC does not emulate (it targets DMG ABC
            // and CGB A-E). These assert the exact post-boot state of other
            // revisions and are unpassable by design — mutually exclusive
            // with the -dmgABC / -cgbABCDE variants that do pass.
            "*-dmg0.gb",  // DMG revision 0
            "*-mgb.gb",   // Game Boy Pocket
            "*-sgb.gb",   // Super Game Boy
            "*-sgb2.gb",  // Super Game Boy 2
            "*-S.gb",     // SGB + SGB2
            "*-A.gb",     // AGB + AGS (Game Boy Advance)
            "*-cgb0.gb",  // CGB revision 0
        ],
    },
    Suite {
        name: "same-suite",
        dir: "roms/same-suite",
        excludes: &[],
    },
];

const DEFAULT_OUT: &str = "docs/testing/scores";

pub fn cmd_score(args: &[String]) -> Result<ExitCode, String> {
    let mut opts = CommonOpts::default();
    let mut out = PathBuf::from(DEFAULT_OUT);
    let mut wanted: Vec<String> = Vec::new();

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
            "--out" => out = PathBuf::from(next_val(&mut it, "--out")?),
            other if other.starts_with('-') => return Err(format!("unknown flag '{other}'")),
            other => wanted.push(other.to_string()),
        }
    }

    let selected = select_suites(&wanted)?;
    std::fs::create_dir_all(&out).map_err(|e| e.to_string())?;

    let mut reports: Vec<(&str, Report)> = Vec::with_capacity(selected.len());
    let mut all_pass = true;

    for suite in selected {
        let report = score_suite(suite, &opts)?;
        eprintln!(
            "{:<11} {}/{} pass ({} fail, {} timeout, {} error)",
            suite.name, report.passed, report.total, report.failed, report.timeout, report.errored
        );

        let json = serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?;
        std::fs::write(out.join(format!("{}.json", suite.name)), json + "\n")
            .map_err(|e| e.to_string())?;

        all_pass &= report.passed == report.total;
        reports.push((suite.name, report));
    }

    std::fs::write(out.join("README.md"), render_index(&reports)).map_err(|e| e.to_string())?;
    eprintln!("wrote {} scoreboard(s) -> {}", reports.len(), out.display());

    Ok(crate::exit_code(all_pass))
}

/// Resolve the requested suite names (empty = all), erroring on an unknown one.
fn select_suites(wanted: &[String]) -> Result<Vec<&'static Suite>, String> {
    if wanted.is_empty() {
        return Ok(SUITES.iter().collect());
    }

    wanted
        .iter()
        .map(|name| {
            SUITES.iter().find(|s| s.name == name).ok_or_else(|| {
                let known: Vec<&str> = SUITES.iter().map(|s| s.name).collect();
                format!("unknown suite '{name}' (known: {})", known.join(", "))
            })
        })
        .collect()
}

fn score_suite(suite: &Suite, opts: &CommonOpts) -> Result<Report, String> {
    let dir = PathBuf::from(suite.dir);
    if !dir.is_dir() {
        return Err(format!(
            "suite '{}' dir not found: {} (run `score` from the repo root)",
            suite.name,
            dir.display()
        ));
    }

    let mut roms = Vec::new();
    collect_roms(&dir, true, &mut roms).map_err(|e| e.to_string())?;
    roms.sort();

    let excludes: Vec<String> = suite.excludes.iter().map(|s| s.to_string()).collect();
    roms.retain(|rom| !is_excluded(rom.strip_prefix(&dir).unwrap_or(rom), &excludes));
    if roms.is_empty() {
        return Err(format!("no ROMs found for suite '{}'", suite.name));
    }

    let results = roms
        .iter()
        .map(|rom| {
            let name = rom.strip_prefix(&dir).unwrap_or(rom).display().to_string();
            // The mooneye misc/ ROMs target CGB hardware but carry DMG-flagged
            // cart headers, so header auto-detection picks the wrong model.
            let model = if suite.name == "mooneye" && name.starts_with("misc/") {
                opts.model.or(Some(core::emu::config::GbModel::Cgb))
            } else {
                opts.model
            };

            match harness::build_cpu_from_path(rom, model) {
                Ok(mut cpu) => RomResult::from_run(name, &harness::run(&mut cpu, opts.protocol, opts.timeout)),
                Err(e) => RomResult::error(name, e),
            }
        })
        .collect();

    Ok(Report::new(&dir, results))
}
