use core::cpu::Cpu;
use crate::ui::Ui;
use core::emu::battery::BatterySave;
use core::bus::Bus;
use core::emu::config::EmuConfig;
use core::emu::ctx::{EmuState, RunMode};
use core::emu::save_state::{EmuSaveState, SaveStateEvent};
use core::emu::{load_save_state, read_cart, Emu};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
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

        if let Err(err) = config.save() {
            eprintln!("failed to create default config: {}", err);
            std::process::exit(1);
        }

        config
    };

    let mut ui = Ui::new(config.graphics.clone(), false).unwrap();
    let mut emu = Emu::new(config).unwrap();

    if let Err(err) = run(&mut emu, &mut ui, cart_path.map(|x| x.into())) {
        eprintln!("Run failed: {}", err);
        std::process::exit(1);
    }
}

fn run(emu: &mut Emu, ui: &mut Ui, cart_path: Option<PathBuf>) -> Result<(), String> {
    if let Some(cart_path) = &emu.ctx.config.last_cart_path {
        if Path::new(cart_path).exists() {
            emu.ctx.state = EmuState::LoadCart(cart_path.into());
        }
    }

    if let Some(cart_path) = cart_path {
        emu.ctx.state = EmuState::LoadCart(cart_path);
    }

    loop {
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
            emu.ctx.config.save().map_err(|e| e.to_string())?;
            break;
        }

        if let EmuState::LoadCart(path) = &emu.ctx.state {
            let cart = read_cart(path).map_err(|e| e.to_string())?;

            let mut bus = Bus::new(cart);
            bus.io.lcd.set_pallet(ui.curr_palette);
            emu.cpu = Cpu::new(bus);

            emu.ctx.config.last_cart_path = Some(path.to_string_lossy().to_string());
            emu.ctx.state = EmuState::Running(RunMode::Normal);
            emu.ctx.reset();

            if emu.ctx.config.load_save_state_at_start {
                let name = emu.ctx.config.get_last_cart_file_stem().unwrap();
                let save_state = EmuSaveState::load_file(&name, 0);

                if let Ok(save_state) = save_state {
                    load_save_state(emu, save_state);
                } else {
                    eprintln!("Failed load save_state: {:?}", save_state);
                };
            }
        }

        if let EmuState::Running(RunMode::Rewind) = &emu.ctx.state {
            if let Some(state) = emu.ctx.rewind_buffer.pop_back() {
                load_save_state(emu, state);
                thread::sleep(Duration::from_millis(100));
            }
        }

        ui.handle_events(&mut emu.cpu.bus, &mut emu.ctx);
        emu.tick(ui)?;

        if emu.ctx.prev_frame != emu.ctx.ppu.current_frame {
            ui.draw_debug(emu.cpu.bus.video_ram.iter_tiles());
        }

        let now = Instant::now();
        if emu.ctx.config.emulation.rewind_size > 0
            && now.duration_since(emu.ctx.last_rewind_save).as_secs_f32() >= 2.0
        {
            if emu.ctx.rewind_buffer.len() > emu.ctx.config.emulation.rewind_size {
                emu.ctx.rewind_buffer.pop_front();
            }

            emu.ctx
                .rewind_buffer
                .push_back(emu.create_save_state(&emu.cpu));
            emu.ctx.last_rewind_save = now;
        }

        if let Some((event, index)) = emu.ctx.pending_save_state.take() {
            let name = emu.ctx.config.get_last_cart_file_stem().unwrap();

            match event {
                SaveStateEvent::Create => {
                    let save_state = emu.create_save_state(&emu.cpu);

                    if let Err(err) = save_state.save_file(&name, index) {
                        eprintln!("Failed save_state: {:?}", err);
                    }
                }
                SaveStateEvent::Load => {
                    let save_state = EmuSaveState::load_file(&name, index);

                    let Ok(save_state) = save_state else {
                        eprintln!("Failed load save_state: {:?}", save_state);
                        continue;
                    };

                    load_save_state(emu, save_state);
                }
            }
        }
    }

    let name = emu.ctx.config.get_last_cart_file_stem().unwrap();

    if let Some(bytes) = emu.cpu.bus.cart.dump_ram() {
        BatterySave::from_bytes(bytes)
            .save_file(&name)
            .map_err(|e| e.to_string())?;
    }

    if let Err(err) = emu.create_save_state(&emu.cpu).save_file(&name, 0) {
        eprintln!("Failed save_state: {:?}", err);
    }

    Ok(())
}
