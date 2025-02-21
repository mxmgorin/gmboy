use mgboy::emu::Emu;
use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    let cart_path = if args.len() < 2 {
        env::var("CART_PATH").ok()
    } else {
        Some(args.remove(1))
    };

    let result = Emu::new();

    let Ok(mut emu) = result else {
        eprintln!("Emu failed: {}", result.unwrap_err());
        std::process::exit(1);
    };

    if let Err(err) = emu.run(cart_path) {
        eprintln!("Emu run failed: {}", err);
        std::process::exit(1);
    }
}
