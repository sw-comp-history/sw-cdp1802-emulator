//! `sw-cdp1802-emulator`: RCA CDP1802 Emulator: instruction execution semantics.
//!
//! Minimal emulator core for the CDP1802 demo subset.

pub mod board;
pub mod dump;
pub mod exec;
pub mod memory;
pub mod state;
pub mod video;

pub use board::{BoardIo, FrontPanel, JoystickAxis, JoystickRcBoard};
pub use dump::{format_cpu_state, format_hex_dump};
pub use exec::{
    ExecError, run, run_with_front_panel, run_with_joystick, step, step_with_front_panel,
    step_with_joystick,
};
pub use memory::Memory;
pub use state::CpuState;
pub use video::{VIDEO_BASE, VIDEO_HEIGHT, VIDEO_SIZE_BYTES, VIDEO_WIDTH, VideoView};
