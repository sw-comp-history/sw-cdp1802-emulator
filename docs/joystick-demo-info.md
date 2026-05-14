# Joystick RC Demo: Correct Emulation Architecture

## The Real Hardware

On a real COSMAC ELF or ELF-II with 256 bytes of RAM, there is no
separate video memory. The entire 256-byte address space (0x0000–0x00FF)
is shared between program code and the CDP1861 Pixie video display.

The 1861 uses DMA to read all 256 bytes every frame. The memory layout
for 64×32 monochrome display is:

```text
Row  0: bytes 0x00–0x07  (8 bytes × 8 pixels = 64 pixels per row)
Row  1: bytes 0x08–0x0F
Row  2: bytes 0x10–0x17
  ...
Row 31: bytes 0xF8–0xFF
```

Bit order is MSB-first within each byte: bit 7 is the leftmost pixel,
bit 0 is the rightmost.

The program code occupies some portion of this same address space. Where
code bytes happen to fall, they appear as visual noise on screen. The
joystick demo program is small enough that most of the display is
available for the ball.

**Self-modifying code is a feature, not a bug.** If the ball position
happens to overlap a program byte, the pixel write overwrites code. On
the next iteration, the CPU executes corrupted instructions and the
program crashes or behaves unpredictably. This is exactly what happened
on the real hardware. It is part of the demo's educational value: the
player learns that code and data share the same memory, and that
overwriting code has consequences. The emulator must not protect against
this.

## What the Current Implementations Get Wrong

### Emulator repo (`examples/asm/joystick_rc_demo.s`)

**Correct**: The 1802 code measures the RC delay via EF4 polling and
computes the ball position itself, writing the pixel byte directly into
video RAM.

**Wrong**: Targets video RAM at 0x2000, which is a fictional separate
video buffer. On real hardware, video RAM starts at 0x0000. With the
high byte always set to 0x20, every `LDI 0x20 / PHI R1` pair is
addressing a memory region that doesn't exist on the real machine.

### I/O repo (`src/asm/joystick_lowmem.s`)

**Correct**: Uses the right memory model (address 0x0000, code and
video in the same 256-byte page). Fits under 64 bytes.

**Wrong**: The 1802 code has no idea where the ball goes. After the
polling ladder, all paths branch to `DRAW: OUT 4 / IDL`. The Rust
`draw_ball()` function in `demo.rs` reads the RC board's internal
bucket state and writes the pixel. This is backwards. The whole point
of the RC polling ladder is that the 1802 program discovers the
position through timing; if Rust reads the bucket directly, the
polling is pure theater.

## Correct Architecture

### Memory model

- One 256-byte page at 0x0000.
- No memory above address 0xFF.
- Code and video share this page.
- The host/emulator renders whatever is in bytes 0x00–0xFF as a 64×32
  display.

### Joystick RC circuit (host responsibility)

The host emulates the analog RC timing circuit. This is external
hardware, not part of the 1802:

- `OUT 2`: pulse the X-axis RC circuit. The host begins charging.
- `OUT 3`: pulse the Y-axis RC circuit. The host begins charging.
- `EF4`: true when the currently-pulsed axis has charged past the
  input threshold.

The host maps the potentiometer position (0–255) to a delay expressed
as a number of instruction ticks before EF4 goes true. The mapping
used by both repos is:

```text
bucket = (value × 4) / 256
```

Yielding bucket values 0, 1, 2, or 3.

After the 1802 executes `OUT 2` or `OUT 3`, the host sets a tick
counter to `bucket + 1`. Each instruction execution decrements the
counter by one. EF4 becomes true when the counter reaches zero. This
means:

- Bucket 0: EF4 true on the very next `B4` check (1 tick after pulse).
- Bucket 1: EF4 true after 2 ticks.
- Bucket 2: EF4 true after 3 ticks.
- Bucket 3: EF4 true after 4 ticks (falls through all 3 `B4`
  instructions to the `BR` fallback).

### Program flow (all 1802 code)

```text
1. Clear display (or accept stale pixels from last frame).
2. OUT 3          → pulse Y-axis RC circuit.
3. B4 / B4 / B4 / BR  → unrolled poll selects one of four Y buckets.
4. OUT 2          → pulse X-axis RC circuit from the selected Y label.
5. B4 / B4 / B4 / BR  → unrolled poll selects one of four X buckets.
6. Write pixel byte 0x80 to the computed video RAM address.
7. IDL            → halt; host renders the display.
```

Steps 2–5 are the measurement. Step 6 is the drawing. The 1802 code
must be the one to decide which address to write. The host never
examines the bucket values for drawing purposes.

### Ball position addressing

The 4×4 grid of possible ball positions maps to video RAM addresses
at base 0x0000:

```text
         X=0    X=1    X=2    X=3
         col 0  col 2  col 4  col 6

Y=0  →  0x00   0x02   0x04   0x06     (row  0)
Y=1  →  0x40   0x42   0x44   0x46     (row  8)
Y=2  →  0x80   0x82   0x84   0x86     (row 16)
Y=3  →  0xC0   0xC2   0xC4   0xC6     (row 24)
```

Each address is `row_offset[y] + col_offset[x]` where row offsets are
0x00, 0x40, 0x80, 0xC0 and column offsets are 0x00, 0x02, 0x04, 0x06.
The pixel value 0x80 sets bit 7 (the leftmost pixel in the byte).

### Display clearing

`OUT 1` as a "clear display" command is a legitimate I/O port
operation. The 1802 is explicitly requesting the action, just like it
would trigger a hardware clear on a real board. This is not "cheating."

Alternatively, the 1802 code can clear video RAM itself by writing
zeroes in a loop. This costs more bytes but is self-contained. On the
real hardware, you'd typically either:

- Clear the entire display at program start (loop over bytes above the
  code region).
- Clear only the previous ball position (save the address from the
  last frame, write zero, then write the new position).

For a single-frame demo, `OUT 1` to clear is fine. For a continuous
loop that moves the ball, the program would clear the old pixel and
draw the new one each frame.

### Drawing the pixel

Two approaches, both valid:

**Unrolled (current emulator repo approach):** Each of the 16 (Y, X)
combinations has its own code block that loads the hard-coded address
into a register and writes 0x80. Simple but large (~12 bytes per block,
~192 bytes total for the draw section). With code starting after the
polling ladder, the total program exceeds 128 bytes.

**Computed (what real programs did):** After determining the Y and X
bucket values, use 1802 arithmetic instructions (ADD, shift) to
compute the address: `addr = (Y × 0x40) + (X × 2)`. Then write 0x80
to that address. This is compact (~20–30 bytes) but requires ADD and
shift opcodes that the current emulator does not yet implement.

To fit in 128 bytes, the computed approach is necessary. This means
the emulator needs ADD (opcode 0xF4) and at least one shift
instruction. The real 1802 instruction set supports this; the
emulator's demo subset just hasn't added them yet.

## Port Assignments

```text
OUT 1  Clear display RAM above the program image.  (Host action)
OUT 2  Pulse X-axis RC timing circuit.              (Host action)
OUT 3  Pulse Y-axis RC timing circuit.              (Host action)
EF4    RC threshold reached for the pulsed axis.    (Host → CPU)
```

There is no `OUT 4`. The 1802 draws the ball itself.

## What the Host Must Do

1. Load the assembled program into memory at 0x0000.
2. On `OUT 1`, zero all memory bytes above the program image (preserve
   code, clear video area).
3. On `OUT 2`, start the X-axis RC timer.
4. On `OUT 3`, start the Y-axis RC timer.
5. After each instruction, decrement the RC tick counter. When it
   reaches zero, set EF4 true.
6. Before each instruction, sync EF4 into the CPU state.
7. Render the 256-byte memory as a 64×32 display (MSB-first, 8 bytes
   per row, 32 rows).

## Live Display Behavior

### Video updates on every memory write

The 1861 scans the full 256-byte memory every display frame. Any byte
that changes — from the ball pixel write, from OUT 1 clearing, from
the program modifying its own data — is visible on the next scan.

The emulator must update the video display after every memory write,
not just after the program halts. This means:

- `STR Rn`: the pixel corresponding to the written address updates
  immediately in the display.
- `OUT 1`: the cleared region updates immediately.
- If the ball write overwrites a code byte, the display shows the new
  pixel value at that position right away. The program may crash on the
  next instruction fetch, and that crash is also visible (the display
  freezes or shows garbage).

### Stepped execution with live register display (web UI)

The web UI must not run the program to completion and then show the
result. It must step through instructions one at a time at a controlled
speed, showing the CPU state after each step:

- **Register values**: D (accumulator), P (program counter register),
  X (index register), Q (output bit), EF4 (input flag), and the
  general-purpose registers R0–RF. These update after every
  instruction.
- **Current instruction**: highlight the instruction about to execute
  (or just executed) in the assembly listing.
- **Memory display**: the 64×32 pixel grid updates after each memory
  write, so the ball pixel appears when the STR instruction executes.
- **Step speed**: a controllable timer (e.g., 100ms–1000ms per step)
  so the user can watch the program execute. A "run" mode steps
  continuously at the selected speed. A "step" mode advances one
  instruction per button press.

This makes the RC polling sequence visible: the user sees EF4 change
from false to true as the tick counter decrements, and sees which B4
instruction branches. Without stepped execution, the entire measurement
happens in one opaque burst and the educational value is lost.

## What the Host Must Not Do

- The host must NOT read the joystick bucket values to decide where to
  draw the ball. That is the 1802 program's job.
- The host must NOT provide a separate video buffer at 0x2000 or any
  other address outside 0x0000–0x00FF.

## Ideal Assembly Program (computed address approach)

This sketch shows the target program structure. It requires ADD and
shift support that the emulator does not yet implement:

```asm
        ORG 0x0000
        OUT 1              ; clear display above program image

        ; Measure Y bucket (0–3) into D as a row byte offset
        OUT 3              ; pulse Y-axis RC
        B4 Y0              ; bucket 0
        B4 Y1              ; bucket 1
        B4 Y2              ; bucket 2
        BR Y3              ; bucket 3
Y0:     LDI 0x00
        BR Y_DONE
Y1:     LDI 0x40
        BR Y_DONE
Y2:     LDI 0x80
        BR Y_DONE
Y3:     LDI 0xc0
Y_DONE: STR R2             ; save Y row offset in scratch byte

        ; Measure X bucket (0–3) into D as a byte offset
        OUT 2              ; pulse X-axis RC
        B4 X0
        B4 X1
        B4 X2
        BR X3
X0:     LDI 0x00
        BR X_DONE
X1:     LDI 0x02
        BR X_DONE
X2:     LDI 0x04
        BR X_DONE
X3:     LDI 0x06

        ; Compute video address: Y row offset + X byte offset.
X_DONE: SEX R2             ; X selects scratch byte
        ADD                ; D = D + M(R2)
        PLO R1             ; R1 now points into 0x0000..0x00ff

        ; Draw the pixel
        LDI 0x80
        STR R1             ; write pixel to video RAM
        IDL                ; halt, host renders
```

The exact arithmetic sequence depends on which shift/add instructions
are available. The key point is that the 1802 code determines the
address from the buckets it measured, not from host-side inspection.

## Summary of Changes Needed

To make the emulator repo's demo correct:

1. **Move video base to 0x0000.** `VIDEO_BASE` should be 0x0000, not
   0x2000. Remove the separate video page concept.

2. **Shrink program to 128 bytes.** Either implement ADD/shift opcodes
   for computed addressing, or find a compact unrolled encoding. The
   polling ladder (~25 bytes) plus a computed draw section (~30 bytes)
   plus clear overhead fits comfortably in 128 bytes.

3. **Keep OUT 1 for clearing.** It is a legitimate I/O command. The
   host zeros memory above the loaded program image.

4. **Remove OUT 4 / draw_ball from the I/O repo.** The host should
   never compute the ball position. The 1802 program must write the
   pixel itself.

5. **Share one .s file between repos.** Both repos should use the
   same assembly source. The emulator repo's CLI and the I/O repo's
   web frontend are different views of the same program.

6. **Video updates on every memory write.** The display must reflect
   memory changes as they happen, not just after the program halts.
   The `STR` instruction that writes the ball pixel should make the
   pixel appear immediately in the rendered display.

7. **Stepped execution in the web UI.** The web frontend must show
   live register values (D, P, X, Q, EF4, R0–RF) and update them
   after each instruction, stepping at a controlled speed. This makes
   the RC polling sequence visible and educational. The user should be
   able to single-step or run at adjustable speed.

8. **Allow self-modifying crashes.** If the ball pixel write
   overwrites a code byte, the program must be allowed to crash or
   misbehave. Do not protect code memory from writes.
