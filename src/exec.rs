//! Instruction execution dispatch for the demo subset.

use sw_cdp1802_isa::Instruction;
use sw_isa_core::DecodeError;

use crate::board::{BoardIo, FrontPanel, JoystickRcBoard};
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
    step_with_board::<FrontPanel>(state, mem, None)
}

pub fn step_with_front_panel(
    state: &mut CpuState,
    mem: &mut Memory,
    board: Option<&mut FrontPanel>,
) -> Result<(), ExecError> {
    step_with_board(state, mem, board)
}

pub fn step_with_joystick(
    state: &mut CpuState,
    mem: &mut Memory,
    board: Option<&mut JoystickRcBoard>,
) -> Result<(), ExecError> {
    step_with_board(state, mem, board)
}

fn step_with_board<B: BoardIo>(
    state: &mut CpuState,
    mem: &mut Memory,
    mut board: Option<&mut B>,
) -> Result<(), ExecError> {
    if state.halted {
        return Err(ExecError::Halted);
    }
    if let Some(board) = board.as_deref() {
        board.sync_inputs_to_cpu(state);
    }

    let pc = state.pc();
    let (insn, size) = mem.decode_at(pc)?;
    state.advance_pc(size);
    state.instr_count += 1;
    exec_instruction(state, mem, board.as_deref_mut(), insn);
    if let Some(board) = board.as_deref_mut() {
        board.sync_outputs_from_cpu(state);
        board.after_instruction();
    }
    Ok(())
}

pub fn run(state: &mut CpuState, mem: &mut Memory, max_steps: u64) -> Result<u64, ExecError> {
    let start = state.instr_count;
    while !state.halted && state.instr_count - start < max_steps {
        step(state, mem)?;
    }
    Ok(state.instr_count - start)
}

pub fn run_with_front_panel(
    state: &mut CpuState,
    mem: &mut Memory,
    board: &mut FrontPanel,
    max_steps: u64,
) -> Result<u64, ExecError> {
    let start = state.instr_count;
    while !state.halted && state.instr_count - start < max_steps {
        step_with_front_panel(state, mem, Some(board))?;
    }
    Ok(state.instr_count - start)
}

pub fn run_with_joystick(
    state: &mut CpuState,
    mem: &mut Memory,
    board: &mut JoystickRcBoard,
    max_steps: u64,
) -> Result<u64, ExecError> {
    let start = state.instr_count;
    while !state.halted && state.instr_count - start < max_steps {
        step_with_joystick(state, mem, Some(board))?;
    }
    Ok(state.instr_count - start)
}

fn exec_instruction<B: BoardIo>(
    state: &mut CpuState,
    mem: &mut Memory,
    mut board: Option<&mut B>,
    insn: Instruction,
) {
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
        Instruction::Output { port } => {
            let idx = state.x & 0x0F;
            let addr = state.read_reg(idx);
            let value = mem.read_byte(addr);
            if let Some(board) = board.as_deref_mut() {
                board.output_port(port, value);
            }
            state.write_reg(idx, addr.wrapping_add(1));
        }
        Instruction::Input { port } => {
            let value = board.as_deref().map_or(0, |board| board.input_port(port));
            mem.write_byte(state.read_reg(state.x), value);
            state.d = value;
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
