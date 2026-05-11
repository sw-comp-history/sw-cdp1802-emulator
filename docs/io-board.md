# CDP1802 ELF-Style I/O Board Plan

Status: planning record. This document captures the CDP1802 register
model and a practical I/O-board emulation plan for COSMAC ELF / ELF-II
style systems.

## Register Model

The CDP1802 has sixteen 16-bit scratchpad registers:

```text
R0 R1 R2 R3 R4 R5 R6 R7 R8 R9 RA RB RC RD RE RF
```

The current emulator models these as `R0..R15`.

Important surrounding CPU state:

- `D`: 8-bit accumulator / data register.
- `DF`: 1-bit data flag, used as carry/borrow by ALU operations.
- `P`: 4-bit selector for which scratchpad register is the program
  counter.
- `X`: 4-bit selector for which scratchpad register is the data/index
  register for several memory and I/O operations.
- `N`: 4-bit instruction register field, normally the low nibble of
  the current opcode.
- `T`: 8-bit temporary register used by interrupt/save behavior.
- `IE`: interrupt-enable bit.

Special register conventions:

- `R0` is used by the built-in DMA controller.
- `R1` is commonly used as the interrupt handler program counter.
- `R2` is commonly used as a stack pointer by software conventions.
- Any register can be selected as the active program counter with
  `SEP Rn`.
- Any register can be selected as the active data/index register with
  `SEX Rn`.

The current demo emulator implements `R0..R15`, `D`, `P`, `X`, and a
halt flag. It does not yet implement `DF`, `Q`, `EF1..EF4`, `T`, `IE`,
interrupts, DMA, or I/O ports.

## Historical I/O Shape

The 1802 was commonly wired directly to small-board hardware. Instead
of a standardized device model, boards decoded CPU pins and I/O port
instructions into simple latches, LEDs, keypad rows, cassette
interfaces, speakers, and video support chips.

Core CPU-visible mechanisms:

- `Q`: one software-controlled output bit. `SEQ` sets it and `REQ`
  resets it. Hobby systems used it for an LED, speaker, cassette
  output, or serial output.
- `EF1..EF4`: four external flag input pins. Branch instructions can
  test each flag in either polarity. ELF-style systems often used an
  EF line for the input pushbutton or serial/cassette input state.
- `INP n` / `OUT n`: seven 8-bit input and output port instruction
  families. Board hardware decides what each port means.
- DMA: the 1802 can transfer bytes between memory and external hardware
  without routing them through `D`; `R0` supplies the DMA address.
- CDP1861 Pixie video: a companion monochrome video chip that used the
  1802 DMA path to shift bitmap bytes to video. Common software modes
  included 64 x 128, 64 x 64, and 64 x 32. The 64 x 32 mode uses 256
  bytes and is the easiest model for CHIP-8-like display behavior.

An ELF-II style trainer commonly had:

- Two hexadecimal LED displays for one byte of visible output.
- Hex keypad or switch input.
- An input button / strobe visible to software through an EF line.
- Optional Pixie-style monochrome video backed by a region of memory.

## Easy Emulation Targets

The useful path is to model board-observable behavior, not full
electrical timing.

### Phase 1: CPU I/O Bits

Add CPU state and opcodes for:

- `Q` output bit.
- `EF1..EF4` input bits.
- `SEQ` and `REQ`.
- EF branch instructions.

This enables tests for LED/speaker/cassette-style bit output and
front-panel button polling. It is small and directly useful.

### Phase 2: Simple Ports And Front Panel

Add a small board model, for example:

```text
Board {
  input_latch: u8,
  hex_display: u8,
  keypad: u8,
  input_pressed: bool,
}
```

Map the first simple ports as emulator policy, not as universal 1802
truth:

- `OUT 1`: write the byte output by the CPU to the two-digit hex
  display.
- `INP 1`: read the keypad/input latch byte into the CPU input path.
- `EF4`: report input button / strobe state.
- `Q`: expose one visible LED bit.

The exact `INP`/`OUT` data movement must follow the 1802 instruction
manual when those opcodes are implemented. The board mapping should
live above that CPU behavior so alternate boards can choose different
port meanings later.

The current front-panel implementation uses this narrow mapping:

- CPU `OUT 1` outputs `M(R(X))`, increments `R(X)`, and updates
  `FrontPanel::hex_display`.
- CPU `INP 1` reads `FrontPanel::input_latch`, writes it to
  `M(R(X))`, and copies it into `D`.
- `FrontPanel::input_pressed` is mirrored to CPU `EF4` before each
  front-panel step.
- CPU `Q` is mirrored to `FrontPanel::q_led`.
- Ports 2 through 7 are decoded by the CPU, but the simple front panel
  leaves them unmapped.

### Phase 3: Memory Bitmap Video View

Add a simple video view over RAM:

```text
base: 0x2000
width: 64 pixels
height: 32 pixels
bytes: 256
bit order: most-significant bit first within each byte
row layout: 8 consecutive bytes per row, 32 rows
```

This is not cycle-accurate CDP1861 DMA. It is an easy framebuffer view
that lets demos show memory as pixels, similar to how ELF/VIP/Pixie
software was experienced.

The user-facing renderer can be plain text first:

```text
# for set pixel
. for clear pixel
```

The current implementation provides `VideoView::elf_64x32()` over
`0x2000..0x20ff`. Pixel `(0, 0)` is bit 7 of byte `0x2000`, pixel
`(7, 0)` is bit 0 of byte `0x2000`, pixel `(8, 0)` is bit 7 of byte
`0x2001`, and pixel `(0, 1)` is bit 7 of byte `0x2008`. The text
renderer emits exactly 32 lines of 64 characters with no trailing
newline.

A later UI can scale each source bit into 2x2 or 4x4 display pixels.

### Phase 4: Pixie/DMA Fidelity

Only after simple board I/O works, add more faithful CDP1861 behavior:

- DMA cycles using `R0`.
- Display enable / blanking behavior.
- Interrupt timing and display-service routines.
- 64 x 128 and reduced-resolution row-repeat behavior.

This is valuable for historical accuracy, but not needed for the first
ELF-II-style board demo.

### Joystick RC Timing Demo

The joystick REPL demo models a simple two-axis analog joystick circuit
like an ELF-II hobby add-on:

- `OUT 2`: pulse the X-axis resistor-capacitor timing circuit.
- `OUT 3`: pulse the Y-axis resistor-capacitor timing circuit.
- `EF4`: report whether the currently pulsed axis has reached the
  input threshold.

Rust owns the analog side of the model. Each axis is a deterministic
`0..255` potentiometer value mapped into four delay buckets. The CDP1802
program owns the measurement and ball movement: it pulses an axis,
polls `EF4` with `B4`, chooses a row/column bucket from the observed
delay, writes a ball pixel into the `VideoView::elf_64x32()` RAM buffer,
and halts for the REPL to render the screen.

The terminal renderer uses spaces for clear pixels and a solid block for
set pixels. There is still no CDP1861/Pixie DMA timing, interrupt, or
composite-video behavior in this model.

## Recommended Saga Additions

Add implementation steps after the current assembler/emulator demo saga
postmortem:

1. `io-bits-q-ef`: implement `Q`, `EF1..EF4`, `SEQ`, `REQ`, and EF
   branch opcodes with unit tests.
2. `io-ports-front-panel`: implement `INP`/`OUT` subset and an
   ELF-style front-panel board model for hex display/keypad/input
   strobe.
3. `memory-bitmap-video`: implement a deterministic 64 x 32 RAM-backed
   monochrome video view and a text renderer.

Keep CDP1861 cycle/DMA accuracy as a later saga after those three steps.

## References

- RCA CDP1802 COSMAC microprocessor user manual.
- COSMAC ELF / ELF-II history and hardware descriptions.
- CDP1861 Pixie video-display documentation and emulator references.
