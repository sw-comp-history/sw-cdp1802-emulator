Step 1 of saga full-cdp1802-coverage: complete demo state dumps.

Goal: unify runnable emulator demos so every post-run report prints full CPU state, including D, DF when available, P, X, Q, EF1..EF4, R0..RF, halt state, and instruction count, while preserving the existing memory, video, and board outputs.

Scope:
- Work in sw-cdp1802-emulator only.
- Add a shared helper for formatting or printing CPU state instead of duplicating per-demo register println calls.
- Update examples/cdp1802_demo.rs, examples/io_board_demo.rs, and examples/joystick_rc_demo.rs as applicable.
- Keep assembler sources in examples/asm/*.s included at compile time.
- Add focused tests or assertions where practical so future demos do not regress to partial register dumps.

Do not expand emulator instruction coverage in this step; this step is presentation and demo consistency only.