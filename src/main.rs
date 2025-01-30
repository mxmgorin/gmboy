use crate::core::emu::Emu;
use std::path::Path;
use std::{env, fs};

mod core;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <cart_path>", args[0]);
        std::process::exit(1);
    }

    let cart_path = &args[1];
    println!("Cart path provided: {}", cart_path);
    let result = read_bytes(cart_path);

    let Ok(cart_bytes) = result else {
        eprintln!("Failed to read cart: {}", result.unwrap_err());
        std::process::exit(1);
    };

    let result = Emu::new(cart_bytes);

    let Ok(mut emu) = result else {
        eprintln!("Emu failed: {}", result.unwrap_err());
        std::process::exit(1);
    };

    emu.run();
}

fn read_bytes(file_path: &str) -> Result<Vec<u8>, String> {
    // Check if the file exists and is readable
    if !Path::new(file_path).exists() {
        return Err(format!("File not found: {}", file_path));
    }

    // Read the file as bytes
    fs::read(file_path).map_err(|e| format!("Failed to read file: {}", e))
}
