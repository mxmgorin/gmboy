use crate::config::DesktopEmuConfig;
use crate::ui::App;
use core::emu::Emu;
use std::path::{Path, PathBuf};
use std::{env};

mod config;
mod video;
mod audio;
mod ui;

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
    let mut app = App::new(config).unwrap();

    if let Some(cart_path) = cart_path {
        if cart_path.exists() {
            emu.load_cart_file(&cart_path, app.config.save_state_on_exit);
        }
    } else if let Some(cart_path) = &app.config.last_cart_path {
        let cart_path = PathBuf::from(cart_path.clone());

        if cart_path.exists() {
            emu.load_cart_file(&cart_path, app.config.save_state_on_exit);
        }
    }

    if let Err(err) = run_emu(&mut emu, &mut app) {
        eprintln!("Failed run_emu: {err}");
    }

    if let Some(cart_path) = &app.config.last_cart_path {
        if let Err(err) = emu.save_files(Path::new(cart_path), app.config.save_state_on_exit) {
            eprint!("Failed save_files: {err}");
        }
    }

    app.config.set_emulation(emu.config);

    if let Err(err) = app.config.save_file().map_err(|e| e.to_string()) {
        eprint!("Failed config.save: {err}");
    }
}

fn run_emu(emu: &mut Emu, app: &mut App) -> Result<(), String> {
    while app.handle_events(emu) {
        if !emu.run_frame(app)? {
            let text = if app.config.last_cart_path.is_none() {
                "DROP FILE"
            } else {
                "PAUSED"
            };
            app.draw_text(text);
            continue;
        }

        emu.push_rewind();
        app.draw_debug(emu.runtime.bus.video_ram.iter_tiles());
    }

    Ok(())
}
