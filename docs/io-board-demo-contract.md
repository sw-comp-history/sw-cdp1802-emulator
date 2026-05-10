# CDP1802 I/O Board Demo Contract

Status: saga step 1 planning record. This file defines the runnable
ELF-style I/O board demonstration for the existing CDP1802 assembler
and emulator substrate.

## Objective

The demo should prove this path:

```text
CDP1802 assembly source -> assembled bytes -> emulator + FrontPanel -> run -> board state + video text
```

The program uses only already implemented CPU and board features:
`SEQ`, `REQ`, `B4`, `INP 1`, `OUT 1`, `FrontPanel`, and
`VideoView::elf_64x32()`.

## Initial Machine And Board State

Initial CPU state:

- Program loaded at `0x0000`.
- `P = 0`, `R0 = 0x0000`.
- `X = 1`, so `OUT 1` and `INP 1` use `R1`.
- All registers and RAM initially zero except loaded program bytes.

Initial front-panel state:

- `input_latch = 0x3c`.
- `keypad = 0x0c`.
- `input_pressed = true`, mirrored to `EF4`.
- `hex_display = 0x00`.
- `q_led = false`.

## Demo Program

Assembly source target:

```asm
        ORG 0x0000
        SEQ
        LDI 0x20
        PHI R1
        LDI 0x00
        PLO R1
        LDI 0xaa
        STR R1
        OUT 1
        LDI 0x55
        STR R1
        OUT 1
        B4 INPUT
        REQ
INPUT:  INP 1
        IDL
```

Program behavior:

- `SEQ` turns on the Q output; the board exposes this as `q_led`.
- `R1` is initialized to the video base address `0x2000`.
- `0xaa` is stored at `0x2000`; `OUT 1` displays it and increments
  `R1` to `0x2001`.
- `0x55` is stored at `0x2001`; `OUT 1` displays it and increments
  `R1` to `0x2002`.
- `B4 INPUT` branches because `input_pressed = true` maps to `EF4`.
  Therefore `REQ` is skipped and Q remains set.
- `INP 1` reads `input_latch` (`0x3c`) into `D` and `M(R1)`, writing
  `0x3c` to `0x2002`.
- `IDL` halts the demo.

Expected assembled bytes:

```text
7b f8 20 b1 f8 00 a1 f8 aa 51 61 f8 55 51 61 37
12 7a 69 00
```

The `B4` target byte is `0x12` because `INPUT` is at byte address
`0x0012`.

## Expected Final State

With the initial state above and a bounded step limit of `100`, the
program should execute 14 instructions and halt.

Expected CPU state:

- `halted = true`.
- `D = 0x3c`.
- `P = 0`.
- `X = 1`.
- `R1 = 0x2002`.
- `Q = true`.
- `EF4 = true`.

Expected front-panel state:

- `hex_display = 0x55`.
- `input_latch = 0x3c`.
- `keypad = 0x0c`.
- `input_pressed = true`.
- `q_led = true`.

Expected RAM state:

- `0x2000 = 0xaa`.
- `0x2001 = 0x55`.
- `0x2002 = 0x3c`.

Expected video state:

- The demo uses `VideoView::elf_64x32()` over `0x2000..0x20ff`.
- Row 0 begins with bytes `aa 55 3c`.
- With the documented MSB-first bit order, row 0 should begin:

```text
#.#.#.#..#.#.#.#..####..
```

All remaining video bytes are zero unless the runnable example writes
additional decorative data.

## Runnable Example Output Shape

The runnable example should mirror `examples/cdp1802_demo.rs` and print:

- Demo title.
- Assembly source.
- Assembled byte dump.
- Initial board inputs.
- Run result: instruction count and halt state.
- Final registers: at least `D`, `P`, `X`, `R1`, `Q`, and `EF4`.
- Front-panel outputs: at least `hex_display`, `q_led`, `input_latch`,
  `keypad`, and `input_pressed`.
- Relevant RAM bytes at `0x2000..0x2002`.
- Text video frame rendered with `#` for set pixels and `.` for clear
  pixels.

## Out Of Scope

Do not add CDP1861/Pixie DMA timing, interrupts, serial/cassette,
composite-video behavior, UI graphics, or new CPU opcodes for this
demo. The goal is a deterministic board simulation using the existing
emulator surface.
