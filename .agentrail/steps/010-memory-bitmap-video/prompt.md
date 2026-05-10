Step 10 of saga `cdp1802-emulator-asm-demo`: memory bitmap video view.

Goal: implement a simple RAM-backed monochrome display view for ELF/Pixie-style demos without cycle-accurate CDP1861 DMA.

Read first:
- `sw-cdp1802-emulator/docs/io-board.md`
- Prior I/O bits and front-panel steps.

Work for this step:
1. Add a deterministic 64 x 32 monochrome video view over a documented RAM range, initially `0x2000..0x20ff` unless the prior steps changed the memory map.
2. Define and test bit order and row layout.
3. Add a text renderer using `#` for set pixels and `.` for clear pixels.
4. Add tests that write bytes to video RAM and assert rendered text output.
5. Do not implement CDP1861 cycle/DMA timing, interrupts, real composite-video behavior, or UI graphics in this step.
6. Run relevant cargo tests, commit changes plus agentrail metadata, and complete the step.
