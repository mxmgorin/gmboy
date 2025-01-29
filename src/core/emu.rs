use crate::core::cart::Cart;
use crate::core::cpu::Cpu;

#[derive(Debug)]
pub struct Emu {
    cart: Cart,
    cpu: Cpu,
    running: bool,
    paused: bool,
    ticks: usize,
}

impl Emu {
    pub fn new(cart_bytes: &[u8]) -> Result<Self, String> {
        Ok(Self {
            cart: Cart::from_bytes(cart_bytes)?,
            cpu: Cpu::new(),
            running: false,
            paused: false,
            ticks: 0,
        })
    }

    pub fn run(&mut self) {
        self.running = true;

        while self.running {
            if self.paused {
                // todo: add pause
                continue;
            }

            self.ticks += 1;
        }
    }
}
