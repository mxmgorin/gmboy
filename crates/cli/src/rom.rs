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

/// True if `rel` (a ROM path relative to the scanned dir) matches any exclude
/// glob. Patterns use `*` (any run, including `/`) and `?` (one char), e.g.
/// `interrupt_time.gb` or `manual-only/*`.
pub fn is_excluded(rel: &Path, excludes: &[String]) -> bool {
    let rel = rel.to_string_lossy().replace('\\', "/");
    excludes.iter().any(|g| glob_match(g, &rel))
}

/// Minimal wildcard match (`*` = any run incl. `/`, `?` = one char) — enough for
/// the exclude patterns above, without pulling in a glob crate.
pub fn glob_match(pattern: &str, text: &str) -> bool {
    let p: Vec<char> = pattern.chars().collect();
    let t: Vec<char> = text.chars().collect();
    let (mut pi, mut ti) = (0, 0);
    let (mut star, mut mark) = (None, 0);

    while ti < t.len() {
        if pi < p.len() && (p[pi] == '?' || p[pi] == t[ti]) {
            pi += 1;
            ti += 1;
        } else if pi < p.len() && p[pi] == '*' {
            star = Some(pi);
            mark = ti;
            pi += 1;
        } else if let Some(s) = star {
            pi = s + 1;
            mark += 1;
            ti = mark;
        } else {
            return false;
        }
    }
    while pi < p.len() && p[pi] == '*' {
        pi += 1;
    }
    pi == p.len()
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

/// Compare the CPU's LCD framebuffer against a reference PNG, allowing each RGB
/// channel to differ by up to `tolerance` (for color-conversion rounding;
/// a real pixel mismatch differs by far more)
pub fn compare_to_reference(cpu: &Cpu, ref_path: &Path, tolerance: u8) -> Result<(), String> {
    let got = cpu.clock.bus.io.ppu.lcd.buffer.rgb888();
    let want = image::open(ref_path)
        .map_err(|e| e.to_string())?
        .to_rgb8()
        .into_raw();

    if got.len() != want.len() {
        return Err(format!(
            "reference size mismatch: got {} bytes, want {}",
            got.len(),
            want.len()
        ));
    }

    let mut mismatches = 0usize;
    let mut first: Option<(usize, u8, u8)> = None;
    for (i, (&g, &w)) in got.iter().zip(want.iter()).enumerate() {
        if g.abs_diff(w) > tolerance {
            mismatches += 1;
            first.get_or_insert((i, g, w));
        }
    }

    match first {
        None => Ok(()),
        Some((i, g, w)) => {
            let px = i / 3;
            Err(format!(
                "{mismatches}/{} bytes differ (tol {tolerance}); first at px ({},{}) got {g} want {w}",
                got.len(),
                px % (LCD_X_RES as usize),
                px / (LCD_X_RES as usize),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::glob_match;

    #[test]
    fn literal_and_question() {
        assert!(glob_match("reg_f.gb", "reg_f.gb"));
        assert!(glob_match("reg_?.gb", "reg_f.gb"));
        assert!(!glob_match("reg_f.gb", "reg_f.gbc"));
        assert!(!glob_match("reg_?.gb", "reg_ff.gb"));
    }

    #[test]
    fn star_crosses_separators() {
        assert!(glob_match("manual-only/*", "manual-only/sprite_priority.gb"));
        assert!(glob_match("*-dmg0.gb", "boot_regs-dmg0.gb"));
        assert!(glob_match("*div*", "acceptance/timer/div_write.gb"));
        assert!(!glob_match("manual-only/*", "acceptance/manual-only.gb"));
    }

    #[test]
    fn star_backtracking() {
        assert!(glob_match("*.gb", "a.gb"));
        assert!(glob_match("a*b*c", "a_x_b_y_c"));
        assert!(!glob_match("a*b*c", "a_x_b_y"));
        assert!(glob_match("*", ""));
        assert!(!glob_match("?", ""));
    }
}
