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
            .expect(&format!("Failed to parse {:?}", config_path))
    } else {
        let config = EmuConfig::default();

        if let Err(err) = config.save_file() {
            eprintln!("failed to create default config: {}", err);
            std::process::exit(1);
        }

        config
    };

    let mut ui = Ui::new(config.graphics.clone(), false).unwrap();
    let mut emu = Emu::new(config).unwrap();

    if let Err(err) = run_emu(&mut emu, &mut ui, cart_path.map(|x| x.into())) {
        eprintln!("Run failed: {}", err);
    }

    emu.save_files();
}

fn run_emu(emu: &mut Emu, ui: &mut Ui, cart_path: Option<PathBuf>) -> Result<(), String> {
    if let Some(cart_path) = &emu.ctx.config.last_cart_path {
        if Path::new(cart_path).exists() {
            emu.ctx.state = EmuState::LoadCart(cart_path.into());
        }
    }

    if let Some(cart_path) = cart_path {
        emu.ctx.state = EmuState::LoadCart(cart_path);
    }

    loop {
        ui.handle_events(&mut emu.cpu.bus, &mut emu.ctx);

        if emu.ctx.state == EmuState::Paused || emu.ctx.state == EmuState::WaitCart {
            let text = if emu.ctx.state == EmuState::Paused {
                "PAUSED"
            } else {
                "DROP FILE"
            };
            ui.draw_text(text);
            ui.handle_events(&mut emu.cpu.bus, &mut emu.ctx);
            thread::sleep(Duration::from_millis(100));
            continue;
        }

        if emu.ctx.state == EmuState::Quit {
            break;
        }

        emu.handle_state(ui.curr_palette);
        emu.tick(ui)?;
        emu.tick_rewind();

        if emu.ctx.prev_frame != emu.ctx.ppu.current_frame {
            ui.draw_debug(emu.cpu.bus.video_ram.iter_tiles());
        }
    }

    Ok(())
}
