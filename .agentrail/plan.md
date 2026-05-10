# CDP1802 Emulator + Assembler Demo Saga

Goal: turn the generated CDP1802 skeletons into a demonstrable assembler/emulator path: assemble a small RCA CDP1802 program, load it into the emulator, execute it deterministically, and verify observable machine state and/or output.

Context to preserve for every step:
- Read `~/github/sw-vibe-coding/gen-isa` first, especially `README.md`, `docs/porting-guide.md`, `docs/spec-format.md`, `docs/postmortem-1130-bringup.md`, and `docs/spec-examples/ibm1130.toml` for the scaffolder pattern and lessons from the IBM 1130 bring-up.
- Treat the IBM 1130 repos as worked examples: `~/github/sw-comp-history/sw-ibm1130-{isa,target,codegen,asm,emulator}`. Use them for shape, tests, demo discipline, and saga pacing; do not blindly copy ISA-specific semantics.
- Related CDP1802 repos are siblings: `sw-cdp1802-isa`, `sw-cdp1802-target`, `sw-cdp1802-codegen`, `sw-cdp1802-asm`, and this `sw-cdp1802-emulator` repo.
- The current generated skeleton says implementation is pending saga steps 7-11; for this focused saga, prioritize the assembler/emulator demonstration over full codegen completeness.
- Keep each step narrow, commit before `agentrail complete`, and do not hand-edit `.agentrail/`.

Proposed next steps:

1. `decisions-and-demo-contract` - Decide the smallest convincing CDP1802 demo: instruction subset, memory map, reset/entry behavior, halt convention, assembler syntax subset, and success criteria. Produce a short docs record in this repo that references `gen-isa` and the IBM 1130 examples.
2. `isa-readiness-check` - Validate `sw-cdp1802-isa` has the opcode/decode/encode surface needed for the demo. Fill only the subset required for the demo if still skeletal, with round-trip tests.
3. `assembler-minimum-demo-subset` - Implement or tighten `sw-cdp1802-asm` for labels, numeric literals, byte emission, and the demo instructions. Add assembler tests that produce exact bytes.
4. `emulator-core-subset` - Implement CDP1802 state, memory access, fetch/decode/execute stepping, and the demo instruction semantics in this repo. Include tests for each instruction used by the demo.
5. `assemble-run-integration` - Add an integration test that assembles demo source with the sibling assembler, loads bytes into emulator memory, runs to the chosen halt condition, and asserts final state.
6. `demo-runner-and-docs` - Add a small example or test fixture that prints the demo source, machine bytes, trace/final registers, and result, mirroring the IBM 1130 demo discipline.
7. `postmortem-and-followups` - Capture what generalized cleanly from the IBM 1130 bring-up, what CDP1802 forced differently, and which follow-up saga should handle full codegen/frontend work.
