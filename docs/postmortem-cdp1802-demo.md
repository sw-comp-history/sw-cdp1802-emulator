# Postmortem: CDP1802 Assembler/Emulator Demo

Status: postmortem record for saga `cdp1802-emulator-asm-demo`.
ASCII-only.

## Summary

This saga delivered the first working CDP1802 assembler/emulator path:

```text
CDP1802 assembly source -> assembled bytes -> emulator memory -> run -> final RAM/register state
```

The demo program initializes `R1` to `0x2000`, stores `0x42`, `0x43`,
and `0x44` through that register, branches to `DONE`, and halts with
`IDL`. The runnable example prints the assembly source, assembled byte
dump, instruction count, halt state, final registers, and RAM
`0x2000..0x2002`.

## Delivered Behavior

- `sw-cdp1802-isa` now has a shared demo instruction model for `IDL`,
  `INC`, `BR`, `STR`, `PLO`, `PHI`, and `LDI`.
- `sw-cdp1802-asm` now assembles the demo subset with `ORG`, colon
  labels, decimal and `0x` hex literals, and `R0..R15` operands.
- `sw-cdp1802-emulator` now models 64 KiB byte-addressed memory,
  `R0..R15`, `D`, `DF`, `P`, `X`, `T`, `Q`, `EF1..EF4`, `IE`, interrupt
  pending state, a halt flag, instruction stepping, bounded run, and
  the current demo opcode subset.
- `tests/run_demo.rs` proves the full path by assembling source,
  asserting exact bytes, loading RAM, running to `IDL`, and checking
  RAM/register post-state.
- `examples/cdp1802_demo.rs` provides a human-readable runnable demo
  comparable to the IBM 1130 example pattern.
- `docs/io-board.md` captures the 1802 register and I/O-board model
  needed for the next ELF-style hardware saga.

## Commit Index

Related commits:

- `sw-cdp1802-isa`: `603a1b7 Add CDP1802 demo ISA subset`
- `sw-cdp1802-asm`: `1bda911 Add CDP1802 demo assembler subset`
- `sw-cdp1802-emulator`: `c319f93 Define CDP1802 demo contract`
- `sw-cdp1802-emulator`: `4eeb698 Add CDP1802 emulator core subset`
- `sw-cdp1802-emulator`: `c7d4924 Add CDP1802 assemble-run integration`
- `sw-cdp1802-emulator`: `1f7ad7b Add runnable CDP1802 demo example`
- `sw-cdp1802-emulator`: `d678373 Plan CDP1802 ELF-style I/O board work`

## Validation

Validation commands for this repo:

```bash
markdown-checker -f README.md
markdown-checker -p docs -f "**/*.md"
cargo fmt --check
cargo test
cargo run --example cdp1802_demo
```

Expected demo result:

- Assembled bytes:
  `f8 20 b1 f8 00 a1 f8 42 51 11 f8 43 51 11 f8 44 51 30 13 00`
- Instruction count: `14`.
- Halted: `true`.
- Final `D`: `0x44`.
- Final `P`: `0x0`.
- Final `X`: `0x0`.
- Final `R1`: `0x2002`.
- RAM `0x2000..0x2002`: `42 43 44`.

The sibling ISA and assembler repos were validated during their
respective implementation steps with `cargo test`.

## What Generalized From IBM 1130

The useful parts of the IBM 1130 bring-up generalized cleanly:

- Agentrail kept the cross-repo work explicit enough to avoid losing
  which repo owned each layer.
- The first demo was defined as a contract before implementation.
- Exact-byte assembler tests remained the best way to pin down a small
  subset before adding broader syntax.
- The assemble-load-run integration test gave stronger evidence than
  isolated unit tests alone.
- The runnable example format is reusable: show source, assembled
  bytes, execution result, registers, and memory that changed.

The CDP1802 path was smaller than the IBM 1130 path because it did not
include target ABI, codegen, linker, or device output work. That was the
right scope for proving the assembler/emulator loop quickly.

## What Differed For CDP1802

The CDP1802 brought different constraints than the IBM 1130:

- Memory is byte-addressed, so the demo asserts byte sequences and byte
  RAM ranges rather than word-addressed state.
- The architecture has sixteen 16-bit scratchpad registers. The active
  program counter is selected by `P`, and the active data/index register
  is selected by `X`.
- Instructions in the demo subset are one or two bytes, and `BR` uses a
  page-local low-byte target.
- `IDL` is a convenient halt convention for this emulator demo.
- Hardware interaction is centered on `Q`, `EF1..EF4`, `INP`, `OUT`,
  DMA, and board wiring rather than an IBM 1130-style XIO device model.

## Limitations

Current limitations are intentional:

- The emulator implements a demo-oriented opcode subset, not the full
  instruction set.
- `DF`, `T`, `IE`, and interrupt-pending state are represented, but ALU
  carry/borrow behavior, interrupt entry/return behavior, DMA, and
  CDP1861/Pixie timing are not implemented yet.
- The assembler has no expressions, includes, macros, disassembler, or
  object format.
- The emulator has no cycle timing.
- Unsupported opcodes fail through decode/execution errors rather than
  emulating undefined hardware behavior.

## Next Recommended Saga Work

The next work should be the ELF-style I/O board path documented in
`docs/io-board.md`.

Recommended order:

1. `io-bits-q-ef`: implement `Q`, `EF1..EF4`, `SEQ`, `REQ`, and EF
   branch opcodes with focused unit tests.
2. `io-ports-front-panel`: implement an `INP`/`OUT` subset plus a
   simple ELF-style board model for one visible LED bit, two hex display
   digits, keypad/input latch, and input strobe.
3. `memory-bitmap-video`: implement a deterministic 64 x 32 RAM-backed
   monochrome video view and text renderer.

CDP1861/Pixie DMA timing should stay out of the next step. A simple
RAM-backed bitmap gives useful demos first, and the cycle-accurate
video path can be a later historical-fidelity saga.
