use crate::core::cpu::instructions::common::{AddressMode, ExecutableInstruction};
use crate::core::cpu::Cpu;
use crate::cpu::instructions::common::FetchedData;

#[derive(Debug, Clone, Copy)]
pub struct SbcInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for SbcInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        match self.address_mode {
            AddressMode::IMP |
            AddressMode::R(_) |
            AddressMode::R_D16(_) |
            AddressMode::MR_R(_, _) |
            AddressMode::R_HLI(_) |
            AddressMode::R_HLD(_) |
            AddressMode::HLI_R(_) |
            AddressMode::HLD_R(_) |
            AddressMode::R_A8(_) |
            AddressMode::A8_R(_) |
            AddressMode::HL_SPe8 |
            AddressMode::D16 |
            AddressMode::D8 |
            AddressMode::D16_R(_) |
            AddressMode::MR_D8(_) |
            AddressMode::MR(_) |
            AddressMode::A16_R(_) |
            AddressMode::R_A16(_) => panic!("not used"),
            AddressMode::R_R(r1, _) |
            AddressMode::R_MR(r1, _) |
            AddressMode::R_D8(r1) => {
                let c_val = cpu.registers.flags.get_c();
                let val = fetched_data.value.wrapping_add(c_val as u16) as u8;
                let r_val = cpu.registers.read_register(r1);

                let r_val_i32 = r_val as i32;
                let retched_val_i32 = fetched_data.value as i32;
                let c_val_i32 = c_val as i32;

                let final_val = r_val.wrapping_sub(val as u16);
                let z = final_val == 0;
                let h = (r_val_i32 & 0xF).wrapping_sub(retched_val_i32 & 0xF).wrapping_sub(c_val_i32) < 0;
                let c = r_val_i32.wrapping_sub(retched_val_i32).wrapping_sub(c_val_i32) < 0;

                cpu.registers.set_register(r1, final_val);
                cpu.registers.flags.set(z.into(), true.into(), h.into(), c.into());
            }

        }
        
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::instructions::common::{AddressMode, FetchedData, RegisterType};
    use crate::core::cpu::instructions::arithmetic::sbc::SbcInstruction;

    #[test]
    fn test_1() {
        let _inst = SbcInstruction { address_mode: AddressMode::R_D8(RegisterType::A) };
        let _fetched_data = FetchedData { value: 0, dest_addr: None };
    }
}
