Step 2 of saga demo-output-cleanup: joystick demo output cleanup.

Goal: improve the joystick RC demo output for human demo use.

Scope:
- Work in sw-cdp1802-emulator.
- Hide Intel HEX by default, or move assembler artifacts behind explicit flags such as --listing and --hex.
- Keep --once X Y for deterministic demo runs.
- Make the full CPU state plainly visible after the run.
- Add a raw video RAM dump for 0x2000..0x20ff, or a clearly documented relevant frame slice, before the rendered 64x32 grid.
- Keep the rendered grid with spaces and a solid block.
- Update README.md and docs/end-to-end-demos.md with the new command-line flags and expected output shape.
- Validate with cargo test and cargo run --example joystick_rc_demo -- --once 128 64.

Do not broaden CPU instruction coverage or assembler mnemonic support in this step.