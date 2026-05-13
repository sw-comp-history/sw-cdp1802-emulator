Step 2 of saga full-cdp1802-coverage: model missing CPU state.

Goal: extend emulator CPU state to represent missing CDP1802 architectural state that is not currently modeled, especially DF, T, interrupt enable/interrupt bookkeeping, and any state needed for later DMA or interrupt instruction coverage.

Scope:
- Work primarily in sw-cdp1802-emulator CPU state and docs.
- Preserve compatibility with existing demos and tests.
- Update docs/io-board.md and related postmortem/demo docs so they clearly separate implemented state from planned state.
- Add unit tests for default values and state mutation helpers.

Do not implement the full instruction set in this step; add the state shape needed for that later work.