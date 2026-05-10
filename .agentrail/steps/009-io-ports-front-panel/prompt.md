Step 9 of saga `cdp1802-emulator-asm-demo`: ELF-style ports and front panel.

Goal: implement a simple, deterministic ELF-style board model around CDP1802 `INP`/`OUT` behavior.

Read first:
- `sw-cdp1802-emulator/docs/io-board.md`
- Step 8 Q/EF implementation and tests.

Work for this step:
1. Implement the minimal `INP n` / `OUT n` CPU behavior required for a front-panel demo.
2. Add an ELF-style board/front-panel model with `input_latch`, `hex_display`, `keypad`, `input_pressed`, and visible Q LED state.
3. Pick and document a narrow port mapping, e.g. `OUT 1` for the two-digit hex display and `INP 1` for keypad/input latch, with `EF4` as input strobe.
4. Update ISA/asm/emulator tests and add a small assemble-run test for the front-panel path.
5. Do not implement video, DMA, interrupts, cassette, serial, timing, or extra unrelated opcodes in this step.
6. Run relevant cargo tests in each touched repo, commit changes, and complete the step.
