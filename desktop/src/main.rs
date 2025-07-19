use crate::ui::Ui;
use core::emu::config::EmuConfig;
use core::emu::ctx::EmuState;
use core::emu::Emu;
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::{env, thread};

pub mod ui;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    let cart_path = if args.len() < 2 {
        env::var("CART_PATH").ok()
    } else {
        Some(args.remove(1))
    };

    let config_path = EmuConfig::default_path();

    let config = if config_path.exists() {
        EmuConfig::from_file(config_path.to_str().unwrap())
            .unwrap_or_else(|_| panic!("Failed to parse {config_path:?}"))
    } else {
        let config = EmuConfig::default();

        if let Err(err) = config.save_file() {
            eprintln!("failed to create default config: {err}");
            std::process::exit(1);
        }

        config
    };

    let mut ui = Ui::new(config.graphics.clone(), false).unwrap();
    let mut emu = Emu::new(config).unwrap();

    if let Err(err) = run_emu(&mut emu, &mut ui, cart_path.map(|x| x.into())) {
        eprintln!("Run failed: {err}");
    }

    emu.save_files();
}

fn run_emu(emu: &mut Emu, ui: &mut Ui, cart_path: Option<PathBuf>) -> Result<(), String> {
    if let Some(cart_path) = &emu.ctx.config.last_cart_path {
        if Path::new(cart_path).exists() {
            emu.load_cart_file(cart_path.into());
        }
    }

    if let Some(cart_path) = cart_path {
        emu.load_cart_file(cart_path);
    }

    loop {
        ui.handle_events(emu);

        if emu.ctx.state == EmuState::Paused {
            let text = if emu.ctx.config.last_cart_path.is_none() {
                "DROP FILE"
            } else {
                "PAUSED"
            };
            ui.draw_text(text, emu.ctx.config.graphics.text_scale);
            thread::sleep(Duration::from_millis(100));
            continue;
        }

        if ui.quit {
            break;
        }

        emu.handle_state();
        emu.run_frame(ui)?;
        emu.push_rewind();
        ui.draw_debug(emu.cpu.bus.video_ram.iter_tiles());
    }

    Ok(())
}
