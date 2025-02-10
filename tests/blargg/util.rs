use rusty_gb_emu::bus::Bus;
use rusty_gb_emu::cart::Cart;
use rusty_gb_emu::cpu::Cpu;
use rusty_gb_emu::debugger::{CpuLogType, Debugger};
use rusty_gb_emu::emu::{read_bytes, EmuCtx};
use std::path::PathBuf;
use std::time::{Duration, Instant};

pub fn run_test_rom(
    name: &str,
    category: Option<TestRomCategory>,
    timeout: Duration,
) -> Result<(), String> {
    let path = get_test_rom_path(&format!("{}.gb", name), category);
    let debugger = Debugger::new(CpuLogType::None, true);
    let cart = Cart::new(read_bytes(path.to_str().unwrap())?)?;
    let mut cpu = Cpu::new(Bus::new(cart));
    let mut ctx = EmuCtx::with_debugger(debugger);
    let instant = Instant::now();

    loop {
        cpu.step(&mut ctx)?;
        let serial_msg = ctx.get_serial_msg().unwrap_or_default().to_lowercase();

        if serial_msg.contains("passed") {
            return Ok(());
        } else if serial_msg.contains("failed") || serial_msg.contains("error") {
            return Err(serial_msg);
        }

        if instant.elapsed() > timeout {
            return Err(format!("TIMEOUT: {}", timeout.as_secs()));
        }
    }
}

pub fn assert_result(name: &str, category: Option<TestRomCategory>, result: Result<(), String>) {
    let name = if let Some(category) = category {
        format!("{:?} {}", category, name)
    } else {
        name.to_owned()
    };

    if let Err(err) = result {
        panic!("{name}: FAILED\n{err}")
    } else {
        println!("{name}: OK");
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TestRomCategory {
    CpuInstructions,
    MemTiming,
}

pub fn get_test_rom_path(rom_name: &str, category: Option<TestRomCategory>) -> PathBuf {
    let mut root = PathBuf::from("tests").join("blargg").join("roms");

    if let Some(category) = category {
        let dir = match category {
            TestRomCategory::CpuInstructions => "cpu_instrs",
            TestRomCategory::MemTiming => "mem_timing",
        };

        root = root.join(dir);
    }

    root = root.join(rom_name);

    root
}
