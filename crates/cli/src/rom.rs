//! ROM discovery on disk and framebuffer screenshots.

use core::cpu::Cpu;
use core::ppu::{LCD_X_RES, LCD_Y_RES};
use std::path::{Path, PathBuf};

/// Collect `*.gb`/`*.gbc` files under `dir`, descending into subdirectories when
/// `recursive` is set.
pub fn collect_roms(dir: &Path, recursive: bool, out: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();

        if path.is_dir() {
            if recursive {
                collect_roms(&path, recursive, out)?;
            }
        } else if is_rom(&path) {
            out.push(path);
        }
    }

    Ok(())
}

fn is_rom(path: &Path) -> bool {
    matches!(
        path.extension()
            .and_then(|e| e.to_str())
            .map(str::to_lowercase)
            .as_deref(),
        Some("gb") | Some("gbc")
    )
}

/// Save the CPU's current LCD framebuffer as an RGB PNG.
pub fn save_screenshot(cpu: &Cpu, path: &Path) -> Result<(), String> {
    let buffer = cpu.clock.bus.io.ppu.lcd.buffer.rgb888();

    image::save_buffer(
        path,
        &buffer,
        LCD_X_RES as u32,
        LCD_Y_RES as u32,
        image::ColorType::Rgb8,
    )
    .map_err(|e| e.to_string())
}

/// Turn a relative ROM path into a flat, filesystem-safe file stem for a
/// screenshot (e.g. `apu/div_write.gb` -> `apu_div_write.gb`).
pub fn sanitize(rel: &Path) -> String {
    rel.to_string_lossy().replace(['/', '\\'], "_")
}

/// Print a hex dump of `len` bytes starting at `addr`, as seen through the bus
/// (so memory-mapping/PPU-mode blocking applies, just like the CPU sees it).
pub fn dump_memory(cpu: &Cpu, addr: u16, len: u16) {
    for row in (0..len).step_by(16) {
        let base = addr.wrapping_add(row);
        print!("{base:04X}:");

        for col in 0..16 {
            if row + col >= len {
                break;
            }
            print!(" {:02X}", cpu.clock.bus.read(base.wrapping_add(col)));
        }

        println!();
    }
}
