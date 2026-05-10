Step 8 of saga `cdp1802-emulator-asm-demo`: I/O bits Q and EF.

Goal: implement the first CPU-visible CDP1802 I/O bits in `sw-cdp1802-isa`, `sw-cdp1802-asm`, and `sw-cdp1802-emulator` as needed.

Read first:
- `sw-cdp1802-emulator/docs/io-board.md`
- `sw-cdp1802-emulator/docs/demo-contract.md`
- Existing CDP1802 ISA/asm/emulator tests.

Work for this step:
1. Add CPU state for `Q` and `EF1..EF4` in the emulator.
2. Implement only `SEQ`, `REQ`, and EF branch instructions needed to test/set/read these bits.
3. Update ISA encode/decode and assembler support for only these opcodes.
4. Add unit tests for Q set/reset and EF branch behavior.
5. Do not implement `INP`, `OUT`, DMA, keypad, video, timing, or extra unrelated opcodes in this step.
6. Run relevant cargo tests in each touched repo, commit changes, and complete the step.
