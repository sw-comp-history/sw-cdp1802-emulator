use sw_cdp1802_emulator::CpuState;

#[test]
fn initial_state_includes_full_control_flags() {
    let state = CpuState::new();

    assert_eq!(state.r, [0; 16]);
    assert_eq!(state.d, 0);
    assert!(!state.df);
    assert_eq!(state.p, 0);
    assert_eq!(state.x, 0);
    assert_eq!(state.t, 0);
    assert!(!state.q);
    assert_eq!(state.ef, [false; 4]);
    assert!(state.interrupt_enabled);
    assert!(!state.interrupt_pending);
    assert!(!state.halted);
    assert_eq!(state.instr_count, 0);
}

#[test]
fn state_helpers_mutate_flags_and_interrupt_latch() {
    let mut state = CpuState::new();

    state.set_data_flag(true);
    assert!(state.data_flag());

    state.set_interrupt_enabled(false);
    assert!(!state.interrupt_enabled);

    state.request_interrupt();
    assert!(state.interrupt_pending);

    state.clear_interrupt_request();
    assert!(!state.interrupt_pending);
}
