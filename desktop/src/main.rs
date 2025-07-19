use crate::config::DesktopEmuConfig;
use crate::ui::Ui;
use core::emu::Emu;
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::{env, thread};

mod config;
pub mod ui;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    let cart_path = if args.len() < 2 {
        env::var("CART_PATH").ok()
    } else {
        Some(args.remove(1))
    }
    .map(PathBuf::from);

    let config_path = DesktopEmuConfig::default_path();

    let config = if config_path.exists() {
        DesktopEmuConfig::from_file(config_path.to_str().unwrap())
            .unwrap_or_else(|_| panic!("Failed to parse {config_path:?}"))
    } else {
        let config = DesktopEmuConfig::default();

        if let Err(err) = config.save_file() {
            eprintln!("failed to create default config: {err}");
            std::process::exit(1);
        }

        config
    };

    let mut emu = Emu::new(
        config.clone_emulation(),
        config.graphics.get_current_pallet(),
        None,
    )
    .unwrap();
    let mut ui = Ui::new(config, false).unwrap();

    if let Some(cart_path) = cart_path {
        if cart_path.exists() {
            emu.load_cart_file(&cart_path, ui.config.load_save_state_at_start);
        }
    } else if let Some(cart_path) = &ui.config.last_cart_path {
        let cart_path = PathBuf::from(cart_path.clone());

        if cart_path.exists() {
            emu.load_cart_file(&cart_path, ui.config.load_save_state_at_start);
        }
    }

    if let Err(err) = run_emu(&mut emu, &mut ui) {
        eprintln!("Failed run_emu: {err}");
    }

    if let Some(cart_path) = &ui.config.last_cart_path {
        if let Err(err) = emu.save_files(Path::new(cart_path)) {
            eprint!("Failed save_files: {err}");
        }
    }

    ui.config.set_emulation(emu.config);

    if let Err(err) = ui.config.save_file().map_err(|e| e.to_string()) {
        eprint!("Failed config.save: {err}");
    }
}

fn run_emu(emu: &mut Emu, ui: &mut Ui) -> Result<(), String> {
    while ui.handle_events(emu) {
        if !emu.run_frame(ui)? {
            let text = if ui.config.last_cart_path.is_none() {
                "DROP FILE"
            } else {
                "PAUSED"
            };
            ui.draw_text(text);
            thread::sleep(Duration::from_millis(100));
            continue;
        }

        emu.push_rewind();
        ui.draw_debug(emu.runtime.bus.video_ram.iter_tiles());
    }

    Ok(())
}
