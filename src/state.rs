//! CDP1802 CPU state for the demo subset.

use sw_cdp1802_isa::ExternalFlag;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CpuState {
    pub r: [u16; 16],
    pub d: u8,
    pub df: bool,
    pub p: u8,
    pub x: u8,
    pub t: u8,
    pub q: bool,
    pub ef: [bool; 4],
    pub interrupt_enabled: bool,
    pub interrupt_pending: bool,
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

    pub fn data_flag(&self) -> bool {
        self.df
    }

    pub fn set_data_flag(&mut self, value: bool) {
        self.df = value;
    }

    pub fn external_flag(&self, flag: ExternalFlag) -> bool {
        self.ef[(flag.index_u8() - 1) as usize]
    }

    pub fn set_external_flag(&mut self, flag: ExternalFlag, value: bool) {
        self.ef[(flag.index_u8() - 1) as usize] = value;
    }

    pub fn set_interrupt_enabled(&mut self, value: bool) {
        self.interrupt_enabled = value;
    }

    pub fn request_interrupt(&mut self) {
        self.interrupt_pending = true;
    }

    pub fn clear_interrupt_request(&mut self) {
        self.interrupt_pending = false;
    }
}

impl Default for CpuState {
    fn default() -> Self {
        Self {
            r: [0; 16],
            d: 0,
            df: false,
            p: 0,
            x: 0,
            t: 0,
            q: false,
            ef: [false; 4],
            interrupt_enabled: true,
            interrupt_pending: false,
            halted: false,
            instr_count: 0,
        }
    }
}
