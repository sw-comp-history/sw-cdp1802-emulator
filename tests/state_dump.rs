use sw_cdp1802_emulator::{CpuState, format_cpu_state, format_hex_dump};

#[test]
fn cpu_state_dump_includes_current_architectural_state() {
    let mut state = CpuState::new();
    state.d = 0x42;
    state.df = true;
    state.p = 0x2;
    state.x = 0xf;
    state.t = 0x2f;
    state.q = true;
    state.ef = [true, false, true, false];
    state.interrupt_enabled = false;
    state.interrupt_pending = true;
    state.halted = true;
    state.instr_count = 7;
    for i in 0..16 {
        state.write_reg(i, 0x1000 + i as u16);
    }

    let dump = format_cpu_state(&state);

    for expected in [
        "D           = 0x42",
        "DF          = true",
        "P           = 0x2",
        "X           = 0xf",
        "T           = 0x2f",
        "Q           = true",
        "EF1         = true",
        "EF2         = false",
        "EF3         = true",
        "EF4         = false",
        "IE          = false",
        "IRQ pending = true",
        "halted      = true",
        "instr_count = 7",
        "R0         = 0x1000",
        "R1         = 0x1001",
        "R2         = 0x1002",
        "R3         = 0x1003",
        "R4         = 0x1004",
        "R5         = 0x1005",
        "R6         = 0x1006",
        "R7         = 0x1007",
        "R8         = 0x1008",
        "R9         = 0x1009",
        "RA         = 0x100a",
        "RB         = 0x100b",
        "RC         = 0x100c",
        "RD         = 0x100d",
        "RE         = 0x100e",
        "RF         = 0x100f",
    ] {
        assert!(
            dump.contains(expected),
            "missing `{expected}` from dump:\n{dump}"
        );
    }
}

#[test]
fn hex_dump_includes_address_bytes_and_ascii() {
    let dump = format_hex_dump(0x2000, &[0x41, 0x00, 0x7e]);

    assert_eq!(
        dump,
        "  2000: 41 00 7e                                         |A.~|\n"
    );
}
