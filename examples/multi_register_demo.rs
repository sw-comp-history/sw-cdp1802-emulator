//! Runnable CDP1802 multi-register assembler/emulator demo.

use sw_cdp1802_asm::{assemble, assemble_intel_hex, assemble_listing};
use sw_cdp1802_emulator::{CpuState, Memory, format_cpu_state, format_hex_dump, run};

const MAX_STEPS: u64 = 100;

const DEMO_SOURCE: &str = include_str!("asm/multi_register_demo.s");

fn main() {
    println!("=== CDP1802 multi-register demo ===");
    println!("--- source ---");
    println!("{DEMO_SOURCE}");

    println!("--- listing ---");
    print!(
        "{}",
        assemble_listing(DEMO_SOURCE).expect("assemble listing")
    );
    println!("--- intel hex ---");
    print!(
        "{}",
        assemble_intel_hex(DEMO_SOURCE).expect("assemble Intel HEX")
    );
    println!();

    let asm = assemble(DEMO_SOURCE).expect("assemble multi-register demo");
    println!("--- assembled ({} bytes) ---", asm.bytes.len());
    print!("{}", format_hex_dump(0, &asm.bytes));
    println!();

    let mut mem = Memory::default();
    mem.load_bytes(0, &asm.bytes);
    let mut state = CpuState::new();
    let steps = run(&mut state, &mut mem, MAX_STEPS).expect("run multi-register demo");

    println!(
        "--- ran {steps} instructions; halted = {} ---",
        state.halted
    );
    println!("--- final CPU state ---");
    print!("{}", format_cpu_state(&state));
    println!();

    println!("--- selected RAM writes ---");
    for addr in [0x2010, 0x2020, 0x2030, 0x2040] {
        println!("0x{addr:04x} = 0x{:02x}", mem.read_byte(addr));
    }
    println!();

    println!("--- ram 0x2010..0x2040 touched bytes ---");
    print!("{}", format_hex_dump(0x2010, &mem.read_range(0x2010, 0x31)));
}
