# CDP1802 Full ISA Coverage

Status: planning and blocker record for saga `full-cdp1802-isa`, step
`audit-full-isa-coverage`.

## Position

The CDP1802 emulator and assembler must support the full RCA CDP1802
instruction set. Demos must not grow by adding one-off instruction shortcuts in
`sw-cdp1802-io` or any other demo layer. The web demos can expose specific I/O
boards and memory maps, but instruction decode, encode, assembly, disassembly,
and execution semantics belong in the shared crates:

- `sw-cdp1802-isa`: architectural instruction definitions, decode, encode, and
  disassembly text.
- `sw-cdp1802-asm`: source parsing, symbols, directives, listing/output formats,
  and assembly through `sw-cdp1802-isa`.
- `sw-cdp1802-emulator`: CPU state, memory, instruction execution, DMA-visible
  state, interrupt-visible state, and device hooks.
- `sw-cdp1802-io`: browser demos only. This crate should use the shared
  assembler and emulator; it should not maintain a separate instruction
  executor.

The planned 4K cassette-loader demo is blocked until the shared crates can
assemble and execute the required real 1802 program without demo-local opcode
patches.

## Baseline Sources

Coverage should be checked against the RCA CDP1802 instruction summary:

- RCA CDP1802/CDP1802C data sheet, instruction summary table:
  https://www.cosmacelf.com/publications/data-sheets/cdp1802-rca.pdf
- RCA User Manual for the CDP1802 COSMAC Microprocessor, Appendix A:
  https://bitsavers.trailing-edge.com/components/rca/cosmac/MPM-201A_User_Manual_for_the_CDP1802_COSMAC_Microprocessor_1976.pdf
- Lowell O. Turner's opcode table is useful as a compact secondary checklist:
  https://www.nyx.net/~lturner/public_html/CDP1802ins2.html

## Current State

The current implementation is still a demo subset:

- `sw-cdp1802-isa/src/lib.rs` says "Demo-subset ISA implementation" and exposes
  only 18 instruction variants.
- `sw-cdp1802-isa/src/decode.rs` and `src/encode.rs` decode/encode only the
  subset needed by the current demos.
- `sw-cdp1802-asm/src/encode.rs` and `src/symtab.rs` recognize only that same
  subset plus `ORG` and `DB`.
- `sw-cdp1802-emulator/src/exec.rs` says "Instruction execution dispatch for
  the demo subset" and executes only the instructions present in
  `sw_cdp1802_isa::Instruction`.
- `sw-cdp1802-io/src/demo.rs` currently has a separate web-demo stepper that
  repeats part of the emulator semantics. That separation must be removed after
  the full emulator path can step browser demos one instruction at a time.

## Opcode-Family Checklist

The following families must be represented in the shared ISA crate, accepted by
the assembler, executed by the emulator, and covered by tests. Mnemonic aliases
should be accepted where historically common, while disassembly should choose a
stable canonical spelling.

| Opcode range | Family | Current status |
| --- | --- | --- |
| `00` | `IDL` | Implemented. |
| `0N` | `LDN Rn` for `R1..RF`; `00` remains `IDL` | Missing. |
| `1N` | `INC Rn` | Implemented. |
| `2N` | `DEC Rn` | Missing. |
| `30..3F` | Short branch/skip: `BR`, `BQ`, `BZ`, `BDF`, `B1..B4`, `SKP`, `BNQ`, `BNZ`, `BNF`, `BN1..BN4` | Only `BR`, `B1..B4`, and `BN1..BN4` implemented. |
| `4N` | `LDA Rn` | Missing. |
| `5N` | `STR Rn` | Implemented. |
| `60` | `IRX` | Missing. |
| `61..67` | `OUT 1..7` | Implemented. |
| `68` | architecturally unused / no operation slot | Must be decoded deliberately, not accidentally. |
| `69..6F` | `INP 1..7` | Implemented. |
| `70..7F` | `RET`, `DIS`, `LDXA`, `STXD`, `ADC`, `SDB`, `SHRC`, `SMB`, `SAV`, `MARK`, `REQ`, `SEQ`, `ADCI`, `SDBI`, `SHLC`, `SMBI` | Only `REQ` and `SEQ` implemented. |
| `8N` | `GLO Rn` | Implemented. |
| `9N` | `GHI Rn` | Missing. |
| `AN` | `PLO Rn` | Implemented. |
| `BN` | `PHI Rn` | Implemented. |
| `C0..CF` | Long branch / long skip group: `LBR`, `LBQ`, `LBZ`, `LBDF`, `NOP`, `LSNQ`, `LSNZ`, `LSNF`, `LSKP`, `LBNQ`, `LBNZ`, `LBNF`, `LSIE`, `LSQ`, `LSZ`, `LSDF` | Missing. |
| `DN` | `SEP Rn` | Missing. |
| `EN` | `SEX Rn` | Implemented. |
| `F0..FF` | ALU/immediate group: `LDX`, `OR`, `AND`, `XOR`, `ADD`, `SD`, `SHR`, `SM`, `LDI`, `ORI`, `ANI`, `XRI`, `ADI`, `SDI`, `SHL`, `SMI` | Only `ADD`, `LDI`, `ADI`, and `SHL` implemented. |

Important aliases and naming decisions:

- `BDF` is also commonly described as branch on data flag; some references use
  sign-oriented aliases such as `BPZ` for related conditions. Pick one
  canonical spelling and document accepted aliases.
- `BNF` is the negated data-flag branch.
- `SHRC`/`RSHR` and `SHLC`/`RSHL` appear as aliases in 1802 material. Accept the
  common aliases; emit one canonical disassembly spelling.
- `NOP` in the `C` group and the unused `0x68` slot need explicit behavior in
  the ISA and emulator. They should not be lumped into generic invalid opcode
  handling without a deliberate decision.

## Per-Repo Work

### `sw-cdp1802-isa`

Required:

- Replace the demo-subset `Instruction` enum with variants for the full opcode
  map.
- Decode all 256 opcode values, including immediate-width and long-branch-width
  instructions.
- Encode every representable instruction.
- Disassemble every instruction with stable canonical mnemonics.
- Add exhaustive decode coverage tests over `0x00..=0xff`.
- Add encode/decode round-trip tests for every instruction family, including all
  sixteen register encodings where applicable.
- Decide and test how invalid or architecturally unused byte patterns are
  represented, especially `0x68`.

### `sw-cdp1802-asm`

Required:

- Accept every canonical 1802 mnemonic.
- Accept documented aliases where useful for historical source compatibility.
- Support all register-family operands across `R0..RF`.
- Support short branch page-local targets and long branch absolute targets with
  clear range errors.
- Keep `ORG` and `DB`, then add output/listing formats needed by demos and
  cassette workflows.
- Add tests that assemble one source covering every instruction family.
- Add tests that assemble, disassemble/list, and reassemble representative
  sources.

### `sw-cdp1802-emulator`

Required:

- Execute every instruction exposed by `sw-cdp1802-isa`.
- Model `P`, `X`, `T`, `D`, `DF`, `Q`, `IE`, external flags, and all sixteen
  16-bit registers consistently.
- Implement correct memory side effects for `LDN`, `LDA`, `LDX`, `LDXA`,
  `STR`, `STXD`, `OUT`, `INP`, `SAV`, and `MARK`.
- Implement ALU and `DF` semantics for add, subtract, shifts, logic, and
  immediate variants.
- Implement short and long branch/skip semantics, including page-local short
  branch behavior.
- Implement `RET`/`DIS` interrupt-enable state changes sufficiently for monitor,
  cassette, and video DMA demos.
- Add conformance tests by instruction family and integration programs that use
  real assembled source.

Cycle-exact timing is not required for the first full-ISA pass, but instruction
side effects must be architecturally correct.

### `sw-cdp1802-io`

Required after shared support lands:

- Remove the duplicate web-demo instruction executor in `src/demo.rs`.
- Drive browser demos through shared emulator single-step APIs.
- Keep the browser yield model: one or a small bounded number of CPU
  instructions per timer/request-animation-frame callback, never a tight loop on
  the browser thread.
- Add memory-map configuration for base 256-byte ELF-II mode and expanded 4K
  mode.
- Add video-base configuration so 4K demos can point the 256-byte video page at
  `0x0100` while code remains at `0x0000`.

## Cassette Loader Demo Blocker

The proposed 4K cassette-loader logo demo is valid and should be built after
full shared ISA support.

Target demo behavior:

- 4K RAM at `0x0000..0x0fff`.
- Toggled-in loader assembled at `0x0000`.
- Video page at `0x0100..0x01ff`.
- Rust emulates a cassette byte stream and status timing.
- 1802 assembler code reads bytes from the cassette device and stores them into
  the video page.
- The logo appears gradually as memory fills.
- The loader halts or loops after 256 bytes.

This likely needs at least:

- `INP`, already in the subset but must be part of the full shared ISA.
- `INC`, already in the subset.
- `GLO`, already in the subset.
- `BNZ`, currently missing.
- Either a real 256-byte counter loop or another full-ISA-valid loop strategy.

The web demo must not fake the load by copying bytes directly into video memory.
Rust may emulate the cassette hardware and byte availability; the CPU program
must perform the memory writes.

## Next-Step Sequence

1. Expand `sw-cdp1802-isa` to the full opcode map, with exhaustive decode tests
   and encode/decode round trips.
2. Expand `sw-cdp1802-asm` to assemble every full-ISA instruction and produce
   durable listings/output formats.
3. Expand `sw-cdp1802-emulator` to execute every full-ISA instruction with
   instruction-family tests and assembled integration programs.
4. Refactor `sw-cdp1802-io` to call the shared emulator stepper instead of its
   duplicate demo-local executor.
5. Add 4K memory/video-base configuration in the emulator and web demo.
6. Implement the cassette-loader logo demo using only full shared assembler and
   emulator support.

