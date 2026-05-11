//! Runnable CDP1802 assembler/emulator demo.

use sw_cdp1802_asm::assemble;
use sw_cdp1802_emulator::{CpuState, Memory, run};

const MAX_STEPS: u64 = 100;

const DEMO_SOURCE: &str = include_str!("asm/cdp1802_demo.s");

fn main() {
    println!("=== CDP1802 demo ===");
    println!("--- source ---");
    println!("{DEMO_SOURCE}");

    let asm = assemble(DEMO_SOURCE).expect("assemble demo");
    println!("--- assembled ({} bytes) ---", asm.bytes.len());
    print_hex_dump(&asm.bytes);
    println!();

    let mut mem = Memory::default();
    mem.load_bytes(0, &asm.bytes);
    let mut state = CpuState::new();
    let steps = run(&mut state, &mut mem, MAX_STEPS).expect("run demo");

    println!(
        "--- ran {steps} instructions; halted = {} ---",
        state.halted
    );
    println!("--- final registers ---");
    println!("D  = 0x{:02x}", state.d);
    println!("P  = 0x{:x}", state.p);
    println!("X  = 0x{:x}", state.x);
    println!("R1 = 0x{:04x}", state.read_reg(1));
    println!();

    println!("--- ram 0x2000..0x2002 ---");
    let data = mem.read_range(0x2000, 3);
    print_hex_dump_at(0x2000, &data);
}

fn print_hex_dump(bytes: &[u8]) {
    print_hex_dump_at(0, bytes);
}

fn print_hex_dump_at(base: u16, bytes: &[u8]) {
    for (i, chunk) in bytes.chunks(16).enumerate() {
        print!("  {:04x}: ", base as usize + i * 16);
        for b in chunk {
            print!("{b:02x} ");
        }
        for _ in chunk.len()..16 {
            print!("   ");
        }
        print!(" |");
        for b in chunk {
            let c = if b.is_ascii_graphic() || *b == b' ' {
                *b as char
            } else {
                '.'
            };
            print!("{c}");
        }
        println!("|");
    }
}
