use core::harness::{self, TestProtocol};
use std::path::PathBuf;
use std::time::Duration;

pub fn run_blargg_rom_serial(
    name: &str,
    category: Option<BlarggRomCategory>,
    timeout: Duration,
) -> Result<(), String> {
    let path = get_blargg_rom_path(&format!("{name}.gb"), category);

    harness::run_rom(&path, None, TestProtocol::BlarggSerial, timeout)?.into_result()
}

pub fn run_blargg_rom_memory(
    name: &str,
    category: Option<BlarggRomCategory>,
    timeout: Duration,
) -> Result<(), String> {
    let path = get_blargg_rom_path(&format!("{name}.gb"), category);

    harness::run_rom(&path, None, TestProtocol::BlarggMemory, timeout)?.into_result()
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
