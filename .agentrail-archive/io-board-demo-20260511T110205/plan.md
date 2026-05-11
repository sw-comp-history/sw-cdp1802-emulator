# CDP1802 I/O Board Demo Saga

Goal: build runnable demonstrations that simulate ELF-style CDP1802
I/O board behavior using the existing assembler, emulator,
front-panel, joystick, and RAM-backed video pieces.

Context to preserve for every step:

- Read `docs/io-board.md` first. It is the board contract for Q, EF4,
  port 1, and the 64 x 32 RAM-backed video view.
- Use the existing runnable demo discipline from
  `examples/cdp1802_demo.rs`: print source, assembled bytes, execution
  result, registers, and observable output.
- Use `~/github/sw-vibe-coding/gen-isa` and the IBM 1130 examples as
  process references, especially for exact-byte tests and readable demo
  output. Do not copy IBM 1130 device semantics.
- Keep the base demo scoped to the already implemented substrate:
  `SEQ`, `REQ`, `B4`/`BN4`, `INP 1`, `OUT 1`, `FrontPanel`, and
  `VideoView::elf_64x32()`. Joystick follow-up work may add a
  Rust-emulated resistor-capacitor joystick circuit and the minimum
  board hooks needed for 1802 assembly to measure it.
- Do not add CDP1861 DMA/timing, interrupts, serial/cassette, UI
  graphics, or unrelated opcodes unless a step explicitly revises the
  scope.

Planned steps:

1. `io-board-demo-contract` - Define the demo program, expected board
   inputs, expected final CPU/front-panel/video state, and output shape.
   Keep this docs-only unless tiny fixture constants are clearly useful.
2. `runnable-io-board-demo` - Add a runnable example that assembles the
   demo source, initializes front-panel input, runs the emulator, and
   prints board state plus a text video frame.
3. `io-board-demo-regression-docs` - Add a regression test for the demo
   and README/docs instructions that show how to run it and what output
   to expect.
4. `joystick-rc-repl-demo` - Implement a Rust-emulated two-axis
   resistor-capacitor joystick circuit plus a terminal REPL. The REPL
   prompts for joystick X/Y values, runs 1802 assembly that pulses
   output pins and polls input pins to measure the simulated RC delay,
   updates a ball in the RAM-backed video buffer, and renders the grid
   using spaces for clear pixels and a solid block for set pixels.
