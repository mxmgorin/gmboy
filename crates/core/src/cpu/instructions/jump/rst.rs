use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn execute_rst_0x00(&mut self) {
        self.execute_rst(0x00)
    }

    #[inline(always)]
    pub fn execute_rst_0x08(&mut self) {
        self.execute_rst(0x08)
    }

    #[inline(always)]
    pub fn execute_rst_0x10(&mut self) {
        self.execute_rst(0x10)
    }

    #[inline(always)]
    pub fn execute_rst_0x18(&mut self) {
        self.execute_rst(0x18)
    }

    #[inline(always)]
    pub fn execute_rst_0x20(&mut self) {
        self.execute_rst(0x20)
    }

    #[inline(always)]
    pub fn execute_rst_0x28(&mut self) {
        self.execute_rst(0x28)
    }

    #[inline(always)]
    pub fn execute_rst_0x30(&mut self) {
        self.execute_rst(0x30)
    }

    #[inline(always)]
    pub fn execute_rst_0x38(&mut self) {
        self.execute_rst(0x38)
    }

    #[inline(always)]
    pub fn execute_rst(&mut self, addr: u16) {
        self.goto_addr_push_pc(addr);
    }
}
