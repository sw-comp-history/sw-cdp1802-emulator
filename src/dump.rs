//! Human-readable CPU state dumps for runnable examples.

use std::fmt::Write;

use crate::state::CpuState;

const REG_NAMES: [&str; 16] = [
    "R0", "R1", "R2", "R3", "R4", "R5", "R6", "R7", "R8", "R9", "RA", "RB", "RC", "RD", "RE", "RF",
];

pub fn format_cpu_state(state: &CpuState) -> String {
    let mut out = String::new();

    writeln!(&mut out, "D           = 0x{:02x}", state.d).expect("write to String");
    writeln!(&mut out, "DF          = {}", state.df).expect("write to String");
    writeln!(&mut out, "P           = 0x{:x}", state.p).expect("write to String");
    writeln!(&mut out, "X           = 0x{:x}", state.x).expect("write to String");
    writeln!(&mut out, "T           = 0x{:02x}", state.t).expect("write to String");
    writeln!(&mut out, "Q           = {}", state.q).expect("write to String");
    for (i, value) in state.ef.iter().enumerate() {
        writeln!(&mut out, "EF{}         = {}", i + 1, value).expect("write to String");
    }
    writeln!(&mut out, "IE          = {}", state.interrupt_enabled).expect("write to String");
    writeln!(&mut out, "IRQ pending = {}", state.interrupt_pending).expect("write to String");
    writeln!(&mut out, "halted      = {}", state.halted).expect("write to String");
    writeln!(&mut out, "instr_count = {}", state.instr_count).expect("write to String");
    for (i, name) in REG_NAMES.iter().enumerate() {
        writeln!(&mut out, "{name:<11}= 0x{:04x}", state.r[i]).expect("write to String");
    }

    out
}

pub fn format_hex_dump(base: u16, bytes: &[u8]) -> String {
    let mut out = String::new();

    for (i, chunk) in bytes.chunks(16).enumerate() {
        write!(&mut out, "  {:04x}: ", base as usize + i * 16).expect("write to String");
        for b in chunk {
            write!(&mut out, "{b:02x} ").expect("write to String");
        }
        for _ in chunk.len()..16 {
            out.push_str("   ");
        }
        out.push_str(" |");
        for b in chunk {
            let c = if b.is_ascii_graphic() || *b == b' ' {
                *b as char
            } else {
                '.'
            };
            out.push(c);
        }
        out.push_str("|\n");
    }

    out
}
