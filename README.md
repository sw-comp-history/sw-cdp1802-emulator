# sw-cdp1802-emulator

RCA CDP1802 Emulator: instruction execution semantics.

## Status

`0.1.0` demo subset. The emulator can execute the first assembler/
emulator contract program for `IDL`, `INC`, `BR`, `STR`, `PLO`, `PHI`,
and `LDI`.

Run the demo:

```bash
cargo run --example cdp1802_demo
```

## Sibling layout

Cross-crate deps assume sibling clones at
`~/github/sw-langtools/<framework-crate>` and
`~/github/<host-org>/sw-cdp1802-<role>`. See
[`gen-isa/docs/decisions.md`](https://github.com/sw-vibe-coding/gen-isa/blob/main/docs/decisions.md)
Sec 1 for the full org map.

## License

MIT.
