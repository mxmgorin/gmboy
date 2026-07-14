//! `oxgbc-cli` — a thin headless runner for Game Boy test ROMs.
//!
//! It drives `core::harness` (the same boot + pass/fail detection the
//! integration tests use) so an arbitrary ROM can be run outside `cargo test`,
//! screenshotted, and batch-scored. Two commands:
//!
//! ```text
//! oxgbc-cli run   <ROM> [--model ..] [--timeout ..] [--protocol ..] [--screenshot P] [--serial]
//! oxgbc-cli check <DIR> [--model ..] [--timeout ..] [--protocol ..] [-r] [--json] [--screenshot-dir D]
//! ```

mod args;
mod commands;
mod report;
mod rom;

use crate::args::DEFAULT_TIMEOUT_SECS;
use crate::commands::{cmd_check, cmd_run};
use std::process::ExitCode;

fn main() -> ExitCode {
    let argv: Vec<String> = std::env::args().collect();

    match dispatch(&argv) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("error: {err}\n");
            print_usage();
            ExitCode::from(2)
        }
    }
}

fn dispatch(argv: &[String]) -> Result<ExitCode, String> {
    match argv.get(1).map(String::as_str) {
        Some("run") => cmd_run(&argv[2..]),
        Some("check") => cmd_check(&argv[2..]),
        Some("-h") | Some("--help") | Some("help") => {
            print_usage();
            Ok(ExitCode::SUCCESS)
        }
        Some(other) => Err(format!("unknown command '{other}'")),
        None => {
            print_usage();
            Ok(ExitCode::from(2))
        }
    }
}

/// Map an "all good?" flag to a process exit code (0 = success, 1 = failure).
pub fn exit_code(ok: bool) -> ExitCode {
    if ok {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

pub fn print_usage() {
    eprintln!("oxgbc-cli — headless Game Boy test-ROM runner\n");
    eprintln!("USAGE:");
    eprintln!("  oxgbc-cli run   <ROM> [options]");
    eprintln!("  oxgbc-cli check <DIR> [options]\n");
    eprintln!("COMMON OPTIONS:");
    eprintln!("  --model <dmg|cgb|auto>   force hardware model (default: auto from header)");
    eprintln!("  --timeout <secs>         per-ROM timeout (default: {DEFAULT_TIMEOUT_SECS})");
    eprintln!("  --protocol <p>           auto|mooneye|blargg-serial|blargg-memory|gbmicrotest");
    eprintln!("                           (default: auto)\n");
    eprintln!("run OPTIONS:");
    eprintln!("  --screenshot <PATH>      save the final framebuffer as PNG");
    eprintln!("  --serial                 print captured serial output");
    eprintln!("  --dump <ADDR[:LEN]>      hex-dump memory after the run (ADDR hex, LEN dec;");
    eprintln!("                           repeatable, e.g. --dump C000:8)\n");
    eprintln!("check OPTIONS:");
    eprintln!("  -r, --recursive          descend into subdirectories");
    eprintln!("  --json                   emit a JSON scoreboard");
    eprintln!("  --screenshot-dir <DIR>   save each ROM's framebuffer as PNG");
}
