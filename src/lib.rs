//! `sw-cdp1802-emulator`: RCA CDP1802 Emulator: instruction execution semantics.
//!
//! Minimal emulator core for the CDP1802 demo subset.

pub mod exec;
pub mod memory;
pub mod state;

pub use exec::{ExecError, run, step};
pub use memory::Memory;
pub use state::CpuState;
