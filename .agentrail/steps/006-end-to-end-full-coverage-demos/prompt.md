Step 6 of saga full-cdp1802-coverage: end-to-end full coverage demos.

Goal: add or refresh runnable demos that use the expanded assembler and emulator coverage, showing source, assembled bytes or listing, execution, full post-run CPU state, memory and register changes, and I/O or video output.

Scope:
- Work in sw-cdp1802-emulator after the emulator and assembler coverage steps have landed.
- Include one demo that intentionally exercises multiple scratchpad registers beyond R0/R1.
- Include one I/O demo that still mirrors the ELF/VIP style board model.
- Keep assembly source in examples/asm/*.s and include it at compile time from Rust examples.
- Update README and docs so users can run each demo and understand expected post-run state.

This is the integration step; do not start it until the previous coverage and output-format work is available.