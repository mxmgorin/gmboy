use crate::auxiliary::clock::Clock;
use crate::cpu::instructions::{FetchedData};
use crate::cpu::interrupts::InterruptType;
use crate::cpu::Registers;
use serde::{Deserialize, Serialize};

pub const CPU_CLOCK_SPEED: u32 = 4194304;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct StepCtx {
    pub opcode: u8,
    pub fetched_data: FetchedData,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Cpu {
    pub registers: Registers,
    pub enabling_ime: bool,
    pub is_halted: bool,
    pub clock: Clock,
    pub step_ctx: StepCtx,
}

impl Cpu {
    pub fn new(clock: Clock) -> Self {
        Self {
            registers: Default::default(),
            enabling_ime: false,
            step_ctx: StepCtx::default(),
            is_halted: false,
            clock,
        }
    }

    /// Costs 2 M-Cycles with push PC
    #[inline(always)]
    pub fn goto_addr_with_cond<const C: u8>(&mut self, addr: u16, push_pc: bool) {
        if self.check_cond::<C>() {
            self.goto_addr(addr, push_pc);
        }
    }

    #[inline(always)]
    pub fn goto_addr(&mut self, addr: u16, push_pc: bool) {
        self.clock.tick_m_cycles(1); // internal: branch decision?

        if push_pc {
            self.push16(self.registers.pc);
        }

        self.registers.pc = addr;
    }

    /// Reads 8bit immediate data by PC and increments PC + 1. Costs 1 M-Cycle.
    #[inline]
    pub fn read_pc(&mut self) -> u8 {
        let value = self.clock.bus.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        self.clock.tick_m_cycles(1);

        value
    }

    /// Reads 16bit immediate data by PC and increments PC + 2. Costs 1 M-Cycle.
    #[inline]
    pub fn read_pc16(&mut self) -> u16 {
        u16::from_le_bytes([self.read_pc(), self.read_pc()])
    }

    /// Reads data from memory. Costs 1 M-Cycle.
    #[inline]
    pub fn read_memory(&mut self, address: u16) -> u8 {
        let value = self.clock.bus.read(address);
        self.clock.tick_m_cycles(1);

        value
    }

    /// Writes to memory. Costs 1 M-Cycle.
    #[inline]
    pub fn write_to_memory(&mut self, address: u16, value: u8) {
        self.clock.bus.write(address, value);
        self.clock.tick_m_cycles(1);
    }

    pub fn step(&mut self, mut _debugger: Option<&mut crate::debugger::Debugger>) {
        #[cfg(any(feature = "debug", debug_assertions))]
        if let Some(ref mut debugger) = _debugger {
            debugger.print(self);
            debugger.update_serial(&mut self.clock.bus);
        }

        self.handle_interrupts();

        if self.is_halted {
            if !self.clock.bus.io.interrupts.ime && self.clock.bus.io.interrupts.any_is_pending() {
                // HALT bug: continue executing instructions
                self.is_halted = false;
            }

            // Do nothing, just wait for an interrupt to wake up
            self.clock.tick_m_cycles(1);

            return;
        }

        self.step_ctx.opcode = self.read_pc();
        let prev_enabling_ime = self.enabling_ime;
        self.execute_opcode();

        if self.enabling_ime && prev_enabling_ime {
            // execute after next instruction when flag is changed
            self.enabling_ime = false;
            self.clock.bus.io.interrupts.ime = true;
        }
    }

    /// Costs 5 M-cycles when an interrupt is executed
    #[inline]
    pub fn handle_interrupts(&mut self) {
        if self.clock.bus.io.interrupts.ime {
            for (it, handler) in INTERRUPT_HANDLERS {
                if self.clock.bus.io.interrupts.is_pending(it) {
                    handler(self);
                    break;
                }
            }

            self.enabling_ime = false;
        }
    }

    #[inline]
    fn handle_interrupt(&mut self, addr: u16, it: InterruptType) {
        self.clock.tick_m_cycles(2);

        self.is_halted = false;
        self.clock.bus.io.interrupts.acknowledge_interrupt(it);
        self.goto_addr(addr, true);

        self.clock.tick_m_cycles(1);
    }
}

pub const INTERRUPT_HANDLERS: [(InterruptType, fn(&mut Cpu)); 5] = [
    (InterruptType::VBlank, |cpu| {
        cpu.handle_interrupt(0x40, InterruptType::VBlank)
    }),
    (InterruptType::LCDStat, |cpu| {
        cpu.handle_interrupt(0x48, InterruptType::LCDStat)
    }),
    (InterruptType::Timer, |cpu| {
        cpu.handle_interrupt(0x50, InterruptType::Timer)
    }),
    (InterruptType::Serial, |cpu| {
        cpu.handle_interrupt(0x58, InterruptType::Serial)
    }),
    (InterruptType::Joypad, |cpu| {
        cpu.handle_interrupt(0x60, InterruptType::Joypad)
    }),
];
