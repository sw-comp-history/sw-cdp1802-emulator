# sw-cdp1802-emulator

RCA CDP1802 Emulator: instruction execution semantics.

CDP stands for CMOS Digital Processor.

## Brief history

RCA introduced the CDP1802, also known as the COSMAC 1802, in the
mid-1970s as an early CMOS 8-bit microprocessor derived from Joseph
Weisbecker's COSMAC work. Its low-power static CMOS design made it
popular in hobbyist systems such as the COSMAC ELF, ELF-II, and COSMAC
VIP, where programs could be entered from switches or keypads and
observed through LEDs, hex displays, and simple video. Radiation-
hardened and high-reliability versions also made the 1802 notable in
embedded and spacecraft systems, including science instruments and
satellite subsystems.

## Status

`0.1.0` demo subset. The emulator can execute the current sibling ISA
subset: `IDL`, `INC`, `BR`, `B1`..`B4`, `BN1`..`BN4`, `STR`, `OUT`,
`INP`, `REQ`, `SEQ`, `PLO`, `PHI`, and `LDI`.

Run the demo:

```bash
cargo run --example cdp1802_demo
```

Run the multi-register demo:

```bash
cargo run --example multi_register_demo
```

The base and multi-register demos print source, assembler listing, Intel
HEX, assembled bytes, execution result, full CPU state, and memory changes.
The multi-register demo intentionally exercises `R2`, `R3`, `RA`, and `RF`
with the currently supported instruction families.

Run the ELF-style I/O board demo:

```bash
cargo run --example io_board_demo
```

The I/O board demo assembles a small CDP1802 program, runs it with the
`FrontPanel` model, and prints the source, assembler listing, Intel HEX,
machine bytes, final CPU state, front-panel state, RAM bytes at
`0x2000..0x2002`, and a 64 x 32 text video frame. It exercises Q as a
visible LED, EF4 as the input strobe, `OUT 1` as a two-digit hex display
write, `INP 1` as an input latch read, and the RAM-backed video view.

Run the joystick resistor-capacitor timing REPL:

```bash
cargo run --example joystick_rc_demo
```

For a deterministic smoke run:

```bash
cargo run --example joystick_rc_demo -- --once 128 64
```

Assembler artifacts are hidden by default for demo readability. Add
`--source`, `--listing`, or `--hex` to show the source file, assembler
listing, or Intel HEX output:

```bash
cargo run --example joystick_rc_demo -- --listing --hex --once 128 64
```

The joystick demo emulates two analog potentiometer axes in Rust. The
CDP1802 program pulses `OUT 2` for X and `OUT 3` for Y, polls `EF4` to
measure the simulated RC delay, writes a ball pixel into video RAM, and
the terminal output shows joystick timing buckets, the final CPU state,
raw video RAM at `0x2000..0x20ff`, and a 64 x 32 grid rendered with
spaces and a solid block.

Demo assembly source lives under `examples/asm/*.s` and is included by
the Rust examples at compile time with `include_str!`. See
`docs/end-to-end-demos.md` for the demo matrix and coverage note.

## Sibling layout

Cross-crate deps assume sibling clones at
`~/github/sw-langtools/<framework-crate>` and
`~/github/<host-org>/sw-cdp1802-<role>`. See
[`gen-isa/docs/decisions.md`](https://github.com/sw-vibe-coding/gen-isa/blob/main/docs/decisions.md)
Sec 1 for the full org map.

## License

MIT.
