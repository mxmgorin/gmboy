use core::auxiliary::clock::Clock;
use std::path::PathBuf;

mod blargg;
mod gbmicrotest;
mod mooneye;
mod sm83;

pub fn print_with_dashes(content: &str) {
    const TOTAL_LEN: usize = 100;
    let content_length = content.len();
    let dashes = "-".repeat(TOTAL_LEN.saturating_sub(content_length));
    println!("{content} {dashes}");
}

pub fn print_result_path(path: PathBuf, result: Result<(), String>) {
    let path = path.to_string_lossy().to_string();

    if let Err(err) = result {
        eprint!("{path}: FAILED\n{err}")
    } else {
        println!("{path}: OK");
    }
}
