use crate::app::App;
use crate::config::AppConfig;
use crate::input::InputHandler;
use core::emu::Emu;
use std::env;
use std::path::PathBuf;

mod app;
mod audio;
mod config;
mod input;
mod video;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = get_config();
    let emu_config = config.get_emu();
    let emu_palette = config.interface.get_current_palette();
    let mut emu = Emu::new(emu_config.clone(), emu_palette, None).unwrap();
    load_cart(&config, &mut emu, args);
    let mut sdl = sdl2::init().unwrap();
    let mut app = App::new(&mut sdl, config).unwrap();
    let mut input = InputHandler::new(&sdl).unwrap();

    if let Err(err) = app.run(&mut emu, &mut input) {
        eprintln!("Failed app run: {err}");
    }

    app.save_files(&mut emu).unwrap()
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
