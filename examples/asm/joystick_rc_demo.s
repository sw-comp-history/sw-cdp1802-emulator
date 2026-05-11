        ; Pulse Y first. Each B4 checks whether the RC signal is ready.
        ; The first ready poll selects one of four Y buckets.
        ORG 0x0000
        OUT 3
        B4 Y0
        B4 Y1
        B4 Y2
        BR Y3

        ; For the selected Y bucket, pulse X and choose an X bucket.
Y0:     OUT 2
        B4 Y0X0
        B4 Y0X1
        B4 Y0X2
        BR Y0X3
Y1:     OUT 2
        B4 Y1X0
        B4 Y1X1
        B4 Y1X2
        BR Y1X3
Y2:     OUT 2
        B4 Y2X0
        B4 Y2X1
        B4 Y2X2
        BR Y2X3
Y3:     OUT 2
        B4 Y3X0
        B4 Y3X1
        B4 Y3X2
        BR Y3X3

        ; Each bucket writes one ball pixel into the 64x32 video buffer.
        ; Row offsets are 0x00, 0x40, 0x80, 0xc0; columns are 0, 16,
        ; 32, and 48, encoded as byte offsets 0, 2, 4, and 6.
Y0X0:   LDI 0x20
        PHI R1
        LDI 0x00
        PLO R1
        LDI 0x80
        STR R1
        BR DONE
Y0X1:   LDI 0x20
        PHI R1
        LDI 0x02
        PLO R1
        LDI 0x80
        STR R1
        BR DONE
Y0X2:   LDI 0x20
        PHI R1
        LDI 0x04
        PLO R1
        LDI 0x80
        STR R1
        BR DONE
Y0X3:   LDI 0x20
        PHI R1
        LDI 0x06
        PLO R1
        LDI 0x80
        STR R1
        BR DONE

Y1X0:   LDI 0x20
        PHI R1
        LDI 0x40
        PLO R1
        LDI 0x80
        STR R1
        BR DONE
Y1X1:   LDI 0x20
        PHI R1
        LDI 0x42
        PLO R1
        LDI 0x80
        STR R1
        BR DONE
Y1X2:   LDI 0x20
        PHI R1
        LDI 0x44
        PLO R1
        LDI 0x80
        STR R1
        BR DONE
Y1X3:   LDI 0x20
        PHI R1
        LDI 0x46
        PLO R1
        LDI 0x80
        STR R1
        BR DONE

Y2X0:   LDI 0x20
        PHI R1
        LDI 0x80
        PLO R1
        LDI 0x80
        STR R1
        BR DONE
Y2X1:   LDI 0x20
        PHI R1
        LDI 0x82
        PLO R1
        LDI 0x80
        STR R1
        BR DONE
Y2X2:   LDI 0x20
        PHI R1
        LDI 0x84
        PLO R1
        LDI 0x80
        STR R1
        BR DONE
Y2X3:   LDI 0x20
        PHI R1
        LDI 0x86
        PLO R1
        LDI 0x80
        STR R1
        BR DONE

Y3X0:   LDI 0x20
        PHI R1
        LDI 0xc0
        PLO R1
        LDI 0x80
        STR R1
        BR DONE
Y3X1:   LDI 0x20
        PHI R1
        LDI 0xc2
        PLO R1
        LDI 0x80
        STR R1
        BR DONE
Y3X2:   LDI 0x20
        PHI R1
        LDI 0xc4
        PLO R1
        LDI 0x80
        STR R1
        BR DONE
Y3X3:   LDI 0x20
        PHI R1
        LDI 0xc6
        PLO R1
        LDI 0x80
        STR R1

DONE:   IDL
