Step 3 of saga full-cdp1802-coverage: complete emulator instruction subset.

Goal: replace the current demo-oriented emulator instruction subset with broad CDP1802 instruction coverage driven by the sibling ISA crate.

Scope:
- Work in sw-cdp1802-emulator and, only if necessary, coordinate with ../sw-cdp1802-isa.
- Implement missing register, memory-reference, ALU, branch/skip, SEP/SEX, long-branch, interrupt, and DMA-adjacent behavior in small coherent slices.
- Add tests that exercise all 16 scratchpad registers R0..RF, not only R0/R1.
- Keep existing demos passing.

If ISA support is missing for an opcode family, stop after documenting the required sibling ISA work rather than inventing parallel opcode definitions.