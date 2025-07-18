use crate::{TestCpuCtxWithPPu};
use core::auxiliary::clock::Clock;
use core::bus::Bus;
use core::cart::Cart;
use core::cpu::Cpu;
use core::debugger::{CpuLogType, Debugger};
use core::emu::{read_bytes};
use std::fmt::Display;
use std::path::PathBuf;
use std::time::{Duration, Instant};

pub fn run_mooneye_rom(
    name: &str,
    category: Option<MooneyeRomCategory>,
    timeout: Duration,
) -> Result<(), String> {
    let path = get_mooneye_rom_path(&format!("{}.gb", name), category);

    run_mooneye_rom_path(path, timeout)
}

pub fn run_mooneye_rom_path(
    path: PathBuf,
    timeout: Duration,
) -> Result<(), String> {
    let cart = Cart::new(read_bytes(path.as_path())?)?;
    let mut callback = TestCpuCtxWithPPu {
        clock: Clock::default(),
        debugger: Debugger::new(CpuLogType::None, false),
        ppu: Default::default(),
    };
    let mut cpu = Cpu::new(Bus::new(cart));
    let instant = Instant::now();

    loop {
        cpu.step(&mut callback)?;

        if cpu.registers.b == 3
            && cpu.registers.c == 5
            && cpu.registers.d == 8
            && cpu.registers.e == 13
            && cpu.registers.h == 21
            && cpu.registers.l == 34
        {
            return Ok(());
        }

        if cpu.registers.b == 0x42
            && cpu.registers.c == 0x42
            && cpu.registers.d == 0x42
            && cpu.registers.e == 0x42
            && cpu.registers.h == 0x42
            && cpu.registers.l == 0x42
        {
            return Err(format!("FAILING RESULT ({:?})", instant.elapsed()));
        }

        if instant.elapsed() > timeout {
            return Err(format!("TIMEOUT: {}", timeout.as_secs()));
        }
    }
}

pub fn assert_result(name: &str, category: Option<MooneyeRomCategory>, result: Result<(), String>) {
    let path = get_mooneye_rom_path(&format!("{}.gb", name), category);

    assert_result_path(path, result);
}

pub fn assert_result_path(path: PathBuf, result: Result<(), String>) {
    let path = path.to_string_lossy().to_string();

    if let Err(err) = result {
        panic!("{path}: FAILED\n{err}")
    } else {
        println!("{path}: OK");
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MooneyeRomCategory {
    OamDma,
    Bits,
    Instr,
    Interrupts,
    Timer,
    Timing,
}

pub fn get_mooneye_rom_path(rom_name: &str, category: Option<MooneyeRomCategory>) -> PathBuf {
    let mut root = PathBuf::from("tests").join("mooneye").join("acceptance");

    if let Some(category) = category {
        root = root.join(category.to_string());
    }

    root = root.join(rom_name);

    root
}

impl Display for MooneyeRomCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dir = match self {
            MooneyeRomCategory::OamDma => "oam_dma",
            MooneyeRomCategory::Bits => "bits",
            MooneyeRomCategory::Instr => "instr",
            MooneyeRomCategory::Interrupts => "interrupts",
            MooneyeRomCategory::Timer => "timer",
            MooneyeRomCategory::Timing => "timing",
        };

        write!(f, "{}", dir)
    }
}
