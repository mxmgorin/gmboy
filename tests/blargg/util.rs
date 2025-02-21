use mgboy::bus::Bus;
use mgboy::cart::Cart;
use mgboy::cpu::Cpu;
use mgboy::debugger::{CpuLogType, Debugger};
use mgboy::emu::{read_bytes, EmuCtx};
use std::path::PathBuf;
use std::time::{Duration, Instant};

pub fn run_blargg_rom_serial(
    name: &str,
    category: Option<TestRomCategory>,
    timeout: Duration,
) -> Result<(), String> {
    let path = get_blargg_rom_path(&format!("{}.gb", name), category);
    let debugger = Debugger::new(CpuLogType::None, true);
    let cart = Cart::new(read_bytes(path.to_str().unwrap())?)?;
    let mut cpu = Cpu::new(Bus::new(cart));
    let instant = Instant::now();
    let mut ctx = EmuCtx {
        clock: Default::default(),
        debugger: Some(debugger),
        cart: None,
    };

    loop {
        cpu.step(&mut ctx)?;
        let serial_msg = ctx.debugger.as_ref().unwrap().get_serial_msg().to_lowercase();

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
    category: Option<TestRomCategory>,
    timeout: Duration,
) -> Result<(), String> {
    let path = get_blargg_rom_path(&format!("{}.gb", name), category);
    let cart = Cart::new(read_bytes(path.to_str().unwrap())?)?;
    let mut cpu = Cpu::new(Bus::new(cart));
    let instant = Instant::now();

    loop {
        cpu.step(&mut EmuCtx::default())?;
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
    OamBug,
}

pub fn get_blargg_rom_path(rom_name: &str, category: Option<TestRomCategory>) -> PathBuf {
    let mut root = PathBuf::from("tests").join("blargg").join("roms");

    if let Some(category) = category {
        let dir = match category {
            TestRomCategory::CpuInstructions => "cpu_instrs",
            TestRomCategory::MemTiming => "mem_timing",
            TestRomCategory::OamBug => "oam_bug",
        };

        root = root.join(dir);
    }

    root = root.join(rom_name);

    root
}
