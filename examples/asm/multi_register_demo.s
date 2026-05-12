        ; Exercise multiple CDP1802 scratchpad registers with the
        ; current assembler/emulator subset, including RA and RF aliases.
        ORG 0x0000
        ; R2 writes 0x52 to 0x2010.
START:  LDI 0x20
        PHI R2
        LDI 0x10
        PLO R2
        LDI 0x52
        STR R2
        INC R2
        ; R3 writes 0x53 to 0x2020.
        LDI 0x20
        PHI R3
        LDI 0x20
        PLO R3
        LDI 0x53
        STR R3
        INC R3
        ; RA writes 0x5a to 0x2030.
        LDI 0x20
        PHI RA
        LDI 0x30
        PLO RA
        LDI 0x5a
        STR RA
        INC RA
        ; RF writes 0x5f to 0x2040.
        LDI 0x20
        PHI RF
        LDI 0x40
        PLO RF
        LDI 0x5f
        STR RF
        INC RF
DONE:   IDL
