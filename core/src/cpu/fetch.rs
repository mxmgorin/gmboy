use crate::cpu::{Cpu, RegisterType};
use serde::{Deserialize, Serialize};

impl Cpu {
    #[inline(always)]
    pub fn read_mr<const R2: u8>(&mut self) -> u8 {
        let addr = self.registers.get_register::<R2>();

        self.read_memory(addr)
    }

    #[inline(always)]
    pub fn read_addr_mr<const R1: u8>(&mut self) -> (u16, u8) {
        let addr = self.registers.get_register::<R1>();
        let value = self.read_memory(addr);

        (addr, value)
    }

    #[inline(always)]
    pub fn read_hmr<const R2: u8>(&mut self) -> u8 {
        let addr = self.registers.get_register::<R2>();
        let addr = 0xFF00 | addr;

        self.read_memory(addr)
    }

    #[inline(always)]
    pub fn read_mr_r<const R1: u8, const R2: u8>(&mut self) -> (u16, u16) {
        (
            self.registers.get_register::<R1>(),
            self.registers.get_register::<R2>(),
        )
    }

    #[inline(always)]
    pub fn read_mri<const R2: u8>(&mut self) -> u8 {
        let addr = self.registers.get_register::<R2>();
        let val = self.read_memory(addr);
        self.registers.set_hl(addr.wrapping_add(1));

        val
    }

    #[inline(always)]
    pub fn read_r_mrd<const R1: u8, const R2: u8>(&mut self) -> u8 {
        let addr = self.registers.get_register::<R2>();
        let value = self.read_memory(addr);
        self.registers.set_hl(addr.wrapping_sub(1));

        value
    }

    #[inline(always)]
    pub fn read_mri_r<const R1: u8, const R2: u8>(&mut self) -> (u16, u16) {
        let addr = self.registers.get_register::<R1>();
        let value = self.registers.get_register::<R2>();
        self.registers.set_hl(addr.wrapping_add(1));

        (addr, value)
    }

    #[inline(always)]
    pub fn read_mrd_r<const R1: u8, const R2: u8>(&mut self) -> (u16, u16) {
        let addr = self.registers.get_register::<R1>();
        let value = self.registers.get_register::<R2>();
        self.registers.set_hl(addr.wrapping_sub(1));

        (addr, value)
    }

    #[inline(always)]
    pub fn read_ha8(&mut self) -> u8 {
        let addr = self.read_pc() as u16;
        let addr = 0xFF00 | addr;

        self.read_memory(addr)
    }

    #[inline(always)]
    pub fn read_a8_r8<const R2: u8>(&mut self) -> (u8, u8) {
        (self.read_pc(), self.registers.get_register8::<R2>())
    }

    #[inline(always)]
    pub fn read_d8(&mut self) -> u8 {
        self.read_pc()
    }

    #[inline(always)]
    pub fn read_a16_r<const R2: u8>(&mut self) -> (u16, u16) {
        (self.read_pc16(), self.registers.get_register::<R2>())
    }

    #[inline(always)]
    pub fn read_mr_d8<const R1: u8>(&mut self) -> (u16, u8) {
        (self.registers.get_register::<R1>(), self.read_pc())
    }

    #[inline(always)]
    pub fn read_a16(&mut self) -> u8 {
        let addr = self.read_pc16();
        self.read_memory(addr)
    }
}

/// Represents the different address modes in the CPU's instruction set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum AddressMode {
    /// Implied: The operand is specified in the instruction itself
    ///
    /// Cycles: 0.
    IMP,
    /// Register: Fetches value of register.
    ///
    /// Cycles: 0.
    R(RegisterType),
    /// Register and 16-bit Data: Fetches the 16-bit data by PC.
    ///
    /// Cycles: 2.
    R_D16(RegisterType),
    /// Register to Register: Fetches the data from second register.
    ///
    /// Cycles: 0.
    R_R(RegisterType, RegisterType),
    /// Memory address Register and Register: Fetches the data from second register and memory address from first register.
    ///
    /// Cycles: 0.
    MR_R(RegisterType, RegisterType),
    /// Register and 8-bit data: Fetches value from PC.
    ///
    /// Cycles: 1.
    R_D8(RegisterType),
    /// Register and Memory address Register: Fetches address from second register.
    ///
    /// Cycles: 1.
    R_MR(RegisterType, RegisterType),
    /// Register and High Memory address Register: Fetches address from second register.
    ///
    /// Cycles: 1.
    R_HMR(RegisterType, RegisterType),
    /// Register and HL increment.
    ///
    /// Cycles: 1.
    R_HLI(RegisterType),
    /// Register and HL decrement.
    ///
    /// Cycles: 1.
    R_HLD(RegisterType),
    /// HL increment and Register.
    ///
    /// Cycles: 0.
    HLI_R(RegisterType),
    /// HL decrement and Register.
    ///
    /// Cycles: 0.
    HLD_R(RegisterType),
    /// Register and 8-bit address: Fetches value from 8-bit address.
    ///
    /// Cycles: 1.
    R_A8(RegisterType),
    /// Register and 8-bit high address: Fetches value from 8-bit address.
    ///
    /// Cycles: 1.
    R_HA8(RegisterType),
    /// 8-bit address and Register: Fetches value from second register.
    ///
    /// Cycles: 1.
    A8_R(RegisterType),
    /// HL and SP: HL,(SP+8e): Fetches PC value.
    ///
    /// Cycles: 1.
    LH_SPi8,
    /// 16-bit data: Fetches 16-bit value from memory by PC.
    ///
    /// Cycles: 2.
    D16,
    /// 8-bit data: Fetches 8-bit value from memory by PC.
    ///
    /// Cycles: 1.
    D8,
    /// Memory Address Register and 8-bit data: Fetches 8-bit value from memory by PC and memory address from register.
    ///
    /// Cycles: 1.
    MR_D8(RegisterType),
    /// Memory Address Register: Fetches memory address from register and data by that address.
    ///
    /// Cycles: 1.
    MR(RegisterType),
    /// 16-bit Address and Register: Fetches value from register and memory address by PC.
    ///
    /// Cycles: 2.
    A16_R(RegisterType),
    /// Register and 16-bit Address: Fetches value from register and memory address by PC.
    ///
    /// Cycles: 1.
    R_A16(RegisterType),
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FetchedData {
    pub addr: u16,
    pub value: u16,
}
