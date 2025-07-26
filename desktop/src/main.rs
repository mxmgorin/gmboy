use crate::app::App;
use crate::config::AppConfig;
use core::apu::Apu;
use core::auxiliary::io::Io;
use core::bus::Bus;
use core::cart::Cart;
use core::emu::runtime::EmuRuntime;
use core::emu::Emu;
use core::ppu::lcd::Lcd;
use core::ppu::palette::LcdPalette;
use core::ppu::Ppu;
use std::env;
use std::path::PathBuf;
use crate::input::handler::InputHandler;

mod app;
mod audio;
mod config;
mod input;
mod video;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = get_config();
    let palettes = get_palettes();

    let mut emu = new_emu(&config, &palettes);
    load_cart(&config, &mut emu, args);

    let mut sdl = sdl2::init().unwrap();
    let mut app = App::new(&mut sdl, config, palettes).unwrap();
    let mut input = InputHandler::new(&sdl).unwrap();

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
    let colors = config.interface.get_current_palette(palettes);

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

fn load_cart(config: &AppConfig, emu: &mut Emu, mut args: Vec<String>) {
    let cart_path = if args.len() < 2 {
        env::var("CART_PATH").ok()
    } else {
        Some(args.remove(1))
    }
    .map(PathBuf::from);

    if let Some(cart_path) = cart_path {
        if cart_path.exists() {
            emu.load_cart_file(&cart_path, config.save_state_on_exit);
        }
    } else if let Some(cart_path) = &config.last_cart_path {
        let cart_path = PathBuf::from(cart_path.clone());

        if cart_path.exists() {
            emu.load_cart_file(&cart_path, config.save_state_on_exit);
        }
    }
}

fn get_config() -> AppConfig {
    let config_path = AppConfig::default_path();

    if config_path.exists() {
        AppConfig::from_file(config_path.to_str().unwrap())
            .unwrap_or_else(|_| panic!("Failed to parse {config_path:?}"))
    } else {
        let config = AppConfig::default();

        if let Err(err) = config.save_file() {
            eprintln!("Failed to create default config: {err}");
            std::process::exit(1);
        }

        config
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
