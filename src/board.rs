//! Minimal ELF-style front-panel board.

use sw_cdp1802_isa::ExternalFlag;

use crate::state::CpuState;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct FrontPanel {
    pub input_latch: u8,
    pub hex_display: u8,
    pub keypad: u8,
    pub input_pressed: bool,
    pub q_led: bool,
}

impl FrontPanel {
    pub const PORT_HEX_KEYPAD: u8 = 1;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn input_port(&self, port: u8) -> u8 {
        match port {
            Self::PORT_HEX_KEYPAD => self.input_latch,
            _ => 0,
        }
    }

    pub fn output_port(&mut self, port: u8, value: u8) {
        if port == Self::PORT_HEX_KEYPAD {
            self.hex_display = value;
        }
    }

    pub fn sync_inputs_to_cpu(&self, state: &mut CpuState) {
        state.set_external_flag(ExternalFlag::Ef4, self.input_pressed);
    }

    pub fn sync_outputs_from_cpu(&mut self, state: &CpuState) {
        self.q_led = state.q;
    }
}
