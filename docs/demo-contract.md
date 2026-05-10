# CDP1802 Assembler/Emulator Demo Contract

Status: saga step 1 planning record. This file defines the first
working demonstration target for `sw-cdp1802-asm` plus
`sw-cdp1802-emulator`; it intentionally does not implement any
semantics.

## Objective

The first demo proves the minimum useful path:

```text
CDP1802 assembly source -> assembled bytes -> emulator memory -> run -> final RAM/register state
```

The demo program writes three known bytes to RAM and halts. It should
be small enough that every expected byte and every final assertion is
obvious in a review.

## Current State

This repo and the sibling `sw-cdp1802-isa` and `sw-cdp1802-asm` repos
are still generated skeletons. The relevant source files currently
contain only module comments:

- `sw-cdp1802-isa/src/{opcode,decode,encode,register}.rs`
- `sw-cdp1802-asm/src/{parser,encode,symtab}.rs`
- `sw-cdp1802-emulator/src/{state,memory,exec}.rs`

That means the next implementation steps must first establish a small
shared instruction representation in `sw-cdp1802-isa`, then make the
assembler and emulator consume the same subset.

## Demo Program

The demo initializes `R1` to data address `0x2000`, stores bytes
`0x42`, `0x43`, and `0x44` through `R1`, increments `R1` between
stores, branches to the halt label, and stops via `IDL`.

Assembly source target:

```asm
        ORG 0x0000
        LDI 0x20
        PHI R1
        LDI 0x00
        PLO R1
        LDI 0x42
        STR R1
        INC R1
        LDI 0x43
        STR R1
        INC R1
        LDI 0x44
        STR R1
        BR DONE
DONE:   IDL
```

Expected bytes, once the assembler subset exists:

```text
f8 20 b1 f8 00 a1 f8 42 51 11 f8 43 51 11 f8 44
51 30 13 00
```

The branch target byte is `0x13` because `DONE` is at byte address
`0x0013` in the same 256-byte page.

## Instruction Subset

The first executable subset is deliberately narrow:

| Mnemonic | Bytes | Demo role |
|---|---:|---|
| `IDL` | `00` | Stop convention for the emulator. |
| `INC Rn` | `1n` | Advance the byte pointer in `R1`. |
| `BR addr` | `30 aa` | Branch within the current 256-byte page. |
| `STR Rn` | `5n` | Store accumulator `D` at memory addressed by `Rn`. |
| `PLO Rn` | `an` | Load low byte of `Rn` from `D`. |
| `PHI Rn` | `bn` | Load high byte of `Rn` from `D`. |
| `LDI imm8` | `f8 kk` | Load an immediate byte into `D`. |

The next step should verify these opcode encodings against the RCA
CDP1802 reference before landing code. This contract fixes the demo
shape, not the authoritative manual citation.

## Machine Model

Minimum emulator state for the demo:

- Sixteen 16-bit registers `R0` through `R15`.
- 8-bit accumulator `D`.
- 4-bit program-counter selector `P`.
- 4-bit data-register selector `X`, present even if unused by this demo.
- Byte-addressed RAM, initially zeroed.
- `halted` flag set by `IDL`.

Reset/entry convention for this saga:

- Memory is 64 KiB unless a test explicitly chooses a smaller size that
  still covers `0x2000..0x2002`.
- Program bytes load at address `0x0000`.
- Initial state uses `P = 0`, `X = 0`, and `R0 = 0x0000`.
- Execution fetches from `R[P]` and advances that register as the
  program counter.

## Success Criteria

The integration test for this demo should:

1. Assemble the source above through `sw-cdp1802-asm`.
2. Assert the exact byte sequence listed above.
3. Load the bytes at RAM address `0x0000`.
4. Run the emulator with a bounded step limit.
5. Assert `halted == true`.
6. Assert RAM at `0x2000..0x2002` is `[0x42, 0x43, 0x44]`.
7. Assert `R1 == 0x2002` after the final store.

The demo runner step should mirror the IBM 1130 example pattern:
print source, assembled bytes, step count, final registers, and the
relevant RAM range.

## Assembler Contract

The first assembler subset should support:

- `;` line comments and blank lines.
- Labels with a trailing colon, matching the IBM 1130 assembler
  convention from `sw-ibm1130-asm`.
- `ORG` for the initial location counter.
- Hex literals with `0x` and plain decimal literals.
- Register operands spelled `R0` through `R15`.
- Exact byte output; no branch relaxation or auto-sizing.

Disassembly and richer directives are out of scope for the first demo.

## References And Lessons

Use `~/github/sw-vibe-coding/gen-isa` as the design hub:

- `README.md` explains the generator-only repo and sibling crate map.
- `docs/porting-guide.md` defines the per-ISA crate shape and says
  emulator semantics stay hand-written.
- `docs/spec-format.md` describes generated ISA fields but leaves
  codegen, ABI, and emulator semantics out of scope.
- `docs/postmortem-1130-bringup.md` is the worked bring-up record.

Use the IBM 1130 repos as process examples:

- `sw-ibm1130-asm/tests/asm.rs` for exact-byte assembler tests and
  round-trip discipline.
- `sw-ibm1130-emulator/tests/run_demos.rs` for assemble-load-run
  integration tests.
- `sw-ibm1130-emulator/examples/_common.rs` for runnable demo output.

CDP1802-specific follow-up work should remain in the CDP1802 sibling
repos unless the step prompt explicitly says otherwise.
