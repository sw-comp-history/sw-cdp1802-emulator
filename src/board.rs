//! Minimal ELF-style front-panel board.

use sw_cdp1802_isa::ExternalFlag;

use crate::state::CpuState;

pub trait BoardIo {
    fn sync_inputs_to_cpu(&self, state: &mut CpuState);

    fn input_port(&self, port: u8) -> u8;

    fn output_port(&mut self, port: u8, value: u8);

    fn sync_outputs_from_cpu(&mut self, state: &CpuState);

    fn after_instruction(&mut self) {}
}

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
}

impl BoardIo for FrontPanel {
    fn sync_inputs_to_cpu(&self, state: &mut CpuState) {
        state.set_external_flag(ExternalFlag::Ef4, self.input_pressed);
    }

    fn input_port(&self, port: u8) -> u8 {
        match port {
            Self::PORT_HEX_KEYPAD => self.input_latch,
            _ => 0,
        }
    }

    fn output_port(&mut self, port: u8, value: u8) {
        if port == Self::PORT_HEX_KEYPAD {
            self.hex_display = value;
        }
    }

    fn sync_outputs_from_cpu(&mut self, state: &CpuState) {
        self.q_led = state.q;
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum JoystickAxis {
    X,
    Y,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct JoystickRcBoard {
    pub x: u8,
    pub y: u8,
    pub q_led: bool,
    active_axis: Option<JoystickAxis>,
    remaining_ticks: u8,
}

impl JoystickRcBoard {
    pub const PORT_X_PULSE: u8 = 2;
    pub const PORT_Y_PULSE: u8 = 3;

    pub fn new(x: u8, y: u8) -> Self {
        Self {
            x,
            y,
            q_led: false,
            active_axis: None,
            remaining_ticks: 0,
        }
    }

    pub fn set_position(&mut self, x: u8, y: u8) {
        self.x = x;
        self.y = y;
    }

    pub fn delay_for_axis(&self, axis: JoystickAxis) -> u8 {
        let value = match axis {
            JoystickAxis::X => self.x,
            JoystickAxis::Y => self.y,
        };
        ((value as u16 * 4) / 256) as u8
    }

    pub fn ready(&self) -> bool {
        self.active_axis.is_some() && self.remaining_ticks == 0
    }

    fn pulse(&mut self, axis: JoystickAxis) {
        self.active_axis = Some(axis);
        self.remaining_ticks = self.delay_for_axis(axis).saturating_add(1);
    }
}

impl BoardIo for JoystickRcBoard {
    fn sync_inputs_to_cpu(&self, state: &mut CpuState) {
        state.set_external_flag(ExternalFlag::Ef4, self.ready());
    }

    fn input_port(&self, _port: u8) -> u8 {
        0
    }

    fn output_port(&mut self, port: u8, _value: u8) {
        match port {
            Self::PORT_X_PULSE => self.pulse(JoystickAxis::X),
            Self::PORT_Y_PULSE => self.pulse(JoystickAxis::Y),
            _ => {}
        }
    }

    fn sync_outputs_from_cpu(&mut self, state: &CpuState) {
        self.q_led = state.q;
    }

    fn after_instruction(&mut self) {
        self.remaining_ticks = self.remaining_ticks.saturating_sub(1);
    }
}
