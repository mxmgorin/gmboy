#[derive(Debug)]
pub struct Cpu {
    registers: CpuRegisters,
}

impl Cpu {
    pub fn new() -> Cpu {
        Self {
            registers: CpuRegisters::new(),
        }
    }

    pub fn step(&mut self) -> Result<(), String> {
        Err("cpu step not implemented yet".to_string())
    }
}

#[derive(Debug, Clone)]
pub struct CpuRegisters {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

impl CpuRegisters {
    pub fn new() -> Self {
        Self {
            a: 0x01,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0x100,
        }
    }
}
