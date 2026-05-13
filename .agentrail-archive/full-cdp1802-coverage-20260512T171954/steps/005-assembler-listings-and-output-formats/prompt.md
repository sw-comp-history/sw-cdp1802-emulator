Step 5 of saga full-cdp1802-coverage: assembler listings and output formats.

Goal: add practical assembler outputs in ../sw-cdp1802-asm: a human-readable listing with addresses, bytes, source lines, labels or symbols, and errors; raw binary output; and at least one loadable text format such as Intel HEX.

Scope:
- Work primarily in ../sw-cdp1802-asm.
- Add public APIs and/or CLI-oriented helpers as appropriate for listing and output generation.
- Include tests for listing layout, raw binary bytes, and Intel HEX or chosen loadable format checksums/records.
- Document how emulator demos consume assembler source and how real or retro hardware workflows can use generated artifacts.

Do not broaden instruction coverage here except for small fixes discovered while testing output formats.