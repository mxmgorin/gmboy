use crate::core::bus::ram::Ram;
use crate::core::bus::Bus;
use crate::core::cart::Cart;
use crate::core::cpu::Cpu;
use crate::debugger::{CpuLogType, Debugger};
use std::path::Path;
use std::{fs, thread};

#[derive(Debug)]
pub struct Emu {
    cpu: Cpu,
    running: bool,
    paused: bool,
    ticks: usize,
}

impl Emu {
    pub fn new(cart_bytes: Vec<u8>) -> Result<Self, String> {
        let cart = Cart::new(cart_bytes)?;

        Ok(Self {
            cpu: Cpu::new(Bus::new(cart, Ram::new())),
            running: false,
            paused: false,
            ticks: 0,
        })
    }

    pub fn run(&mut self) -> Result<(), String> {
        self.running = true;
        #[cfg(debug_assertions)]
        let mut debugger = Some(Debugger::new(CpuLogType::Assembly, true));
        #[cfg(not(debug_assertions))]
        let mut debugger = None;

        while self.running {
            if self.paused {
                thread::sleep(std::time::Duration::from_millis(50));
                continue;
            }

            self.cpu.step(&mut debugger)?;
            self.ticks += 1;
        }

        Ok(())
    }

    pub fn load_cart(cart_path: &str) -> Result<Emu, String> {
        let result = read_bytes(cart_path);

        let Ok(cart_bytes) = result else {
            return Err(format!("Failed to read cart: {}", result.unwrap_err()));
        };

        let result = Emu::new(cart_bytes);

        let Ok(emu) = result else {
            return Err(format!("Emu failed: {}", result.unwrap_err()));
        };

        Ok(emu)
    }

    fn _print_cart(&self, cart: &Cart) {
        println!("Cart Loaded:");
        println!("\t Title    : {}", cart.header.title);
        println!("\t Type     : {:?}", cart.header.cart_type);
        println!("\t ROM Size : {:?}", cart.header.rom_size);
        println!("\t RAM Size : {:?}", cart.header.ram_size);
        println!("\t LIC Code : {:?} ", cart.header.new_licensee_code);
        println!("\t ROM Version : {:02X}", cart.header.mask_rom_version);
    }
}

pub fn read_bytes(file_path: &str) -> Result<Vec<u8>, String> {
    if !Path::new(file_path).exists() {
        return Err(format!("File not found: {}", file_path));
    }

    fs::read(file_path).map_err(|e| format!("Failed to read file: {}", e))
}
