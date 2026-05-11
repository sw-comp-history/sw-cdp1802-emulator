        ; Simulate an ELF-style front panel and video RAM write.
        ORG 0x0000
        ; Turn on Q; the board mirrors this to q_led.
        SEQ
        ; R1 points at the RAM-backed 64x32 video buffer.
        LDI 0x20
        PHI R1
        LDI 0x00
        PLO R1
        ; Store and display the first visible video byte.
        LDI 0xaa
        STR R1
        OUT 1
        ; OUT increments R1, so the second byte lands at 0x2001.
        LDI 0x55
        STR R1
        OUT 1
        ; EF4 is the input strobe; skip REQ while it is pressed.
        B4 INPUT
        REQ
        ; Read the input latch into D and M(R1), then halt.
INPUT:  INP 1
        IDL
