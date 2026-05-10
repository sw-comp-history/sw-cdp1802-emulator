//! Instruction execution dispatch for the demo subset.

use sw_cdp1802_isa::Instruction;
use sw_isa_core::DecodeError;

use crate::memory::Memory;
use crate::state::CpuState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecError {
    Halted,
    Decode(DecodeError),
}

impl From<DecodeError> for ExecError {
    fn from(value: DecodeError) -> Self {
        ExecError::Decode(value)
    }
}

pub fn step(state: &mut CpuState, mem: &mut Memory) -> Result<(), ExecError> {
    if state.halted {
        return Err(ExecError::Halted);
    }

    let pc = state.pc();
    let (insn, size) = mem.decode_at(pc)?;
    state.advance_pc(size);
    state.instr_count += 1;
    exec_instruction(state, mem, insn);
    Ok(())
}

pub fn run(state: &mut CpuState, mem: &mut Memory, max_steps: u64) -> Result<u64, ExecError> {
    let start = state.instr_count;
    while !state.halted && state.instr_count - start < max_steps {
        step(state, mem)?;
    }
    Ok(state.instr_count - start)
}

fn exec_instruction(state: &mut CpuState, mem: &mut Memory, insn: Instruction) {
    match insn {
        Instruction::Idle => {
            state.halted = true;
        }
        Instruction::Increment { reg } => {
            let idx = reg.index_u8();
            let value = state.read_reg(idx).wrapping_add(1);
            state.write_reg(idx, value);
        }
        Instruction::Branch { target } => {
            let high = state.pc() & 0xFF00;
            state.set_pc(high | target as u16);
        }
        Instruction::BranchExternalFlag {
            flag,
            expected,
            target,
        } => {
            if state.external_flag(flag) == expected {
                let high = state.pc() & 0xFF00;
                state.set_pc(high | target as u16);
            }
        }
        Instruction::Store { reg } => {
            mem.write_byte(state.read_reg(reg.index_u8()), state.d);
        }
        Instruction::ResetQ => {
            state.q = false;
        }
        Instruction::SetQ => {
            state.q = true;
        }
        Instruction::PutLow { reg } => {
            let idx = reg.index_u8();
            let value = (state.read_reg(idx) & 0xFF00) | state.d as u16;
            state.write_reg(idx, value);
        }
        Instruction::PutHigh { reg } => {
            let idx = reg.index_u8();
            let value = ((state.d as u16) << 8) | (state.read_reg(idx) & 0x00FF);
            state.write_reg(idx, value);
        }
        Instruction::LoadImmediate { value } => {
            state.d = value;
        }
    }
}
