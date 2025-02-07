use crate::core::cpu::instructions::{
    AddressMode, ConditionType, ExecutableInstruction, Instruction,
};
use crate::core::cpu::Cpu;
use crate::cpu::instructions::FetchedData;

#[derive(Debug, Clone, Copy)]
pub struct JrInstruction {
    pub condition_type: Option<ConditionType>,
}

impl ExecutableInstruction for JrInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        let rel = fetched_data.value as i8;
        let addr = (cpu.registers.pc as i32).wrapping_add(rel as i32);
        Instruction::goto_addr(cpu, self.condition_type, addr as u16, false);
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::D8
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_jr_instruction() {
        let s: u8 = 0xFE;
        println!("{}", s);
        let rel = s as i8;
        println!("{}", rel);
        println!("{}", rel as i32);

        let val: u16 = 0xFFFF;
        println!("{}", val);
        println!("{}", val as i32);
        println!("{}", val as u32 as i32);
    }
}
