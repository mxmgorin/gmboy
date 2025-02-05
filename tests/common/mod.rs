use rusty_gb_emu::bus::ram::Ram;
use rusty_gb_emu::bus::Bus;
use rusty_gb_emu::cart::Cart;
use rusty_gb_emu::cpu::Cpu;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Sm83TestCase {
    pub name: String,
    #[serde(rename = "initial")]
    pub initial_state: CpuState,
    #[serde(rename = "final")]
    pub final_state: CpuState,
    pub cycles: Vec<Cycle>,
}

impl Sm83TestCase {
    pub fn from_json(json: &str) -> Sm83TestCase {
        serde_json::from_str(json).unwrap()
    }
    
    pub fn assert_final_state(&self, cpu: &Cpu) {
        assert_eq!(cpu.registers.a, self.final_state.a);
        assert_eq!(cpu.registers.b, self.final_state.b);
        assert_eq!(cpu.registers.c, self.final_state.c);
        assert_eq!(cpu.registers.d, self.final_state.d);
        assert_eq!(cpu.registers.e, self.final_state.e);
        assert_eq!(cpu.registers.flags.byte, self.final_state.f);
        assert_eq!(cpu.registers.h, self.final_state.h);
        assert_eq!(cpu.registers.l, self.final_state.l);
        assert_eq!(cpu.registers.sp, self.final_state.sp);
        assert_eq!(cpu.registers.pc, self.final_state.pc);
        assert_eq!(cpu.enabling_ime, self.final_state.ime != 0);

        assert_eq!(cpu.bus.io.interrupts.ie_register, self.final_state.ie);

        // Assert RAM contents
        for ram in self.final_state.ram.iter() {
            assert_eq!(cpu.bus.read(ram.0), ram.1);
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CpuState {
    pub pc: u16,
    pub sp: u16,
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub ime: u8,
    #[serde(default)]
    pub ie: u8,
    pub ram: Vec<RamState>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RamState(pub u16, pub u8);

#[derive(Debug, Serialize, Deserialize)]
pub struct Cycle(pub u16, pub u8, pub String);

pub fn set_up_cpu(test_case: &Sm83TestCase) -> Cpu {
    let mut cpu = Cpu::new(set_up_bus(test_case));
    set_cpu_state(&mut cpu, test_case);

    cpu
}

pub fn set_up_bus(test_case: &Sm83TestCase) -> Bus {
    let mut bytes = vec![0u8; 40000];
    for ram in test_case.initial_state.ram.iter() {
        bytes[ram.0 as usize] = ram.1;
    }

    Bus::new(Cart::new(bytes).unwrap(), Ram::new())
}

pub fn set_cpu_state(cpu: &mut Cpu, test_case: &Sm83TestCase) {
    cpu.registers.pc = test_case.initial_state.pc;
    cpu.registers.sp = test_case.initial_state.sp;
    cpu.registers.a = test_case.initial_state.a;
    cpu.registers.b = test_case.initial_state.b;
    cpu.registers.c = test_case.initial_state.c;
    cpu.registers.d = test_case.initial_state.d;
    cpu.registers.e = test_case.initial_state.e;
    cpu.registers.h = test_case.initial_state.h;
    cpu.registers.l = test_case.initial_state.l;
    cpu.registers.flags.byte = test_case.initial_state.f;
    cpu.bus.io.interrupts.ie_register = test_case.initial_state.ie;
    cpu.enabling_ime = test_case.initial_state.ime != 0
}
