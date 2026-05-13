Step 4 of saga full-cdp1802-coverage: complete assembler mnemonic subset.

Goal: expand the sibling assembler support used by this project from the current demo subset to full CDP1802 mnemonic coverage matching the ISA and emulator.

Scope:
- Work primarily in ../sw-cdp1802-asm and coordinate with ../sw-cdp1802-isa as needed.
- Add tests for R0..RF operands, all opcode families, labels, ranges, and diagnostics.
- Update README and docs/assemblers.md in the assembler repo to reflect actual capability.
- Update emulator demos only after assembler support lands and remains compatible.

Do not add listing or file output formats in this step except where required for tests; those belong to the next step.