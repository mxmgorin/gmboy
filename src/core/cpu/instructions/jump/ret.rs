use crate::core::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::instructions::{ConditionType, FetchedData};
use crate::cpu::stack::Stack;
use crate::cpu::{Cpu, CpuCycleCallback};

#[derive(Debug, Clone, Copy)]
pub struct RetInstruction {
    pub condition_type: Option<ConditionType>,
}

impl ExecutableInstruction for RetInstruction {
    fn execute(
        &self,
        cpu: &mut Cpu,
        callback: &mut impl CpuCycleCallback,
        _fetched_data: FetchedData,
    ) {
        if self.condition_type.is_some() {
            callback.m_cycles(1, &mut cpu.bus);
        }

        if ConditionType::check_cond(&cpu.registers, self.condition_type) {
            let lo = Stack::pop(cpu, callback) as u16;
            let hi = Stack::pop(cpu, callback) as u16;

            let addr = (hi << 8) | lo;

            cpu.set_pc(addr, callback);
        }
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
