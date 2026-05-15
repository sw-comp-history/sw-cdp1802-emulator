//! Instruction execution dispatch.

use sw_cdp1802_isa::{Instruction, LongBranchCondition, LongSkipCondition};
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

pub fn step_with_io<B: BoardIo>(
    state: &mut CpuState,
    mem: &mut Memory,
    board: Option<&mut B>,
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
    exec_instruction(state, mem, board.as_deref_mut(), insn)?;
    if let Some(board) = board {
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
) -> Result<(), ExecError> {
    match insn {
        Instruction::Idle => {
            state.halted = true;
        }
        Instruction::Increment { reg } => {
            let idx = reg.index_u8();
            let value = state.read_reg(idx).wrapping_add(1);
            state.write_reg(idx, value);
        }
        Instruction::LoadVia { reg } => {
            state.d = mem.read_byte(state.read_reg(reg.index_u8()));
        }
        Instruction::Decrement { reg } => {
            let idx = reg.index_u8();
            let value = state.read_reg(idx).wrapping_sub(1);
            state.write_reg(idx, value);
        }
        Instruction::Branch { target } => {
            let high = state.pc() & 0xFF00;
            state.set_pc(high | target as u16);
        }
        Instruction::BranchQ { expected, target } => {
            if state.q == expected {
                let high = state.pc() & 0xFF00;
                state.set_pc(high | target as u16);
            }
        }
        Instruction::BranchZero { expected, target } => {
            if (state.d == 0) == expected {
                let high = state.pc() & 0xFF00;
                state.set_pc(high | target as u16);
            }
        }
        Instruction::BranchDataFlag { expected, target } => {
            if state.df == expected {
                let high = state.pc() & 0xFF00;
                state.set_pc(high | target as u16);
            }
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
        Instruction::ShortSkip { .. } => {}
        Instruction::LoadAdvance { reg } => {
            let idx = reg.index_u8();
            let addr = state.read_reg(idx);
            state.d = mem.read_byte(addr);
            state.write_reg(idx, addr.wrapping_add(1));
        }
        Instruction::Store { reg } => {
            mem.write_byte(state.read_reg(reg.index_u8()), state.d);
        }
        Instruction::Irx => {
            let idx = state.x & 0x0F;
            let value = state.read_reg(idx).wrapping_add(1);
            state.write_reg(idx, value);
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
        Instruction::Reserved68 => {}
        Instruction::Return => {
            restore_xp_from_stack(state, mem);
            state.interrupt_enabled = true;
        }
        Instruction::DisableInterrupt => {
            restore_xp_from_stack(state, mem);
            state.interrupt_enabled = false;
        }
        Instruction::LoadViaXAdvance => {
            let idx = state.x & 0x0F;
            let addr = state.read_reg(idx);
            state.d = mem.read_byte(addr);
            state.write_reg(idx, addr.wrapping_add(1));
        }
        Instruction::StoreViaXDecrement => {
            let idx = state.x & 0x0F;
            let addr = state.read_reg(idx);
            mem.write_byte(addr, state.d);
            state.write_reg(idx, addr.wrapping_sub(1));
        }
        Instruction::AddWithCarry => {
            let value = mem.read_byte(state.read_reg(state.x));
            add_into_d(state, value, state.df);
        }
        Instruction::SubtractDWithBorrow => {
            let value = mem.read_byte(state.read_reg(state.x));
            subtract_into_d(state, value, state.d, !state.df);
        }
        Instruction::ShiftRightWithCarry => {
            let carry_in = state.df;
            state.df = state.d & 0x01 != 0;
            state.d = (state.d >> 1) | if carry_in { 0x80 } else { 0 };
        }
        Instruction::SubtractMemoryWithBorrow => {
            let value = mem.read_byte(state.read_reg(state.x));
            subtract_into_d(state, state.d, value, !state.df);
        }
        Instruction::Save => {
            mem.write_byte(state.read_reg(state.x), state.t);
        }
        Instruction::Mark => {
            mem.write_byte(state.read_reg(2), state.t);
            state.x = state.p & 0x0F;
            state.write_reg(2, state.read_reg(2).wrapping_sub(1));
        }
        Instruction::ResetQ => {
            state.q = false;
        }
        Instruction::SetQ => {
            state.q = true;
        }
        Instruction::GetLow { reg } => {
            state.d = (state.read_reg(reg.index_u8()) & 0x00FF) as u8;
        }
        Instruction::GetHigh { reg } => {
            state.d = (state.read_reg(reg.index_u8()) >> 8) as u8;
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
        Instruction::AddWithCarryImmediate { value } => {
            add_into_d(state, value, state.df);
        }
        Instruction::SubtractDWithBorrowImmediate { value } => {
            subtract_into_d(state, value, state.d, !state.df);
        }
        Instruction::ShiftLeftWithCarry => {
            let carry_in = state.df;
            state.df = state.d & 0x80 != 0;
            state.d = state.d.wrapping_shl(1) | u8::from(carry_in);
        }
        Instruction::SubtractMemoryWithBorrowImmediate { value } => {
            subtract_into_d(state, state.d, value, !state.df);
        }
        Instruction::LongBranch { condition, target } => {
            if long_branch_condition_matches(state, condition) {
                state.set_pc(target);
            }
        }
        Instruction::NoOperation => {}
        Instruction::LongSkip { condition } => {
            if long_skip_condition_matches(state, condition) {
                state.advance_pc(2);
            }
        }
        Instruction::SetP { reg } => {
            state.p = reg.index_u8();
        }
        Instruction::SetX { reg } => {
            state.x = reg.index_u8();
        }
        Instruction::LoadViaX => {
            state.d = mem.read_byte(state.read_reg(state.x));
        }
        Instruction::Or => {
            state.d |= mem.read_byte(state.read_reg(state.x));
        }
        Instruction::And => {
            state.d &= mem.read_byte(state.read_reg(state.x));
        }
        Instruction::Xor => {
            state.d ^= mem.read_byte(state.read_reg(state.x));
        }
        Instruction::Add => {
            let value = mem.read_byte(state.read_reg(state.x));
            add_into_d(state, value, false);
        }
        Instruction::SubtractDNoBorrow => {
            let value = mem.read_byte(state.read_reg(state.x));
            subtract_into_d(state, value, state.d, false);
        }
        Instruction::ShiftRight => {
            state.df = state.d & 0x01 != 0;
            state.d >>= 1;
        }
        Instruction::SubtractMemoryNoBorrow => {
            let value = mem.read_byte(state.read_reg(state.x));
            subtract_into_d(state, state.d, value, false);
        }
        Instruction::AddImmediate { value } => {
            add_into_d(state, value, false);
        }
        Instruction::OrImmediate { value } => {
            state.d |= value;
        }
        Instruction::AndImmediate { value } => {
            state.d &= value;
        }
        Instruction::XorImmediate { value } => {
            state.d ^= value;
        }
        Instruction::SubtractDImmediateNoBorrow { value } => {
            subtract_into_d(state, value, state.d, false);
        }
        Instruction::ShiftLeft => {
            state.df = state.d & 0x80 != 0;
            state.d = state.d.wrapping_shl(1);
        }
        Instruction::SubtractMemoryNoBorrowImmediate { value } => {
            subtract_into_d(state, state.d, value, false);
        }
    }
    Ok(())
}

fn restore_xp_from_stack(state: &mut CpuState, mem: &Memory) {
    let idx = state.x & 0x0F;
    let value = mem.read_byte(state.read_reg(idx));
    state.write_reg(idx, state.read_reg(idx).wrapping_add(1));
    state.x = value >> 4;
    state.p = value & 0x0F;
}

fn add_into_d(state: &mut CpuState, value: u8, carry: bool) {
    let sum = state.d as u16 + value as u16 + u16::from(carry);
    state.d = sum as u8;
    state.df = sum > 0xFF;
}

fn subtract_into_d(state: &mut CpuState, minuend: u8, subtrahend: u8, borrow: bool) {
    let subtrahend = subtrahend as u16 + u16::from(borrow);
    let minuend = minuend as u16;
    state.d = minuend.wrapping_sub(subtrahend) as u8;
    state.df = minuend >= subtrahend;
}

fn long_branch_condition_matches(state: &CpuState, condition: LongBranchCondition) -> bool {
    match condition {
        LongBranchCondition::Always => true,
        LongBranchCondition::Q => state.q,
        LongBranchCondition::Zero => state.d == 0,
        LongBranchCondition::DataFlag => state.df,
        LongBranchCondition::NotQ => !state.q,
        LongBranchCondition::NotZero => state.d != 0,
        LongBranchCondition::NotDataFlag => !state.df,
    }
}

fn long_skip_condition_matches(state: &CpuState, condition: LongSkipCondition) -> bool {
    match condition {
        LongSkipCondition::Always => true,
        LongSkipCondition::Q => state.q,
        LongSkipCondition::Zero => state.d == 0,
        LongSkipCondition::DataFlag => state.df,
        LongSkipCondition::NotQ => !state.q,
        LongSkipCondition::NotZero => state.d != 0,
        LongSkipCondition::NotDataFlag => !state.df,
        LongSkipCondition::InterruptEnabled => state.interrupt_enabled,
    }
}
