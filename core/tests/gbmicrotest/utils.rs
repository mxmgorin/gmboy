use core::{auxiliary::clock::Clock, bus::Bus, cart::Cart, cpu::Cpu, emu::config::GbModel};
use std::{
    collections::BTreeMap,
    fs,
    path::PathBuf,
    time::{Duration, Instant},
};

pub fn get_gbmicrotest_dir() -> PathBuf {
    let mut path = std::env::current_dir().unwrap();
    path.pop();
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
    let cart = Cart::new(core::read_bytes(path.as_path())?)?;
    let bus = Bus::new(cart, Default::default(), model);
    let clock = Clock::new(bus);
    let mut cpu = Cpu::new(clock);
    let instant = Instant::now();

    loop {
        cpu.step();

        let _expected = cpu.clock.bus.read(0xFF81);
        let _actual = cpu.clock.bus.read(0xFF80);
        let result = cpu.clock.bus.read(0xFF82);

        if result == 0xFF {
            return Err(format!("FAILING RESULT"));
        } else if result == 0x01 {
            return Ok(());
        }

        if instant.elapsed() > timeout {
            return Err(format!("TIMEOUT: {}", timeout.as_secs()));
        }
    }
}
