use rusty_gb_emu::emu::Emu;
use std::path::Path;
use std::{env, fs};

fn main() {
    let mut args: Vec<String> = env::args().collect();

    let cart_path = if args.len() < 2 {
        if let Ok(cart_path) = env::var("CART_PATH") {
            cart_path
        } else {
            eprintln!("Usage: {} <cart_path>", args[0]);
            std::process::exit(1);
        }
    } else {
        args.remove(1)
    };

    let emu = Emu::load_cart(&cart_path);

    let Ok(mut emu) = emu else {
        eprintln!("Emu load_cart failed: {}", emu.unwrap_err());
        std::process::exit(1);
    };

    if let Err(err) = emu.run() {
        eprintln!("Emu run failed: {}", err);
        std::process::exit(1);
    }
}
