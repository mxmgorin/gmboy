const INTERRUPTS_BY_ADDRESSES: [(u16, InterruptType); 5] = [
    (0x40, InterruptType::VBlank),
    (0x48, InterruptType::LCDStat),
    (0x50, InterruptType::Timer),
    (0x58, InterruptType::Serial),
    (0x60, InterruptType::Joypad),
];

#[derive(Debug, Clone)]
pub struct Interrupts {
    /// Interrupt flags
    pub int_flags: u8,
    pub cpu_halted: bool,
    pub int_master_enabled: bool,
    /// Interrupt enable register
    pub ie_register: u8,
}

impl Interrupts {
    pub fn new() -> Self {
        Self {
            int_flags: 0,
            cpu_halted: false,
            int_master_enabled: false,
            ie_register: 0,
        }
    }

    pub fn get_interrupt(&mut self) -> Option<InterruptType> {
        for (_address, interrupt_type) in INTERRUPTS_BY_ADDRESSES {
            if self.need_interrupt(interrupt_type) {
                return Some(interrupt_type);
            }
        }

        None
    }

    pub fn request_interrupt(&mut self, it: InterruptType) {
        self.int_flags |= it as u8;
    }

    pub fn handle_interrupt(
        &mut self,
        it: InterruptType,
    ) {
        let it = it as u8;
        self.int_flags &= !it;
        self.cpu_halted = false;
        self.int_master_enabled = false;
    }

    fn need_interrupt(&self, it: InterruptType) -> bool {
        let it = it as u8;

        if (self.int_flags & it != 0) && (self.ie_register & it != 0) {
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

impl InterruptType {
    pub const fn get_address(self) -> u16 {
        INTERRUPTS_BY_ADDRESSES[self as usize - 1].0
    }
}
