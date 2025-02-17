use rusty_gb_emu::bus::Bus;
use rusty_gb_emu::cart::Cart;
use rusty_gb_emu::cpu::{Cpu};
use rusty_gb_emu::emu::{read_bytes, EmuCtx};
use std::fmt::Display;
use std::path::PathBuf;
use std::time::{Duration, Instant};

pub fn run_mooneye_rom(
    name: &str,
    category: Option<MooneyeRomCategory>,
    timeout: Duration,
) -> Result<(), String> {
    let path = get_mooneye_rom_path(&format!("{}.gb", name), category);
    let cart = Cart::new(read_bytes(path.to_str().unwrap())?)?;
    let mut callback = EmuCtx::with_fps_limit(10000.0);
    let mut cpu = Cpu::new(Bus::new(cart));
    let instant = Instant::now();

    loop {
        cpu.step(&mut callback, None)?;

        if cpu.registers.b == 3
            && cpu.registers.c == 5
            && cpu.registers.d == 8
            && cpu.registers.e == 13
            && cpu.registers.h == 21
            && cpu.registers.l == 34
        {
            return Ok(());
        }

        if instant.elapsed() > timeout {
            return Err(format!("TIMEOUT: {}", timeout.as_secs()));
        }
    }
}

pub fn assert_result(name: &str, category: Option<MooneyeRomCategory>, result: Result<(), String>) {
    let path = get_mooneye_rom_path(&format!("{}.gb", name), category)
        .to_string_lossy()
        .to_string();

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
        };

        write!(f, "{}", dir)
    }
}
