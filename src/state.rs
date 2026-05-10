//! CDP1802 CPU state for the demo subset.

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CpuState {
    pub r: [u16; 16],
    pub d: u8,
    pub p: u8,
    pub x: u8,
    pub halted: bool,
    pub instr_count: u64,
}

impl CpuState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pc(&self) -> u16 {
        self.r[(self.p & 0x0F) as usize]
    }

    pub fn set_pc(&mut self, value: u16) {
        self.r[(self.p & 0x0F) as usize] = value;
    }

    pub fn advance_pc(&mut self, n: u16) {
        let pc = self.pc().wrapping_add(n);
        self.set_pc(pc);
    }

    pub fn read_reg(&self, index: u8) -> u16 {
        self.r[(index & 0x0F) as usize]
    }

    pub fn write_reg(&mut self, index: u8, value: u16) {
        self.r[(index & 0x0F) as usize] = value;
    }
}

impl Default for CpuState {
    fn default() -> Self {
        Self {
            r: [0; 16],
            d: 0,
            p: 0,
            x: 0,
            halted: false,
            instr_count: 0,
        }
    }
}
