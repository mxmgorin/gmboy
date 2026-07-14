use core::emu::config::GbModel;
use core::harness::{self, TestProtocol};
use std::{collections::BTreeMap, fs, path::PathBuf, time::Duration};

pub fn get_gbmicrotest_dir() -> PathBuf {
    // Runs with crates/core as the working dir; pop twice to reach the repo root.
    let mut path = std::env::current_dir().unwrap();
    path.pop(); // crates/core -> crates
    path.pop(); // crates -> repo root
    path.join("roms").join("gbmicrotest")
}

pub fn run_gbmicrotest_dir_roms(
    take: usize,
    skip: usize,
    stop_on_err: bool,
) -> BTreeMap<PathBuf, Result<(), String>> {
    let timeout = Duration::from_secs(1);
    let dir_path = get_gbmicrotest_dir();
    let dir = fs::read_dir(dir_path).unwrap();

    let roms: Vec<_> = dir
        .filter_map(|dir| {
            if let Ok(entry) = dir {
                Some(entry.path())
            } else {
                None
            }
        })
        .collect();

    let mut results = BTreeMap::new();

    for path in roms.into_iter().skip(skip).take(take) {
        let result = run_gbmicrotest_rom_path(Some(GbModel::Dmg), path.clone(), timeout);
        let is_err = result.is_err();
        results.insert(path, result);

        if stop_on_err && is_err {
            return results;
        }
    }

    results
}

pub fn run_gbmicrotest_rom_path(
    model: Option<GbModel>,
    path: PathBuf,
    timeout: Duration,
) -> Result<(), String> {
    harness::run_rom(&path, model, TestProtocol::GbMicrotest, timeout)?.into_result()
}
