use sw_cdp1802_emulator::{
    CpuState, ExecError, FrontPanel, Memory, run, step, step_with_front_panel,
};
use sw_cdp1802_isa::{Cdp1802, ExternalFlag, Instruction, Reg};
use sw_isa_core::Architecture;

fn write_insn(mem: &mut Memory, addr: u16, insn: Instruction) -> u16 {
    let mut buf = [0u8; 2];
    let n = Cdp1802::encode(&insn, &mut buf).unwrap();
    mem.load_bytes(addr, &buf[..n]);
    addr.wrapping_add(n as u16)
}

fn state_with_pc_away_from(reg: u8) -> CpuState {
    let mut state = CpuState::new();
    state.p = (reg.wrapping_add(1)) & 0x0F;
    state.write_reg(state.p, 0);
    state
}

#[test]
fn initial_state_matches_demo_entry_contract() {
    let state = CpuState::new();
    assert_eq!(state.p, 0);
    assert_eq!(state.x, 0);
    assert_eq!(state.pc(), 0);
    assert_eq!(state.r, [0; 16]);
    assert!(!state.q);
    assert_eq!(state.ef, [false; 4]);
    assert!(!state.halted);
}

#[test]
fn ldi_loads_d_and_advances_pc() {
    let mut mem = Memory::default();
    write_insn(&mut mem, 0, Instruction::LoadImmediate { value: 0x42 });
    let mut state = CpuState::new();
    step(&mut state, &mut mem).unwrap();
    assert_eq!(state.d, 0x42);
    assert_eq!(state.pc(), 2);
}

#[test]
fn phi_and_plo_write_register_halves_from_d() {
    let mut mem = Memory::default();
    let mut addr = 0;
    addr = write_insn(&mut mem, addr, Instruction::LoadImmediate { value: 0x20 });
    addr = write_insn(
        &mut mem,
        addr,
        Instruction::PutHigh {
            reg: Reg::new_masked(1),
        },
    );
    addr = write_insn(&mut mem, addr, Instruction::LoadImmediate { value: 0x34 });
    write_insn(
        &mut mem,
        addr,
        Instruction::PutLow {
            reg: Reg::new_masked(1),
        },
    );
    let mut state = CpuState::new();
    run(&mut state, &mut mem, 4).unwrap();
    assert_eq!(state.read_reg(1), 0x2034);
}

#[test]
fn phi_and_plo_cover_all_scratchpad_registers() {
    for reg in 0..16 {
        let mut mem = Memory::default();
        let mut addr = 0;
        addr = write_insn(&mut mem, addr, Instruction::LoadImmediate { value: 0x12 });
        addr = write_insn(
            &mut mem,
            addr,
            Instruction::PutHigh {
                reg: Reg::new_masked(reg),
            },
        );
        addr = write_insn(&mut mem, addr, Instruction::LoadImmediate { value: 0x34 });
        write_insn(
            &mut mem,
            addr,
            Instruction::PutLow {
                reg: Reg::new_masked(reg),
            },
        );
        let mut state = state_with_pc_away_from(reg);

        run(&mut state, &mut mem, 4).unwrap();

        assert_eq!(state.read_reg(reg), 0x1234, "R{reg:x}");
    }
}

#[test]
fn glo_loads_register_low_byte_into_d() {
    let mut mem = Memory::default();
    write_insn(
        &mut mem,
        0,
        Instruction::GetLow {
            reg: Reg::new_masked(1),
        },
    );
    let mut state = CpuState::new();
    state.write_reg(1, 0x12ab);

    step(&mut state, &mut mem).unwrap();

    assert_eq!(state.d, 0xab);
}

#[test]
fn str_stores_d_at_register_address() {
    let mut mem = Memory::default();
    write_insn(
        &mut mem,
        0,
        Instruction::Store {
            reg: Reg::new_masked(1),
        },
    );
    let mut state = CpuState::new();
    state.d = 0x7A;
    state.write_reg(1, 0x2000);
    step(&mut state, &mut mem).unwrap();
    assert_eq!(mem.read_byte(0x2000), 0x7A);
}

#[test]
fn str_covers_all_scratchpad_registers() {
    for reg in 0..16 {
        let mut mem = Memory::default();
        write_insn(
            &mut mem,
            0,
            Instruction::Store {
                reg: Reg::new_masked(reg),
            },
        );
        let mut state = state_with_pc_away_from(reg);
        let target = 0x2000 + reg as u16;
        state.d = 0x80 | reg;
        state.write_reg(reg, target);

        step(&mut state, &mut mem).unwrap();

        assert_eq!(mem.read_byte(target), 0x80 | reg, "R{reg:x}");
    }
}

#[test]
fn inc_increments_selected_register() {
    let mut mem = Memory::default();
    write_insn(
        &mut mem,
        0,
        Instruction::Increment {
            reg: Reg::new_masked(1),
        },
    );
    let mut state = CpuState::new();
    state.write_reg(1, 0xFFFF);
    step(&mut state, &mut mem).unwrap();
    assert_eq!(state.read_reg(1), 0);
}

#[test]
fn inc_covers_all_scratchpad_registers() {
    for reg in 0..16 {
        let mut mem = Memory::default();
        write_insn(
            &mut mem,
            0,
            Instruction::Increment {
                reg: Reg::new_masked(reg),
            },
        );
        let mut state = state_with_pc_away_from(reg);
        state.write_reg(reg, 0x12FF);

        step(&mut state, &mut mem).unwrap();

        assert_eq!(state.read_reg(reg), 0x1300, "R{reg:x}");
    }
}

#[test]
fn sex_selects_x_register() {
    let mut mem = Memory::default();
    write_insn(
        &mut mem,
        0,
        Instruction::SetX {
            reg: Reg::new_masked(2),
        },
    );
    let mut state = CpuState::new();

    step(&mut state, &mut mem).unwrap();

    assert_eq!(state.x, 2);
}

#[test]
fn add_adds_memory_at_rx_and_sets_df_on_carry() {
    let mut mem = Memory::default();
    write_insn(&mut mem, 0, Instruction::Add);
    mem.write_byte(0x1234, 0x22);
    let mut state = CpuState::new();
    state.d = 0xf0;
    state.x = 1;
    state.write_reg(1, 0x1234);

    step(&mut state, &mut mem).unwrap();

    assert_eq!(state.d, 0x12);
    assert!(state.df);
}

#[test]
fn adi_adds_immediate_and_shl_shifts_d_through_df() {
    let mut mem = Memory::default();
    let mut addr = 0;
    addr = write_insn(&mut mem, addr, Instruction::AddImmediate { value: 0x03 });
    write_insn(&mut mem, addr, Instruction::ShiftLeft);
    let mut state = CpuState::new();
    state.d = 0x7f;

    step(&mut state, &mut mem).unwrap();
    assert_eq!(state.d, 0x82);
    assert!(!state.df);

    step(&mut state, &mut mem).unwrap();
    assert_eq!(state.d, 0x04);
    assert!(state.df);
}

#[test]
fn br_replaces_low_byte_in_current_pc_page() {
    let mut mem = Memory::default();
    write_insn(&mut mem, 0x1200, Instruction::Branch { target: 0x34 });
    let mut state = CpuState::new();
    state.set_pc(0x1200);
    step(&mut state, &mut mem).unwrap();
    assert_eq!(state.pc(), 0x1234);
}

#[test]
fn seq_and_req_set_and_reset_q() {
    let mut mem = Memory::default();
    let mut addr = 0;
    addr = write_insn(&mut mem, addr, Instruction::SetQ);
    write_insn(&mut mem, addr, Instruction::ResetQ);
    let mut state = CpuState::new();

    step(&mut state, &mut mem).unwrap();
    assert!(state.q);
    assert_eq!(state.pc(), 1);

    step(&mut state, &mut mem).unwrap();
    assert!(!state.q);
    assert_eq!(state.pc(), 2);
}

#[test]
fn ef_branch_takes_when_expected_flag_matches() {
    let mut mem = Memory::default();
    write_insn(
        &mut mem,
        0x1200,
        Instruction::BranchExternalFlag {
            flag: ExternalFlag::Ef4,
            expected: true,
            target: 0x34,
        },
    );
    let mut state = CpuState::new();
    state.set_pc(0x1200);
    state.set_external_flag(ExternalFlag::Ef4, true);

    step(&mut state, &mut mem).unwrap();

    assert_eq!(state.pc(), 0x1234);
}

#[test]
fn ef_branch_falls_through_when_expected_flag_does_not_match() {
    let mut mem = Memory::default();
    write_insn(
        &mut mem,
        0x1200,
        Instruction::BranchExternalFlag {
            flag: ExternalFlag::Ef2,
            expected: true,
            target: 0x34,
        },
    );
    let mut state = CpuState::new();
    state.set_pc(0x1200);

    step(&mut state, &mut mem).unwrap();

    assert_eq!(state.pc(), 0x1202);
}

#[test]
fn negated_ef_branch_takes_when_flag_is_clear() {
    let mut mem = Memory::default();
    write_insn(
        &mut mem,
        0x1200,
        Instruction::BranchExternalFlag {
            flag: ExternalFlag::Ef1,
            expected: false,
            target: 0x40,
        },
    );
    let mut state = CpuState::new();
    state.set_pc(0x1200);

    step(&mut state, &mut mem).unwrap();

    assert_eq!(state.pc(), 0x1240);
}

#[test]
fn out_writes_memory_at_rx_to_hex_display_and_increments_rx() {
    let mut mem = Memory::default();
    write_insn(&mut mem, 0, Instruction::Output { port: 1 });
    mem.write_byte(0x2000, 0xAB);
    let mut state = CpuState::new();
    state.x = 1;
    state.write_reg(1, 0x2000);
    let mut board = FrontPanel::new();

    step_with_front_panel(&mut state, &mut mem, Some(&mut board)).unwrap();

    assert_eq!(board.hex_display, 0xAB);
    assert_eq!(state.read_reg(1), 0x2001);
    assert_eq!(state.pc(), 1);
}

#[test]
fn inp_reads_latch_to_d_and_memory_at_rx() {
    let mut mem = Memory::default();
    write_insn(&mut mem, 0, Instruction::Input { port: 1 });
    let mut state = CpuState::new();
    state.x = 1;
    state.write_reg(1, 0x2000);
    let mut board = FrontPanel::new();
    board.input_latch = 0x5C;
    board.keypad = 0x0C;

    step_with_front_panel(&mut state, &mut mem, Some(&mut board)).unwrap();

    assert_eq!(state.d, 0x5C);
    assert_eq!(mem.read_byte(0x2000), 0x5C);
    assert_eq!(state.read_reg(1), 0x2000);
}

#[test]
fn front_panel_maps_input_pressed_to_ef4_and_q_to_led() {
    let mut mem = Memory::default();
    let mut addr = 0;
    addr = write_insn(&mut mem, addr, Instruction::SetQ);
    write_insn(
        &mut mem,
        addr,
        Instruction::BranchExternalFlag {
            flag: ExternalFlag::Ef4,
            expected: true,
            target: 0x20,
        },
    );
    let mut state = CpuState::new();
    let mut board = FrontPanel::new();
    board.input_pressed = true;

    step_with_front_panel(&mut state, &mut mem, Some(&mut board)).unwrap();
    assert!(board.q_led);

    step_with_front_panel(&mut state, &mut mem, Some(&mut board)).unwrap();
    assert_eq!(state.pc(), 0x20);
    assert!(state.external_flag(ExternalFlag::Ef4));
}

#[test]
fn idl_halts_and_second_step_errors() {
    let mut mem = Memory::default();
    write_insn(&mut mem, 0, Instruction::Idle);
    let mut state = CpuState::new();
    step(&mut state, &mut mem).unwrap();
    assert!(state.halted);
    assert_eq!(state.pc(), 1);
    assert_eq!(step(&mut state, &mut mem), Err(ExecError::Halted));
}

#[test]
fn contract_demo_bytes_run_to_expected_memory_state() {
    let bytes = [
        0xF8, 0x20, 0xB1, 0xF8, 0x00, 0xA1, 0xF8, 0x42, 0x51, 0x11, 0xF8, 0x43, 0x51, 0x11, 0xF8,
        0x44, 0x51, 0x30, 0x13, 0x00,
    ];
    let mut mem = Memory::default();
    mem.load_bytes(0, &bytes);
    let mut state = CpuState::new();
    let steps = run(&mut state, &mut mem, 100).unwrap();
    assert_eq!(steps, 14);
    assert!(state.halted);
    assert_eq!(mem.read_range(0x2000, 3), vec![0x42, 0x43, 0x44]);
    assert_eq!(state.read_reg(1), 0x2002);
}
