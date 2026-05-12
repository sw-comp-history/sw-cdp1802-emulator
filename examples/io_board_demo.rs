//! Runnable CDP1802 ELF-style I/O board demo.

use sw_cdp1802_asm::{assemble, assemble_intel_hex, assemble_listing};
use sw_cdp1802_emulator::{
    CpuState, FrontPanel, Memory, VIDEO_BASE, VideoView, format_cpu_state, run_with_front_panel,
};

pub const MAX_STEPS: u64 = 100;

pub const DEMO_SOURCE: &str = include_str!("asm/io_board_demo.s");

fn main() {
    println!("=== CDP1802 ELF-style I/O board demo ===");
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

    let asm = assemble(DEMO_SOURCE).expect("assemble I/O board demo");
    println!("--- assembled ({} bytes) ---", asm.bytes.len());
    print_hex_dump(&asm.bytes);
    println!();

    let mut mem = Memory::default();
    mem.load_bytes(0, &asm.bytes);

    let mut state = CpuState::new();
    state.x = 1;

    let mut board = FrontPanel::new();
    board.input_latch = 0x3c;
    board.keypad = 0x0c;
    board.input_pressed = true;

    println!("--- initial front panel ---");
    print_front_panel(&board);
    println!();

    let steps = run_with_front_panel(&mut state, &mut mem, &mut board, MAX_STEPS)
        .expect("run I/O board demo");

    println!(
        "--- ran {steps} instructions; halted = {} ---",
        state.halted
    );
    println!("--- final CPU state ---");
    print!("{}", format_cpu_state(&state));
    println!();

    println!("--- final front panel ---");
    print_front_panel(&board);
    println!();

    println!("--- ram 0x2000..0x2002 ---");
    print_hex_dump_at(VIDEO_BASE, &mem.read_range(VIDEO_BASE, 3));
    println!();

    println!("--- video 64x32 @ 0x2000 ---");
    println!("{}", VideoView::elf_64x32().render_text(&mem));
}

fn print_front_panel(board: &FrontPanel) {
    println!("input_latch  = 0x{:02x}", board.input_latch);
    println!("keypad       = 0x{:02x}", board.keypad);
    println!("input_pressed = {}", board.input_pressed);
    println!("hex_display  = 0x{:02x}", board.hex_display);
    println!("q_led        = {}", board.q_led);
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
