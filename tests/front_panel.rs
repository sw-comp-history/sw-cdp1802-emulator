use sw_cdp1802_asm::assemble;
use sw_cdp1802_emulator::{CpuState, FrontPanel, Memory, run_with_front_panel};

const SOURCE: &str = r#"
        ORG 0x0000
        LDI 0x20
        PHI R1
        LDI 0x00
        PLO R1
        LDI 0xab
        STR R1
        OUT 1
        INP 1
        IDL
"#;

#[test]
fn assembled_front_panel_program_outputs_and_inputs_port_one() {
    let asm = assemble(SOURCE).expect("assemble front-panel demo");
    assert_eq!(
        asm.bytes,
        vec![
            0xF8, 0x20, 0xB1, 0xF8, 0x00, 0xA1, 0xF8, 0xAB, 0x51, 0x61, 0x69, 0x00
        ]
    );

    let mut mem = Memory::default();
    mem.load_bytes(0, &asm.bytes);
    let mut state = CpuState::new();
    state.x = 1;
    let mut board = FrontPanel::new();
    board.input_latch = 0x5C;
    board.input_pressed = true;

    let steps = run_with_front_panel(&mut state, &mut mem, &mut board, 100).expect("run demo");

    assert_eq!(steps, 9);
    assert!(state.halted);
    assert_eq!(board.hex_display, 0xAB);
    assert_eq!(state.d, 0x5C);
    assert_eq!(mem.read_byte(0x2000), 0xAB);
    assert_eq!(mem.read_byte(0x2001), 0x5C);
    assert_eq!(state.read_reg(1), 0x2001);
}
