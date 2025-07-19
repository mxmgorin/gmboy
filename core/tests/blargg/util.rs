use crate::TestCpuCtx;
use core::bus::Bus;
use core::cart::Cart;
use core::cpu::Cpu;
use core::debugger::{CpuLogType, Debugger};
use core::emu::emu::read_bytes;
use std::path::PathBuf;
use std::time::{Duration, Instant};

pub fn run_blargg_rom_serial(
    name: &str,
    category: Option<BlarggRomCategory>,
    timeout: Duration,
) -> Result<(), String> {
    let path = get_blargg_rom_path(&format!("{name}.gb"), category);
    let cart = Cart::new(read_bytes(path.as_path())?)?;
    let mut cpu = Cpu::new(Bus::new(cart, Default::default()));
    let instant = Instant::now();
    let mut ctx = TestCpuCtx {
        clock: Default::default(),
        debugger: Debugger::new(CpuLogType::None, true),
    };

    loop {
        cpu.step(&mut ctx)?;
        let serial_msg = ctx.debugger.get_serial_msg().to_lowercase();

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

pub fn run_blargg_rom_memory(
    name: &str,
    category: Option<BlarggRomCategory>,
    timeout: Duration,
) -> Result<(), String> {
    let path = get_blargg_rom_path(&format!("{}.gb", name), category);
    let cart = Cart::new(read_bytes(path.as_path())?)?;
    let mut cpu = Cpu::new(Bus::new(cart, Default::default()));
    let instant = Instant::now();
    let mut ctx = TestCpuCtx {
        clock: Default::default(),
        debugger: Debugger::new(CpuLogType::None, false),
    };

    loop {
        cpu.step(&mut ctx)?;
        let b1 = cpu.bus.read(0xA001);
        let b2 = cpu.bus.read(0xA002);
        let b3 = cpu.bus.read(0xA003);
        let result = cpu.bus.read(0xA000);

        if b1 == 0xDE && b2 == 0xB0 && b3 == 0x61 && result != 0x80 {
            match result {
                0 => return Ok(()),
                1 => return Err(format!("{result}: failed")),
                2 => return Err(format!("{result}: error2")),
                _ => unreachable!(),
            }
        }

        if instant.elapsed() > timeout {
            return Err(format!("TIMEOUT: {}", timeout.as_secs()));
        }
    }
}

pub fn assert_result(name: &str, category: Option<BlarggRomCategory>, result: Result<(), String>) {
    let path = get_blargg_rom_path(&format!("{}.gb", name), category)
        .to_string_lossy()
        .to_string();

    if let Err(err) = result {
        panic!("{path}: FAILED\n{err}")
    } else {
        println!("{path}: OK");
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BlarggRomCategory {
    CpuInstructions,
    MemTiming,
    OamBug,
}

pub fn get_blargg_rom_path(rom_name: &str, category: Option<BlarggRomCategory>) -> PathBuf {
    let mut root = PathBuf::from("tests").join("blargg").join("roms");

    if let Some(category) = category {
        let dir = match category {
            BlarggRomCategory::CpuInstructions => "cpu_instrs",
            BlarggRomCategory::MemTiming => "mem_timing",
            BlarggRomCategory::OamBug => "oam_bug",
        };

        root = root.join(dir);
    }

    root = root.join(rom_name);

    root
}
