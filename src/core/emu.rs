use crate::core::bus::Bus;
use crate::core::cart::Cart;
use crate::core::cpu::Cpu;
use crate::core::ram::Ram;
use std::thread;
use crate::core::debugger::Debugger;

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

        println!("Cart Loaded:");
        println!("\t Title    : {}", cart.header.title);
        println!("\t Type     : {:?}", cart.header.cart_type);
        println!("\t ROM Size : {:?}", cart.header.rom_size);
        println!("\t RAM Size : {:?}", cart.header.ram_size);
        println!("\t LIC Code : {:?} ", cart.header.new_licensee_code);
        println!("\t ROM Version : {:02X}", cart.header.mask_rom_version);

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
        let mut debugger = Some(Debugger::new());
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
}
