use crate::core::cpu::Cpu;
use crate::core::stack::Stack;

const INTERRUPTS_BY_ADDRESSES: [(u16, InterruptType); 5] = [
    (0x40, InterruptType::VBlank),
    (0x48, InterruptType::LCDStat),
    (0x50, InterruptType::Timer),
    (0x58, InterruptType::Serial),
    (0x60, InterruptType::Joypad),
];

pub enum Interrupts {}

impl Interrupts {
    pub fn handle(cpu: &mut Cpu) {
        for (address, interrupt_type) in INTERRUPTS_BY_ADDRESSES {
            if Self::check_interrupt(cpu, address, interrupt_type) {
                break;
            }
        }
    }

    fn handle_interrupt(cpu: &mut Cpu, address: u16) {
        Stack::push16(cpu, address);
        cpu.registers.pc = address;
    }

    fn check_interrupt(cpu: &mut Cpu, address: u16, it: InterruptType) -> bool {
        let it = it as u8;

        if (cpu.bus.io.int_flags & it != 0) && (cpu.bus.ie_register & it != 0) {
            Self::handle_interrupt(cpu, address);
            cpu.bus.io.int_flags &= !it;
            cpu.halted = false;
            cpu.int_master_enabled = false;

            return true;
        }

        false
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptType {
    VBlank = 1,
    LCDStat = 2,
    Timer = 3,
    Serial = 4,
    Joypad = 5,
}
