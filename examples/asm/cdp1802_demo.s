        ; Store three bytes at RAM 0x2000..0x2002, then halt.
        ORG 0x0000
        ; R1 points at the output buffer.
        LDI 0x20
        PHI R1
        LDI 0x00
        PLO R1
        ; Write 0x42, 0x43, 0x44 through R1.
        LDI 0x42
        STR R1
        INC R1
        LDI 0x43
        STR R1
        INC R1
        LDI 0x44
        STR R1
        ; Stop through an explicit branch target.
        BR DONE
DONE:   IDL
