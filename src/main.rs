use gmboy::config::Config;
use gmboy::emu::Emu;
use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    let cart_path = if args.len() < 2 {
        env::var("CART_PATH").ok()
    } else {
        Some(args.remove(1))
    };

    let config_path = Config::default_path();

    let config = if config_path.exists() {
        Config::from_file(config_path.to_str().unwrap()).expect("Failed to parse save/config.json")
    } else {
        eprintln!("config.json not found in the save folder");
        std::process::exit(1);
    };

    let mut emu = Emu::new(config).unwrap();

    if let Err(err) = emu.run(cart_path) {
        eprintln!("Emu run failed: {}", err);
        std::process::exit(1);
    }
}
