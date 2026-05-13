Saga: full CDP1802 emulator and assembler coverage follow-up

Step 1: complete-demo-state-dumps
Goal: unify runnable emulator demos so every post-run report prints full CPU state, including D, DF when available, P, X, Q, EF1..EF4, R0..RF, halt state, instruction count, and the same memory/video/board outputs currently shown. Keep the demo source in examples/asm/*.s included at compile time. Add tests or snapshot-style assertions where practical to prevent regressions in the all-register dump format.

Step 2: model-missing-cpu-state
Goal: extend emulator CPU state to represent missing CDP1802 architectural state that is not currently modeled: DF, T, interrupt enable/interrupt bookkeeping, and any state needed for later DMA/interrupt instruction coverage. Update docs/io-board.md and postmortem notes so they distinguish implemented state from planned state. Keep behavior compatible with existing demos.

Step 3: complete-emulator-instruction-subset
Goal: replace the current demo-oriented emulator instruction subset with broader CDP1802 instruction coverage driven by the sibling ISA crate. Implement and test the missing register, memory reference, ALU, branch/skip, SEP/SEX, long branch, interrupt, and DMA-adjacent behavior in small reviewed slices. Ensure all 16 registers are exercised by tests, not only R0/R1.

Step 4: complete-assembler-mnemonic-subset
Goal: expand the sibling assembler support used by this project from the current demo subset to full CDP1802 mnemonic coverage matching the ISA/emulator. Coordinate changes with ../sw-cdp1802-asm and ../sw-cdp1802-isa, including tests for R0..RF operands, all opcode families, labels, ranges, and diagnostics. Update emulator demos only after assembler support lands.

Step 5: assembler-listings-and-output-formats
Goal: add practical assembler outputs in ../sw-cdp1802-asm: a human-readable listing with addresses, bytes, source lines, labels/symbols, and errors; raw binary output; and at least one loadable text format such as Intel HEX. Document how the emulator demos consume assembler source and how real or retro hardware workflows can use the generated artifacts.

Step 6: end-to-end-full-coverage-demos
Goal: add or refresh runnable demos that use the expanded assembler and emulator coverage, showing source, assembled bytes/listing, execution, full post-run CPU state, memory/register changes, and I/O/video output. Include one demo that intentionally exercises multiple scratchpad registers beyond R0/R1 and one I/O demo that still mirrors the ELF/VIP style board model.