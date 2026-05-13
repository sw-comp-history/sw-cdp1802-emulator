# CDP1802 End-to-End Demos

Status: current runnable demo guide for saga `full-cdp1802-coverage`.

The demos use assembly source files in `examples/asm/*.s` and include them in
Rust examples with `include_str!`. Each demo assembles source at runtime, runs
the emulator, and prints final CPU state. Most demos show assembler artifacts by
default; the joystick demo keeps them behind explicit flags for cleaner REPL
output.

## Base Memory Demo

```bash
cargo run --example cdp1802_demo
```

Shows source, listing, Intel HEX, assembled bytes, full CPU state, and RAM
`0x2000..0x2002`.

## Multi-Register Demo

```bash
cargo run --example multi_register_demo
```

Exercises multiple scratchpad registers beyond `R0` and `R1`: `R2`, `R3`,
`RA`, and `RF`. It writes bytes to `0x2010`, `0x2020`, `0x2030`, and
`0x2040`, then prints the full CPU state and selected RAM locations.

This demo intentionally stays within the instruction families currently
available from `sw-cdp1802-isa` and `sw-cdp1802-asm`.

## ELF-Style I/O Board Demo

```bash
cargo run --example io_board_demo
```

Mirrors an ELF/VIP-style board model with a front panel, Q LED, EF4 input
strobe, port-1 input/output behavior, RAM-backed video bytes, full CPU state,
and a 64 x 32 text video frame.

## Joystick RC Timing Demo

```bash
cargo run --example joystick_rc_demo
```

Deterministic single-frame run:

```bash
cargo run --example joystick_rc_demo -- --once 128 64
```

Show assembler artifacts when needed:

```bash
cargo run --example joystick_rc_demo -- --source --listing --hex --once 128 64
```

Rust emulates the joystick potentiometers and resistor-capacitor timing
circuits. The 1802 assembly program pulses output ports, polls EF4, writes a
ball pixel to RAM-backed video, and the terminal renders the 64 x 32 grid.

Default output shape:

- joystick X/Y delay buckets
- instruction count and halt status
- `--- final CPU state ---` with D, DF, P, X, T, Q, EF1..EF4, IE, IRQ pending,
  halted, instruction count, and R0..RF
- ball bucket and final R1 video address
- raw `0x2000..0x20ff` video RAM dump
- rendered 64 x 32 grid using spaces for clear pixels and `█` for the ball

## Coverage Note

Assembler listing and Intel HEX output are available in the runnable demo set,
with the joystick demo exposing them through explicit flags. Full CDP1802
instruction coverage remains blocked until `sw-cdp1802-isa` defines the
remaining opcode families. See `docs/instruction-coverage.md`.
