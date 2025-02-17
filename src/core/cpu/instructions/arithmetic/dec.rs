use crate::core::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::instructions::{DataDestination, FetchedData};
use crate::cpu::{Cpu, CpuCallback};

#[derive(Debug, Clone, Copy)]
pub struct DecInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for DecInstruction {
    fn execute(
        &self,
        cpu: &mut Cpu,
        callback: &mut impl CpuCallback,
        fetched_data: FetchedData,
    ) {
        let mut value = fetched_data.value.wrapping_sub(1);

        match fetched_data.dest {
            DataDestination::Register(r) => {
                if r.is_16bit() {
                    callback.m_cycles(1, &mut cpu.bus);
                }

                cpu.registers.set_register(r, value);
                value = cpu.registers.read_register(r);
            }
            DataDestination::Memory(addr) => {
                cpu.write_to_memory(addr, value as u8, callback);
            }
        }

        if (cpu.current_opcode & 0x0B) == 0x0B {
            return;
        }

        cpu.registers.flags.set(
            (value == 0).into(),
            true.into(),
            ((value & 0x0F) == 0x0F).into(),
            None,
        );
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
