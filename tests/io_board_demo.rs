use sw_cdp1802_asm::assemble;
use sw_cdp1802_emulator::{
    CpuState, FrontPanel, Memory, VIDEO_BASE, VideoView, run_with_front_panel,
};

#[allow(dead_code)]
#[path = "../examples/io_board_demo.rs"]
mod io_board_demo;

const EXPECTED_BYTES: &[u8] = &[
    0x7B, 0xF8, 0x20, 0xB1, 0xF8, 0x00, 0xA1, 0xF8, 0xAA, 0x51, 0x61, 0xF8, 0x55, 0x51, 0x61, 0x37,
    0x12, 0x7A, 0x69, 0x00,
];

#[test]
fn io_board_demo_runs_to_contract_state() {
    let asm = assemble(io_board_demo::DEMO_SOURCE).expect("assemble I/O board demo");
    assert_eq!(asm.bytes, EXPECTED_BYTES);

    let mut mem = Memory::default();
    mem.load_bytes(0, &asm.bytes);
    let mut state = CpuState::new();
    state.x = 1;
    let mut board = FrontPanel::new();
    board.input_latch = 0x3C;
    board.keypad = 0x0C;
    board.input_pressed = true;

    let steps = run_with_front_panel(&mut state, &mut mem, &mut board, io_board_demo::MAX_STEPS)
        .expect("run I/O board demo");

    assert_eq!(steps, 14);
    assert!(state.halted);
    assert_eq!(state.d, 0x3C);
    assert_eq!(state.p, 0);
    assert_eq!(state.x, 1);
    assert_eq!(state.read_reg(1), 0x2002);
    assert!(state.q);
    assert!(state.ef[3]);

    assert_eq!(board.hex_display, 0x55);
    assert_eq!(board.input_latch, 0x3C);
    assert_eq!(board.keypad, 0x0C);
    assert!(board.input_pressed);
    assert!(board.q_led);

    assert_eq!(mem.read_range(VIDEO_BASE, 3), vec![0xAA, 0x55, 0x3C]);

    let rendered = VideoView::elf_64x32().render_text(&mem);
    let lines: Vec<&str> = rendered.lines().collect();
    assert_eq!(&lines[0][..24], "#.#.#.#..#.#.#.#..####..");
    assert!(lines[1..].iter().all(|line| *line == ".".repeat(64)));
}
