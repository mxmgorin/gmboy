use rusty_gb_emu::bus::ram::Ram;
use rusty_gb_emu::bus::Bus;
use rusty_gb_emu::cart::Cart;
use rusty_gb_emu::cpu::Cpu;
use rusty_gb_emu::debugger::{CpuLogType, Debugger};
use rusty_gb_emu::emu::read_bytes;
use std::path::PathBuf;
use std::time::{Duration, Instant};

pub fn run_test_rom(rom_name: &str, category: Option<TestRomCategory>, timeout: Duration) {
    let path = get_test_rom_path(rom_name, category);
    let mut debugger = Debugger::new(CpuLogType::None, true);
    let cart = Cart::new(read_bytes(path.to_str().unwrap()).unwrap()).unwrap();
    let mut cpu = Cpu::new(Bus::new(cart, Ram::new()));
    let instant = Instant::now();

    loop {
        cpu.step(Some(&mut debugger)).unwrap();
        let serial_msg = debugger.get_serial_msg().to_lowercase();

        if serial_msg.contains("passed") {
            println!("{}: OK", rom_name);
            break;
        } else if serial_msg.contains("failed") || serial_msg.contains("error") {
            println!("{}: FAILED", rom_name);
            println!("{}", serial_msg);
        }

        if instant.elapsed() > timeout {
            println!("{}: FAILED", rom_name);
            panic!("Timed out!");
        }
    }
}

pub enum TestRomCategory {
    CpuInstructions,
}

pub fn get_test_rom_path(rom_name: &str, category: Option<TestRomCategory>) -> PathBuf {
    let mut root = PathBuf::from("tests").join("blargg").join("roms");

    if let Some(category) = category {
        match category {
            TestRomCategory::CpuInstructions => root = root.join("cpu_instrs"),
        }
    }

    root = root.join(rom_name);

    root
}
