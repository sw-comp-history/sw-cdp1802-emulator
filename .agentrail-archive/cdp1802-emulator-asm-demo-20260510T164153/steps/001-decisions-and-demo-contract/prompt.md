Step 1 of saga `cdp1802-emulator-asm-demo`: decisions and demo contract.

Goal: define the smallest convincing CDP1802 assembler/emulator demonstration before implementing code.

Read first:
- `~/github/sw-vibe-coding/gen-isa/README.md`
- `~/github/sw-vibe-coding/gen-isa/docs/porting-guide.md`
- `~/github/sw-vibe-coding/gen-isa/docs/spec-format.md`
- `~/github/sw-vibe-coding/gen-isa/docs/postmortem-1130-bringup.md`
- IBM 1130 worked examples in `~/github/sw-comp-history/sw-ibm1130-{isa,asm,emulator}`
- CDP1802 sibling skeletons in `~/github/sw-comp-history/sw-cdp1802-{isa,target,codegen,asm}`

Work for this step:
1. Inspect the CDP1802 emulator repo and sibling CDP1802 ISA/asm repos enough to know what is already skeletal versus implemented.
2. Create or update a short docs file in this repo that records:
   - demo program objective,
   - exact instruction subset needed,
   - reset/entry address and memory layout,
   - halt/stop convention,
   - assembler syntax subset for the demo,
   - observable success criteria,
   - references to the `gen-isa` scaffolder docs and IBM 1130 examples.
3. Do not implement emulator or assembler semantics in this step. Keep it as a planning/contract step.
4. Run markdown validation if available and commit the docs plus `.agentrail/` changes.
5. Complete the step with `agentrail complete --summary ... --reward 1 --actions ...`.

Expected next step after this one: `isa-readiness-check`, focused on making `sw-cdp1802-isa` encode/decode enough of the demo subset with round-trip tests.
