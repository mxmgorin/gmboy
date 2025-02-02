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
        if cpu.bus.io.serial.has_data() {
            self.msg[self.size] = cpu.bus.io.serial.take_data();
            self.size += 1;
        }
    }

    pub fn print(&self) {
        if self.msg[0] != 0 {
            let msg_str = String::from_utf8_lossy(&self.msg[..self.size]);
            println!("DBG: {:?}", msg_str);
        }
    }
}
