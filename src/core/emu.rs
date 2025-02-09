use crate::bus::Bus;
use crate::core::cart::Cart;
use crate::core::cpu::Cpu;
use crate::core::ui::Ui;
use crate::debugger::{CpuLogType, Debugger};
use std::path::Path;
use std::{fs, thread};

#[derive(Debug)]
pub struct Emu {
    cpu: Cpu,
    running: bool,
    paused: bool,
}

pub trait EventHandler {
    fn on_quit(&mut self);
}

impl EventHandler for Emu {
    fn on_quit(&mut self) {
        self.running = false;
    }
}

impl Emu {
    pub fn new(cart_bytes: Vec<u8>) -> Result<Self, String> {
        let cart = Cart::new(cart_bytes)?;

        Ok(Self {
            cpu: Cpu::new(Bus::new(cart)),
            running: false,
            paused: false,
        })
    }

    pub fn run(&mut self) -> Result<(), String> {
        let mut ui = Ui::new()?;
        self.running = true;
        let serial_enabled = true;
        let mut debugger = Debugger::new(CpuLogType::None, serial_enabled);

        while self.running {
            if self.paused {
                thread::sleep(std::time::Duration::from_millis(50));
                continue;
            }

            self.cpu.step(Some(&mut debugger))?;
            let mut ui_update = false;

            if serial_enabled {
                let msg = debugger.get_serial_msg();
                
                if !msg.is_empty() {
                    ui_update = true;
                    println!("{msg}");
                }
            }

            ui.handle_events(self);
            
            if ui_update {
                let tiles = self.cpu.bus.ppu.get_tiles();
                println!("{:?}", tiles);
                ui.update(&self.cpu.bus);
            }
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
