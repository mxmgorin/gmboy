use crate::app::App;
use crate::config::AppConfig;
use crate::input::handler::InputHandler;
use crate::library::RomsLibrary;
use core::apu::Apu;
use core::auxiliary::io::Io;
use core::bus::Bus;
use core::cart::Cart;
use core::emu::runtime::EmuRuntime;
use core::emu::Emu;
use core::ppu::lcd::Lcd;
use core::ppu::palette::LcdPalette;
use core::ppu::Ppu;
use std::path::PathBuf;
use std::{env, fs};

mod app;
mod audio;
mod config;
mod input;
mod library;
mod video;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = get_config();
    let palettes = get_palettes();
    let mut emu = new_emu(&config, &palettes);
    let mut sdl = sdl2::init().unwrap();
    let mut input = InputHandler::new(&sdl, &config.input).unwrap();
    let mut app = App::new(&mut sdl, config, palettes).unwrap();
    load_cart(&mut app, &mut emu, args);

    if let Err(err) = app.run(&mut emu, &mut input) {
        eprintln!("Failed app run: {err}");
    }

    if let Err(err) = app.save_files(&mut emu) {
        eprintln!("Failed app.save_files: {err}");
    }
}

fn new_emu(config: &AppConfig, palettes: &[LcdPalette]) -> Emu {
    let emu_config = config.get_emu_config();
    let apu_config = config.audio.get_apu_config();
    let colors = config.interface.get_palette_colors(palettes);

    let lcd = Lcd::new(colors);
    let apu = Apu::new(apu_config);
    let bus = Bus::new(Cart::empty(), Io::new(lcd, apu));
    let mut ppu = Ppu::default();

    if config.interface.show_fps {
        ppu.toggle_fps();
    }

    let debugger = None;
    let runtime = EmuRuntime::new(ppu, bus, debugger);

    Emu::new(emu_config.clone(), runtime).unwrap()
}

fn load_cart(app: &mut App, emu: &mut Emu, mut args: Vec<String>) {
    let cart_path = if args.len() < 2 {
        env::var("CART_PATH").ok()
    } else {
        Some(args.remove(1))
    }
    .map(PathBuf::from);

    if let Some(cart_path) = cart_path {
        if cart_path.exists() {
            app.load_cart_file(emu, &cart_path);
        }
    } else {
        let library = RomsLibrary::get_or_create();

        if let Some(cart_path) = library.get_last_path() {
            let cart_path = PathBuf::from(cart_path.clone());

            if cart_path.exists() {
                app.load_cart_file(emu, &cart_path);
            }
        }
    }
}

fn get_config() -> AppConfig {
    let config_path = AppConfig::default_path();

    if config_path.exists() {
        let config = AppConfig::from_file(&config_path);

        let Ok(config) = config else {
            eprintln!("Failed to parse config file: {}", config.unwrap_err());

            let backup_path = config_path.with_file_name(format!(
                "{}.bak",
                config_path.file_name().unwrap().to_string_lossy()
            ));
            if let Err(rename_err) = fs::rename(config_path, &backup_path) {
                eprintln!("Failed to rename invalid config file: {rename_err}");
            } else {
                eprintln!("Renamed invalid config to {:?}", backup_path);
            }

            let default_config = AppConfig::default();

            if let Err(save_err) = default_config.save_file() {
                panic!("Failed to save default config: {save_err}");
            }

            return default_config;
        };

        config
    } else {
        let default_config = AppConfig::default();

        if let Err(err) = default_config.save_file() {
            panic!("Failed to create default config: {err}");
        }

        default_config
    }
}

fn get_palettes() -> Box<[LcdPalette]> {
    let path = LcdPalette::default_palettes_path();

    if path.exists() {
        core::read_json_file(&path).unwrap()
    } else {
        let palettes = LcdPalette::default_palettes().into_boxed_slice();
        LcdPalette::save_palettes_file(&palettes).unwrap();

        palettes
    }
}
