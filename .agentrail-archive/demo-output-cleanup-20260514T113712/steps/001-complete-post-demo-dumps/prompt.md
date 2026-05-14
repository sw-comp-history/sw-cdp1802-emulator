Step 1 of saga demo-output-cleanup: complete post-demo dumps.

Goal: improve every runnable demo post-run section so it clearly shows a complete CPU state dump with all modeled state and R0..RF, plus relevant memory buffers.

Scope:
- Work in sw-cdp1802-emulator.
- Review cdp1802_demo, multi_register_demo, io_board_demo, and joystick_rc_demo output sections.
- Ensure post-run labels consistently say final CPU state and include D, DF, P, X, T, Q, EF1..EF4, IE, IRQ pending, halted, instr_count, and R0..RF.
- Ensure each demo shows the relevant memory bytes or buffer after execution. For video demos, include a raw RAM dump in addition to rendered output when useful.
- Add or update focused tests where practical to prevent regressions in complete dump content.

Do not broaden CPU instruction coverage or assembler mnemonic support in this step.