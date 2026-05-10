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

`0.1.0` demo subset. The emulator can execute the first assembler/
emulator contract program for `IDL`, `INC`, `BR`, `STR`, `PLO`, `PHI`,
and `LDI`.

Run the demo:

```bash
cargo run --example cdp1802_demo
```

Run the ELF-style I/O board demo:

```bash
cargo run --example io_board_demo
```

The I/O board demo assembles a small CDP1802 program, runs it with the
`FrontPanel` model, and prints the source, machine bytes, final
registers, front-panel state, RAM bytes at `0x2000..0x2002`, and a
64 x 32 text video frame. It exercises Q as a visible LED, EF4 as the
input strobe, `OUT 1` as a two-digit hex display write, `INP 1` as an
input latch read, and the RAM-backed video view.

## Sibling layout

Cross-crate deps assume sibling clones at
`~/github/sw-langtools/<framework-crate>` and
`~/github/<host-org>/sw-cdp1802-<role>`. See
[`gen-isa/docs/decisions.md`](https://github.com/sw-vibe-coding/gen-isa/blob/main/docs/decisions.md)
Sec 1 for the full org map.

## License

MIT.
