use crate::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::instructions::{DataDestination, FetchedData};
use crate::cpu::{Cpu, CpuCallback};

#[derive(Debug, Clone, Copy)]
pub struct IncInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for IncInstruction {
    fn execute(&self, cpu: &mut Cpu, callback: &mut impl CpuCallback, fetched_data: FetchedData) {
        let mut value = fetched_data.value.wrapping_add(1);

        match fetched_data.dest {
            DataDestination::Register(r) => {
                if r.is_16bit() {
                    callback.m_cycles(1);
                }

                cpu.registers.set_register(r, value);
                value = cpu.registers.read_register(r);
            }
            DataDestination::Memory(addr) => {
                // uses only HL
                value &= 0xFF; // Ensure it fits into 8 bits
                cpu.write_to_memory(addr, value as u8, callback);
            }
        }

        if (cpu.current_opcode & 0x03) == 0x03 {
            return;
        }

        cpu.registers.flags.set(
            (value == 0).into(),
            false.into(),
            ((value & 0x0F) == 0).into(),
            None,
        );
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
