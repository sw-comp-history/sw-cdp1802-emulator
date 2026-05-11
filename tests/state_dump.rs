use sw_cdp1802_emulator::{CpuState, format_cpu_state};

#[test]
fn cpu_state_dump_includes_current_architectural_state() {
    let mut state = CpuState::new();
    state.d = 0x42;
    state.p = 0x2;
    state.x = 0xf;
    state.q = true;
    state.ef = [true, false, true, false];
    state.halted = true;
    state.instr_count = 7;
    for i in 0..16 {
        state.write_reg(i, 0x1000 + i as u16);
    }

    let dump = format_cpu_state(&state);

    for expected in [
        "D           = 0x42",
        "DF          = <not modeled>",
        "P           = 0x2",
        "X           = 0xf",
        "Q           = true",
        "EF1         = true",
        "EF2         = false",
        "EF3         = true",
        "EF4         = false",
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
