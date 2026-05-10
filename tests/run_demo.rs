//! End-to-end demo: assemble source, load bytes, run to IDL, and
//! assert final machine state.

use sw_cdp1802_asm::assemble;
use sw_cdp1802_emulator::{CpuState, Memory, run};

const MAX_STEPS: u64 = 100;

const DEMO_SOURCE: &str = r#"
        ORG 0x0000
        LDI 0x20
        PHI R1
        LDI 0x00
        PLO R1
        LDI 0x42
        STR R1
        INC R1
        LDI 0x43
        STR R1
        INC R1
        LDI 0x44
        STR R1
        BR DONE
DONE:   IDL
"#;

const DEMO_BYTES: &[u8] = &[
    0xF8, 0x20, 0xB1, 0xF8, 0x00, 0xA1, 0xF8, 0x42, 0x51, 0x11, 0xF8, 0x43, 0x51, 0x11, 0xF8, 0x44,
    0x51, 0x30, 0x13, 0x00,
];

#[test]
fn assembled_demo_runs_to_contract_state() {
    let asm = assemble(DEMO_SOURCE).expect("assemble demo");
    assert_eq!(asm.bytes, DEMO_BYTES);

    let mut mem = Memory::default();
    mem.load_bytes(0, &asm.bytes);
    let mut state = CpuState::new();
    let steps = run(&mut state, &mut mem, MAX_STEPS).expect("run demo");

    assert_eq!(steps, 14);
    assert!(
        state.halted,
        "program did not halt within {MAX_STEPS} steps"
    );
    assert_eq!(mem.read_range(0x2000, 3), vec![0x42, 0x43, 0x44]);
    assert_eq!(state.read_reg(1), 0x2002);
}
