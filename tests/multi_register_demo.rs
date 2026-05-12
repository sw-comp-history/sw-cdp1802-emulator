//! End-to-end multi-register demo coverage.

use sw_cdp1802_asm::{assemble, assemble_intel_hex, assemble_listing};
use sw_cdp1802_emulator::{CpuState, Memory, run};

const MAX_STEPS: u64 = 100;
const DEMO_SOURCE: &str = include_str!("../examples/asm/multi_register_demo.s");

#[test]
fn multi_register_demo_runs_to_expected_state() {
    let asm = assemble(DEMO_SOURCE).expect("assemble multi-register demo");
    let listing = assemble_listing(DEMO_SOURCE).expect("assemble listing");
    let hex = assemble_intel_hex(DEMO_SOURCE).expect("assemble Intel HEX");

    assert!(listing.contains("PHI RA"));
    assert!(listing.contains("PLO RF"));
    assert!(listing.contains("Symbols"));
    assert!(hex.ends_with(":00000001FF\n"));

    let mut mem = Memory::default();
    mem.load_bytes(0, &asm.bytes);
    let mut state = CpuState::new();
    let steps = run(&mut state, &mut mem, MAX_STEPS).expect("run multi-register demo");

    assert_eq!(steps, 29);
    assert!(state.halted);
    assert_eq!(mem.read_byte(0x2010), 0x52);
    assert_eq!(mem.read_byte(0x2020), 0x53);
    assert_eq!(mem.read_byte(0x2030), 0x5a);
    assert_eq!(mem.read_byte(0x2040), 0x5f);
    assert_eq!(state.read_reg(2), 0x2011);
    assert_eq!(state.read_reg(3), 0x2021);
    assert_eq!(state.read_reg(10), 0x2031);
    assert_eq!(state.read_reg(15), 0x2041);
}
