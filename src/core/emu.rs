use crate::core::cart::Cart;
use crate::core::cpu::Cpu;
use std::thread;

#[derive(Debug)]
pub struct Emu {
    cart: Cart,
    cpu: Cpu,
    running: bool,
    paused: bool,
    ticks: usize,
}

impl Emu {
    pub fn new(cart_bytes: Vec<u8>) -> Result<Self, String> {
        let cart = Cart::new(cart_bytes)?;

        println!("Cart: {:?}", cart);

        Ok(Self {
            cart,
            cpu: Cpu::new(),
            running: false,
            paused: false,
            ticks: 0,
        })
    }

    pub fn run(&mut self) -> Result<(), String> {
        self.running = true;

        while self.running {
            if self.paused {
                thread::sleep(std::time::Duration::from_millis(50));
                continue;
            }

            self.cpu.step()?;
            self.ticks += 1;
        }

        Ok(())
    }
}
