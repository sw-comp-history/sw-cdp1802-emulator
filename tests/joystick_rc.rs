use sw_cdp1802_asm::assemble;
use sw_cdp1802_emulator::{
    BoardIo, CpuState, JoystickAxis, JoystickRcBoard, Memory, step_with_joystick,
};

#[allow(dead_code)]
#[path = "../examples/joystick_rc_demo.rs"]
mod joystick_rc_demo;

#[test]
fn rc_delay_maps_axis_values_to_four_poll_buckets() {
    let board = JoystickRcBoard::new(0, 255);
    assert_eq!(board.delay_for_axis(JoystickAxis::X), 0);
    assert_eq!(board.delay_for_axis(JoystickAxis::Y), 3);

    let board = JoystickRcBoard::new(64, 128);
    assert_eq!(board.delay_for_axis(JoystickAxis::X), 1);
    assert_eq!(board.delay_for_axis(JoystickAxis::Y), 2);
}

#[test]
fn rc_pulse_becomes_ready_after_axis_delay() {
    let mut board = JoystickRcBoard::new(128, 0);
    let mut state = CpuState::new();

    board.output_port(JoystickRcBoard::PORT_X_PULSE, 0);
    board.sync_inputs_to_cpu(&mut state);
    assert!(!state.ef[3]);

    board.after_instruction();
    board.sync_inputs_to_cpu(&mut state);
    assert!(!state.ef[3]);

    board.after_instruction();
    board.sync_inputs_to_cpu(&mut state);
    assert!(!state.ef[3]);

    board.after_instruction();
    board.sync_inputs_to_cpu(&mut state);
    assert!(state.ef[3]);
}

#[test]
fn joystick_demo_frame_places_ball_from_measured_x_y() {
    let asm = assemble(joystick_rc_demo::DEMO_SOURCE).expect("assemble joystick demo");
    assert!(asm.bytes.len() < 64);

    let frame = joystick_rc_demo::run_frame(128, 64);

    assert!(frame.state.halted);
    assert_eq!(frame.board.delay_for_axis(JoystickAxis::X), 2);
    assert_eq!(frame.board.delay_for_axis(JoystickAxis::Y), 1);
    assert_eq!(frame.state.read_reg(1), 0x0044);
    assert_eq!(frame.memory.read_byte(0x0044), 0x80);

    let rendered = joystick_rc_demo::render_solid_video(&frame.memory);
    let lines: Vec<&str> = rendered.lines().collect();
    assert_eq!(lines.len(), 32);
    assert_eq!(lines[8].chars().nth(32), Some('█'));
    assert!(rendered.chars().filter(|c| *c == '█').count() > 1);
}

#[test]
fn joystick_demo_args_hide_assembler_artifacts_by_default() {
    let options = joystick_rc_demo::parse_args(&["--once".into(), "128".into(), "64".into()])
        .expect("parse --once");

    assert_eq!(options.once, Some((128, 64)));
    assert!(!options.show_source);
    assert!(!options.show_listing);
    assert!(!options.show_hex);

    let options = joystick_rc_demo::parse_args(&[
        "--source".into(),
        "--listing".into(),
        "--hex".into(),
        "--once".into(),
        "128".into(),
        "64".into(),
    ])
    .expect("parse artifact flags");

    assert!(options.show_source);
    assert!(options.show_listing);
    assert!(options.show_hex);
}

#[test]
fn joystick_step_uses_ef4_to_observe_ready_signal() {
    let source = "OUT 2\nB4 READY\nIDL\nREADY: IDL\n";
    let asm = assemble(source).expect("assemble tiny joystick poll");
    let mut memory = Memory::default();
    memory.load_bytes(0, &asm.bytes);
    let mut state = CpuState::new();
    state.x = 15;
    let mut board = JoystickRcBoard::new(0, 0);

    step_with_joystick(&mut state, &mut memory, Some(&mut board)).expect("out");
    step_with_joystick(&mut state, &mut memory, Some(&mut board)).expect("branch");

    assert_eq!(state.pc(), 4);
}
