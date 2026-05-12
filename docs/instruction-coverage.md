# CDP1802 Instruction Coverage

Status: blocker record for saga `full-cdp1802-coverage`, step
`complete-emulator-instruction-subset`.

## Current Emulator Coverage

The emulator executes every instruction currently exposed by the sibling
`sw-cdp1802-isa` crate:

- `IDL`
- `INC Rn`
- `BR addr`
- `B1`, `B2`, `B3`, `B4`
- `BN1`, `BN2`, `BN3`, `BN4`
- `STR Rn`
- `OUT 1` through `OUT 7`
- `INP 1` through `INP 7`
- `REQ`, `SEQ`
- `PLO Rn`, `PHI Rn`
- `LDI imm8`

The current emulator tests now exercise the implemented register-family
instructions across all sixteen scratchpad registers `R0..RF`, rather than
only the demo-visible `R1`.

## Blocking Issue

The broader emulator work cannot proceed cleanly until `sw-cdp1802-isa`
defines and decodes the missing CDP1802 instruction families. The emulator
dispatch currently matches on `sw_cdp1802_isa::Instruction`; adding local
parallel opcode definitions in the emulator would split the architectural
model and make the later assembler work inconsistent.

Per the saga instruction, this step stops at the documented blocker instead
of inventing emulator-only instruction definitions.

## Required Sibling ISA Work

`sw-cdp1802-isa` needs instruction enum variants, decode, encode, display,
and exact-byte tests for these opcode families before the emulator can add
the corresponding execution semantics:

- Register operations:
  - `LDN Rn` for `R1..RF`
  - `DEC Rn`
  - `GLO Rn`
  - `GHI Rn`
  - `SEP Rn`
  - `SEX Rn`
- Memory-reference operations:
  - `LDA Rn`
  - `LDX`
  - `LDXA`
  - `STXD`
  - `IRX`
  - `SAV`
  - `MARK`
- ALU and data-flag operations:
  - `OR`, `AND`, `XOR`
  - `ADD`, `ADC`, `ADI`, `ADCI`
  - `SD`, `SDB`, `SDI`, `SDBI`
  - `SM`, `SMB`, `SMI`, `SMBI`
  - `SHR`/`SHRC`
  - `SHL`/`SHLC`
  - `ORI`, `ANI`, `XRI`
- Short branches and skips:
  - `BQ`, `BNQ`
  - `BZ`, `BNZ`
  - `BDF`, `BNF`
  - `NBR`
- Long branch and long skip group:
  - long unconditional branch and no-long-branch forms
  - long `Q`, zero, data-flag, and interrupt-enable branches/skips
  - long negated variants
- Interrupt/control operations:
  - `RET`
  - `DIS`
  - behavior-facing representation for `MARK`, `SAV`, `T`, and interrupt
    enable changes.

## Emulator Work After ISA Support Lands

Once the ISA crate exposes those instructions, the emulator can add execution
semantics in small slices:

1. Register transfer and selector instructions: `GLO`, `GHI`, `SEP`, `SEX`,
   `DEC`, and `LDN`.
2. Memory-reference instructions: `LDA`, `LDX`, `LDXA`, `STXD`, `IRX`.
3. ALU and `DF` behavior.
4. Short branch/skip completion.
5. Long branch/skip completion.
6. Interrupt/control behavior using the already-modeled `T`, `IE`, and
   interrupt-pending state.
7. DMA-adjacent behavior, including the `R0` address convention, without
   modeling full cycle timing in the first pass.

Each slice should keep the runnable demos passing and include tests that cover
edge cases for all relevant registers, wrapping, `DF`, `P`, `X`, and memory
side effects.
