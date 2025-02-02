use crate::core::cpu::Cpu;

#[derive(Debug, Clone)]
pub struct Debugger {
    msg: [u8; 1024],
    size: usize,
}

impl Debugger {
    pub fn new() -> Self {
        Debugger {
            msg: [0; 1024],
            size: 0,
        }
    }

    pub fn update(&mut self, cpu: &mut Cpu) {
        if cpu.bus.read(0xFF02) == 0x81 {
            let letter = cpu.bus.read(0xFF01);
            self.msg[self.size] = letter;
            self.size += 1;
            cpu.bus.write(0xFF02, 0);
        }
    }

    pub fn print(&self) {
        if self.msg[0] != 0 {
            let msg_str = String::from_utf8_lossy(&self.msg[..self.size]);
            println!("DBG: {:?}", msg_str);
        }
    }
}
